use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use home::home_dir;

use anyhow::Context;

use crate::identifier::IdentifierStruct;
use crate::yaml_conf_load::yaml_conf_load_with;
use crate::jvalue;
use crate::brokers::{BrokerProxy, Broker,  broker_factories_wrapper::BrokerFactoriesWrapper};
use crate::object::{ObjectCore, JuizObjectCoreHolder, JuizObjectClass};

use crate::value::{obj_get_str, obj_merge};
use crate::{ContainerPtr, CoreBroker, Identifier, JuizError, JuizObject, JuizResult, ProcessPtr, Value};
use crate::utils::{get_array, juiz_lock};
use crate::utils::manifest_util::{construct_id, id_from_manifest, manifest_merge, when_contains_do_mut};
use super::system_builder;
use crate::utils::when_contains_do;

use std::time;

type SpinCallbackFunctionType = dyn Fn() -> JuizResult<()>;

#[allow(dead_code)]
pub struct System {
    core: ObjectCore,
    manifest: Value,
    core_broker: Arc<Mutex<CoreBroker>>,
    sleep_time: Duration,
    broker_factories: HashMap<String, Arc<Mutex<BrokerFactoriesWrapper>>>,
    brokers: HashMap<String, Arc<Mutex<dyn Broker>>>,
    broker_proxies: HashMap<String, Arc<Mutex<dyn BrokerProxy>>>,
    pub tokio_runtime: tokio::runtime::Runtime,
    spin_callback: Option<Box<SpinCallbackFunctionType>>,
    working_dir: Option<PathBuf>,
}

fn check_system_manifest(manifest: Value) -> JuizResult<Value> {
    if !manifest.is_object() {
        return Err(anyhow::Error::from(JuizError::ValueIsNotObjectError{value:manifest}).context("check_system_manifest failed."));
    }
    return Ok(manifest);
}

impl JuizObjectCoreHolder for System {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}

impl JuizObject for System {
    fn profile_full(&self) -> JuizResult<Value> {
        let bf: Value = juiz_lock(self.core_broker())?.profile_full()?.try_into()?;
        let p = self.core.profile_full()?;
        Ok(obj_merge(p, &jvalue!({
            "core_broker": bf,
        }))?.into())
    }
}


impl System {

    pub fn new(manifest: Value) -> JuizResult<System> {
        env_logger::init();
        let checked_manifest = check_system_manifest(manifest)?;
        let updated_manifest:Value = merge_home_manifest(checked_manifest)?;
        Ok(System{
            core: ObjectCore::create(JuizObjectClass::System("System"), "system", "system"),
            manifest: updated_manifest.clone(),
            core_broker: Arc::new(Mutex::new(CoreBroker::new(jvalue!({"type_name": "CoreBroker", "name": "core_broker"}))?)),
            sleep_time: time::Duration::from_millis(100),
            broker_factories: HashMap::new(),
            brokers: HashMap::new(),
            broker_proxies: HashMap::new(),
            tokio_runtime: tokio::runtime::Builder::new_multi_thread().thread_name("juiz_core::System").worker_threads(4).enable_all().build().unwrap(),
            spin_callback: None,
            working_dir: None,
        })
    }

    pub fn set_spin_callback(&mut self, cb: Box<SpinCallbackFunctionType>) -> () {
        self.spin_callback = Some(cb);
    }

    pub fn set_spin_sleeptime(&mut self, duration: Duration) -> () {
        self.sleep_time = duration;
    }

    pub(crate) fn core_broker(&self) -> &Arc<Mutex<CoreBroker>> {
        &self.core_broker
    }

    pub fn set_working_dir(mut self, path: &Path) -> Self {
        self.working_dir = Some(path.into());
        self
    }

    pub fn get_working_dir(&self) -> Option<PathBuf> {
        self.working_dir.clone()
    }

    ///
    pub fn process_from_id(&self, id: &Identifier) -> JuizResult<ProcessPtr> {
        log::trace!("System::process_from_id(id={id:}) called");
        let s = IdentifierStruct::from(id.clone());
        if s.broker_type_name == "core" {
            return juiz_lock(&self.core_broker)?.store().processes.get(id);
        }
        self.process_proxy(id)
    }

    pub fn process_from_typename_and_name(&self, type_name: &str, name: &str) -> JuizResult<ProcessPtr> {
        let id = construct_id("Process", type_name, name, "core", "core");
        juiz_lock(&self.core_broker)?.store().processes.get(&id)
    }
    
    
    pub fn container_from_id(&self, id: &Identifier) -> JuizResult<ContainerPtr> {
        log::trace!("System::container_from_id({id}) called");
        let s = IdentifierStruct::from(id.clone());
        if s.broker_type_name == "core" {
            return juiz_lock(&self.core_broker)?.store().containers.get(id);
        }
        self.container_proxy(id)
    }

    pub fn container_from_typename_and_name(&self, type_name: &str, name: &str) -> JuizResult<ContainerPtr> {
        let id = construct_id("Container", type_name, name, "core", "core");
        juiz_lock(&self.core_broker)?.store().containers.get(&id)
    }

    pub fn container_process_from_id(&self, id: &Identifier) -> JuizResult<ProcessPtr> {
        log::trace!("System::container_process_from_id(id={id:}) called");
        let cp = juiz_lock(&self.core_broker)?.store().container_processes.get(id)?;
        log::trace!("cps  OK");
        return Ok(cp);
    }

    pub fn any_process_from_id(&self, id: &Identifier) -> JuizResult<ProcessPtr> {
        log::trace!("System::any_process_from_id(id={id:}) called");
        let result = self.process_from_id(id);
        if result.is_ok() {
            return result;
        }
        log::trace!("System::any_process_from_id(id={id:}) failed. No process is found. Now searching container_process...");
        self.container_process_from_id(id)
    }

    pub fn container_process_from_typename_and_name(&self, type_name: &str, name: &str) -> JuizResult<ProcessPtr> {
        let id = construct_id("ContainerProcess", type_name, name, "core", "core");
        juiz_lock(&self.core_broker)?.store().container_processes.get(&id)
    }

    pub fn any_process_from_typename_and_name(&self, type_name: &str, name: &str) -> JuizResult<ProcessPtr> {
        let result = self.process_from_typename_and_name(type_name, name);
        if result.is_ok() {
            return result;
        }
        self.container_process_from_typename_and_name(type_name, name)
    }

    pub fn process_from_manifest(&self, manifest: &Value) -> JuizResult<ProcessPtr> {
        let id = id_from_manifest(manifest)?.to_string();
        self.process_from_id(&id)
    }

    pub fn container_from_manifest(&self, manifest: &Value) -> JuizResult<ContainerPtr> {
        let id = id_from_manifest(manifest)?.to_string();
        self.container_from_id(&id)
    }

    pub fn container_process_from_manifest(&self, manifest: &Value) -> JuizResult<ProcessPtr> {
        let id = id_from_manifest(manifest)?.to_string();
        self.container_process_from_id(&id)
    }

    pub fn any_process_from_manifest(&self, manifest: &Value) -> JuizResult<ProcessPtr> {
        
        let id_result = id_from_manifest(manifest);
        if id_result.is_ok() {
            return self.any_process_from_id(&id_result.unwrap().to_string());
        }

        let type_name = obj_get_str(manifest, "type_name")?;
        let name = obj_get_str(manifest, "name")?;
        self.any_process_from_typename_and_name(type_name, name)
    }

    pub fn container_proxy(&self, id: &Identifier) -> JuizResult<ContainerPtr> {
        juiz_lock(&self.core_broker)?.container_proxy_from_identifier(id)
    }


    pub fn process_proxy(&self, id: &Identifier) -> JuizResult<ProcessPtr> {
        juiz_lock(&self.core_broker)?.process_proxy_from_identifier(id)
    }

    pub fn container_process_proxy(&self, id: &Identifier) -> JuizResult<ProcessPtr> {
        juiz_lock(&self.core_broker)?.container_process_proxy_from_identifier(id)
    }

    fn setup(&mut self) -> JuizResult<()> {
        log::trace!("System::setup() called");
        let manifest_copied = self.manifest.clone();
        let _ = when_contains_do_mut(&manifest_copied, "plugins", |v| {
            system_builder::setup_plugins(self, v).context("system_builder::setup_plugins in System::setup() failed")
        })?;

        let _ = when_contains_do(&self.manifest, "processes", |v| {
            system_builder::setup_processes(self, v).context("system_builder::setup_processes in System::setup() failed")
        })?;

        let _ = when_contains_do(&self.manifest, "containers", |v| {
            system_builder::setup_containers(self, v).context("system_builder::setup_containers in System::setup() failed")
        })?;


        system_builder::setup_local_broker_factory(self).context("system_builder::setup_local_broker_factory in System::setup() failed.")?;
        system_builder::setup_local_broker(self).context("system_builder::setup_local_broker in System::setup() failed.")?;

        system_builder::setup_ipc_broker_factory(self).context("system_builder::setup_ipc_broker_factory in System::setup() failed.")?;
        //system_builder::setup_ipc_broker(self).context("system_builder::setup_ipc_broker in System::setup() failed.")?;
        
        let _ = when_contains_do_mut(&manifest_copied, "brokers", |v| {
            system_builder::setup_brokers(self, v).context("system_builder::setup_brokers in System::setup() failed.")
        })?;

        for (type_name, broker) in self.brokers.iter() {
            log::info!("starting Broker({type_name:})");
            let _ = juiz_lock(&broker)?.start().context("Broker(type_name={type_name:}).start() in System::setup() failed.")?;
        }

        let _ = when_contains_do_mut(&manifest_copied, "broker_proxies", |v| {
            system_builder::setup_broker_proxies(self, v).context("system_builder::setup_broker_proxies in System::setup() failed.")
        })?;

        let _ = when_contains_do(&manifest_copied, "ecs", |v| {
            system_builder::setup_ecs(self, v).context("system_builder::setup_ecs in System::setup() failed")
        })?;

        let _ =  when_contains_do(&self.manifest, "connections", |v| {
            system_builder::setup_connections(self, v).context("system_builder::setup_connections in System::setup() failed.")
        })?;


        log::debug!("System::setup() successfully finished.");
        Ok(())
    }

    fn cleanup(&mut self) -> JuizResult<()> {
        log::trace!("System::cleanup() called");
        system_builder::cleanup_ecs(self).context("system_builder::cleanup_ecs in System::cleanup() failed")?;
        system_builder::cleanup_brokers(self).context("system_builder::cleanup_ecs in System::cleanup() failed")?;
        Ok(())
    }

    pub fn cleanup_brokers(&mut self) -> JuizResult<()> {
        log::trace!("System::cleanup_brokers() called");
        self.brokers.clear();
        Ok(())
    }

    ///
    /// SIGINTおよびSIGTERMを待つ。待つ間はsleep_time秒だけsleepするたびにself.spin()を呼ぶ。
    /// 
    pub fn wait_for_singal(&mut self) -> JuizResult<()> {
        let term = Arc::new(AtomicBool::new(false));
        let _ = signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&term));
        let _ = signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term));
        while !term.load(Ordering::Relaxed) {
            self.spin();
            std::thread::sleep(self.sleep_time);
        }
        Ok(())
    }

    fn stop(&mut self) -> JuizResult<()> {

        for (type_name, broker) in self.brokers.iter() {
            log::info!("stopping Broker({type_name:})");
            let _ = juiz_lock(&broker)?.stop()?;
        }

        Ok(())
    }

    ///
    /// run中およびrun_and_do中に呼ばれる周期実行関数。
    /// 
    fn spin(&mut self) -> () {
        // log::debug!("System::spin() called");
        if self.spin_callback.is_some() {
            let _ = self.spin_callback.as_ref().unwrap()();
        }
    }

    pub fn run(&mut self) -> JuizResult<()> {
        log::debug!("System::run() called");
        log::debug!("Juiz System Now Started.");
        self.setup().context("System::setup() in System::run() failed.")?;
        self.wait_for_singal().context("System::wait_for_signal() in System::run() failed.")?;
        self.stop()?;
        log::debug!("System::run() exit");
        self.cleanup()?;
        Ok(())
    }

    pub fn run_and_do(&mut self,  func: fn(&mut System) -> JuizResult<()>) -> JuizResult<()> {
        log::debug!("System::run_and_do() called");
        self.setup().context("System::setup() in System::run_and_do() failed.")?;
        log::debug!("Juiz System Now Started.");
        (func)(self).context("User function passed for System::run_and_do() failed.")?;
        self.wait_for_singal().context("System::wait_for_signal() in System::run_and_do() failed.")?;
        log::debug!("System::run_and_do() exit");
        self.stop()?;
        self.cleanup()?;
        Ok(())
    }

    pub fn run_and_do_once<F>(&mut self, func: F) -> JuizResult<()> where F: FnOnce(&mut System) -> JuizResult<()>  {
        log::debug!("System::run_and_do_once() called");
        self.setup().context("System::setup() in System::run_and_do_once() failed.")?;
        log::debug!("Juiz System Now Started.");
        (func)(self).context("User function passed for System::run_and_do_once() failed.")?;
        //self.wait_for_singal().context("System::wait_for_signal() in System::run_and_do() failed.")?;
        self.stop()?;
        log::debug!("System::run_and_do_once() exit");
        self.cleanup()?;
        Ok(())
    }

    pub fn broker_proxy(&self, manifest: &Value) -> JuizResult<Arc<Mutex<dyn BrokerProxy>>> {
        log::trace!("System::broker_proxy({}) called", manifest);
        juiz_lock(&self.core_broker)?.broker_proxy_from_manifest(manifest)
    }

    pub fn register_broker_factories_wrapper(&mut self, bf: Arc<Mutex<BrokerFactoriesWrapper>>) -> JuizResult<Arc<Mutex<BrokerFactoriesWrapper>>> {
        let type_name = juiz_lock(&bf)?.type_name().to_string();
        log::trace!("System::register_broker_factories_wrapper(BrokerFactory(type_name={:?})) called", type_name);
        if self.broker_factories.contains_key(&type_name) {
            log::error!("system does not contains broker factory with type_name='{type_name:}'.");
            return Err(anyhow::Error::from(JuizError::BrokerFactoryOfSameTypeNameAlreadyExistsError{type_name: type_name}));
        }
        self.broker_factories.insert(type_name.clone(), Arc::clone(&bf));
        juiz_lock(&self.core_broker())?.store_mut().register_broker_factory_manifest(type_name.as_str(), juiz_lock(&bf)?.profile_full()?.try_into()?)?;
        juiz_lock(&self.core_broker())?.store_mut().broker_proxies.register_factory(juiz_lock(&bf)?.broker_proxy_factory.clone())?;
        log::trace!("System::register_broker_factories_wrapper(BrokerFactory(type_name={:?})) exit", type_name);
        Ok(bf)
    }

    fn broker_factories_wrapper(&self, type_name: &str) -> JuizResult<&Arc<Mutex<BrokerFactoriesWrapper>>> {
        match self.broker_factories.get(type_name) {
            None => Err(anyhow::Error::from(JuizError::BrokerFactoryCanNotFoundError{type_name: type_name.to_string()})),
            Some(bf) => Ok(bf)
        }
    }

    pub fn create_broker(&mut self, manifest: &Value) -> JuizResult<Arc<Mutex<dyn Broker>>> {
        log::trace!("System::create_broker({manifest:}) called");
        let type_name = obj_get_str(manifest, "type_name")?;
        let bf = self.broker_factories_wrapper(type_name)?;
        let b = juiz_lock(bf).context("Locking BrokerFactoriesWrapper in System::create_broker() failed.")?.create_broker(&manifest).context("BrokerFactoriesWrapper.create_broker() failed in System::create_broker()")?;
        self.register_broker(b)
    }

    pub(crate) fn register_broker(&mut self, broker: Arc<Mutex<dyn Broker>>) -> JuizResult<Arc<Mutex<dyn Broker>>> {
        let type_name;
        
        {
            type_name = juiz_lock(&broker).context("Locking broker to get type_name failed.")?.type_name().to_string();
        }
        log::trace!("System::register_broker(type_name={type_name:}) called");
        
        self.brokers.insert(type_name.clone(), Arc::clone(&broker));
        let p: Value  =juiz_lock(&broker).context("Locking passed broker failed.")?.profile_full().context("Broker::profile_full")?.try_into()?;
        juiz_lock(&self.core_broker()).context("Blocking CoreBroker failed.")?.store_mut().register_broker_manifest(type_name.as_str(), p)?;
        
        Ok(broker)
    }

    pub fn create_broker_proxy(&mut self, manifest: &Value) -> JuizResult<Arc<Mutex<dyn BrokerProxy>>> {
        log::trace!("System::create_broker_proxy({manifest:}) called");
        let type_name = obj_get_str(manifest, "type_name")?;
        let bf = self.broker_factories_wrapper(type_name)?;
        let b = juiz_lock(bf).context("Locking BrokerFactoriesWrapper in System::create_broker_proxy() failed.")?.create_broker_proxy(&manifest).context("BrokerFactoriesWrapper.create_broker_proxy() failed in System::create_broker()")?;
        self.register_broker_proxy(b)
    }
    
    pub(crate) fn register_broker_proxy(&mut self, broker_proxy: Arc<Mutex<dyn BrokerProxy>>) -> JuizResult<Arc<Mutex<dyn BrokerProxy>>> {
        let type_name;        
        {
            type_name = juiz_lock(&broker_proxy).context("Locking broker to get type_name failed.")?.type_name().to_string();
        }
        log::trace!("System::register_broker(type_name={type_name:}) called");
        juiz_lock(&self.core_broker())?.store_mut().broker_proxies.register(broker_proxy.clone())?;
        //self.broker_proxies.insert(type_name.clone(), Arc::clone(&broker_proxy));
        //let p: Value  =juiz_lock(&broker_proxy).context("Locking passed broker_proxy failed.")?.profile_full().context("BrokerProxy::profile_full")?.try_into()?;
        //juiz_lock(&self.core_broker()).context("Blocking CoreBroker failed.")?.store_mut().register_broker_proxy_manifest(type_name.as_str(), p)?;
        
        Ok(broker_proxy)
    }

    pub fn process_list(&self) -> JuizResult<Vec<Value>> {
        log::trace!("System::process_list() called");
        let mut local_processes = juiz_lock(&self.core_broker())?.store().processes.list_manifests()?;
        for proxy in juiz_lock(&self.core_broker())?.store().broker_proxies.objects().into_iter() {
            log::trace!("process_list for proxy ()");
            for v in get_array(&juiz_lock(proxy)?.process_list()?)?.iter() {
                local_processes.push(v.clone());
            }
        }
        log::debug!("ids: {local_processes:?}");    
        return Ok(local_processes);
    }

    pub fn container_list(&self) -> JuizResult<Vec<Value>> {
        log::trace!("System::container_list() called");
        let mut local_containers = juiz_lock(&self.core_broker())?.store().containers.list_manifests()?;
        for proxy in juiz_lock(&self.core_broker())?.store().broker_proxies.objects().into_iter() {
            for v in get_array(&juiz_lock(proxy)?.container_list()?)?.iter() {
                local_containers.push(v.clone());
            }
        }
        log::debug!("ids: {local_containers:?}");    
        return Ok(local_containers);
    }

    pub fn container_process_list(&self) -> JuizResult<Vec<Value>> {
        log::trace!("System::container_process_list() called");
        let mut local_processes = juiz_lock(&self.core_broker())?.store().container_processes.list_manifests()?;
        for proxy in juiz_lock(&self.core_broker())?.store().broker_proxies.objects().into_iter() {
            for v in get_array(&juiz_lock(proxy)?.container_process_list()?)?.iter() {
                local_processes.push(v.clone());
            }
        }
        log::debug!("ids: {local_processes:?}");    
        return Ok(local_processes);
    }

    pub fn any_process_list(&self) -> JuizResult<Vec<Value>> {
        log::trace!("System::any_process_list() called");
        let mut ps = self.process_list()?;
        let mut cps = self.container_process_list()?;
        cps.append(&mut ps);
        return Ok(cps)
    }
    
}


fn param_map(juiz_homepath: PathBuf) -> HashMap<&'static str, String> {
    HashMap::from([("${HOME}", juiz_homepath.to_str().unwrap().to_owned())])
}

fn merge_home_manifest(manifest: Value) -> JuizResult<Value> {
    log::trace!("merge_home_manifest({manifest}) called");
    match home_dir() {
        Some(homepath) => {
            let juiz_homepath = homepath.join(".juiz");
            let juiz_conf_homepath = juiz_homepath.join("conf");
            let juiz_default_conf_filepath = juiz_conf_homepath.join("default.conf");
            if juiz_default_conf_filepath.exists() {
                let system_manifest = yaml_conf_load_with(juiz_default_conf_filepath.to_str().unwrap().to_owned(), param_map(juiz_homepath))?;
                log::trace!(" - system_manifest: {system_manifest:}");
                let merged_manifest =  manifest_merge(system_manifest, &manifest)?;
                log::trace!(" - merged manifest: {merged_manifest:}");
                return Ok(merged_manifest);
            }
        }
        None => {}
    }

    Ok(manifest)
}