use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{
    atomic::{AtomicBool, Ordering}, 
    Arc, Mutex};
use std::time::{self, Duration};
use home::home_dir;
use juiz_sdk::anyhow::{self, anyhow, Context};
use juiz_sdk::utils::manifest_util::manifest_merge;
use juiz_sdk::utils::yaml_conf_load::yaml_conf_load_with;

use crate::brokers::broker_ptr::BrokerPtr;
use crate::prelude::*;

use crate::{
    brokers::{
        broker_proxy::SystemBrokerProxy,
        broker_factories_wrapper::BrokerFactoriesWrapper},
};

use super::system_builder;
use super::system_store::{SystemStore, SystemStorePtr};

type SpinCallbackFunctionType = dyn Fn() -> JuizResult<()>;

#[allow(dead_code)]
pub struct System {
    core: ObjectCore,
    manifest: Value,
    core_broker: CoreBrokerPtr,
    store: SystemStorePtr,
    sleep_time: Duration,
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
        let bf: Value = self.core_broker().lock()?.profile_full()?.try_into()?;
        let p = self.core.profile_full()?;
        Ok(obj_merge(p, &jvalue!({
            "core_broker": bf,
        }))?.into())
    }
}

impl System {

    pub fn new(manifest: Value) -> JuizResult<System> {
        let checked_manifest = check_system_manifest(manifest)?;
        let updated_manifest:Value = merge_home_manifest(checked_manifest)?;
        let store = SystemStorePtr::new(SystemStore::new());
        Ok(System {
            core: ObjectCore::create(JuizObjectClass::System("System"), "system", "system"),
            manifest: updated_manifest.clone(),
            core_broker: CoreBrokerPtr::new(CoreBroker::new(jvalue!({"type_name": "CoreBroker", "name": "core_broker"}), store.clone())?),
            sleep_time: time::Duration::from_millis(100),
            store,
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

    pub fn core_broker(&self) -> &CoreBrokerPtr {
        &self.core_broker
    }

    pub fn set_working_dir(mut self, path: &Path) -> Self {
        self.working_dir = Some(path.into());
        self
    }

    pub fn get_working_dir(&self) -> Option<PathBuf> {
        self.working_dir.clone()
    }


    // pub fn any_process_from_typename_and_name(&self, type_name: &str, name: &str) -> JuizResult<ProcessPtr> {
    //     let result = self.process_from_typename_and_name(type_name, name);
    //     if result.is_ok() {
    //         return result;
    //     }
    //     self.container_process_from_typename_and_name(type_name, name)
    // }

    // pub fn process_proxy(&self, id: &Identifier) -> JuizResult<ProcessPtr> {
    //     self.core_broker.lock_mut()?.process_proxy_from_identifier(id)
    // }

    // pub fn container_process_proxy(&self, id: &Identifier) -> JuizResult<ProcessPtr> {
    //     self.core_broker.lock_mut()?.container_process_proxy_from_identifier(id)
    // }

    // pub fn ec_proxy(&self, id: &Identifier) -> JuizResult<Arc<Mutex<dyn ExecutionContextFunction>>> {
    //     self.core_broker.lock_mut()?.ec_proxy_from_identifier(id)
    // }

    pub fn setup(mut self) -> JuizResult<Self> {
        log::trace!("System::setup() called");
        let manifest_copied = self.manifest.clone();
        log::debug!(" - manifest: {:}", self.manifest);
        let option = self.get_opt();
        //log::info!("option: {option:}");
        let _ = when_contains_do_mut(&manifest_copied, "plugins", |v| {
            system_builder::setup_plugins(&mut self, v, &option).context("system_builder::setup_plugins in System::setup() failed")
        })?;

        system_builder::setup_objects(&mut self, &manifest_copied)?;

        system_builder::setup_topic_synchronization(&mut self)?;
        log::debug!("System::setup() successfully finished.");
        Ok(self)
    }

    fn cleanup(&mut self) -> JuizResult<()> {
        system_builder::cleanup_objects(self)
    }

    pub fn add_subsystem_by_id(self, id_opt: Option<Identifier>) -> JuizResult<Self> {
        log::trace!("add_subsystem_by_id(id={id_opt:?})");
        if id_opt.is_none() {
            return Ok(self);
        }
        let id = id_opt.unwrap();
        let id_struct = IdentifierStruct::new_broker_id(id.clone())?;
        let profile = id_struct.to_broker_manifest();
        match self.core_broker().lock_mut().unwrap().system_add_subsystem(profile) {
            Ok(_) => {},
            Err(e) => {
                log::error!("Error in add_subsystem_by_id(id={id:?}). Error({e:})");
                return Err(anyhow!(e));
            }
        } 
        Ok(self)
    }

    pub fn start_brokers(&self) -> JuizResult<()> {
        self.store.lock()?.brokers.iter().map(|(type_name, broker)| {
            log::info!("starting Broker({type_name:})");
            broker.lock_mut()?.start()
        }).collect::<JuizResult<Vec<()>>>()?;
        Ok(())
    }

    fn get_opt(&self) -> Value {
        let manif_obj = self.manifest.as_object().unwrap();
        if manif_obj.contains_key("option") {
            manif_obj.get("option").unwrap().clone()
        } else {
            jvalue!({})
        }
    }

    fn get_opt_mut(&mut self) -> &mut Value {
        let manif_obj = self.manifest.as_object_mut().unwrap();
        if manif_obj.contains_key("option") {
            manif_obj.get_mut("option").unwrap()
        } else {
            manif_obj.insert("option".to_owned(), jvalue!({}));
            manif_obj.get_mut("option").unwrap()        
        }
    }

    pub fn start_http_broker(mut self, flag_start: bool) -> Self {
        let opt_value = self.get_opt_mut().as_object_mut().unwrap();
        if opt_value.contains_key("http_broker") {
            opt_value.get_mut("http_broker").unwrap().as_object_mut().unwrap().insert("start".to_owned(), jvalue!(flag_start));
        } else {
            opt_value.insert("http_broker".to_owned(), jvalue!({"start": flag_start}));
        };
        self
    }

    pub fn cleanup_brokers(&mut self) -> JuizResult<()> {
        log::trace!("System::cleanup_brokers() called");
        self.store.lock_mut()?.brokers.clear();
        log::trace!("brokers cleared");
        self.store.lock_mut()?.broker_factories.clear();
        log::trace!("broker factories cleared");
        
        log::trace!("System::cleanup_brokers() exit");
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

        for (type_name, broker) in self.store.lock()?.brokers.iter() {
            log::info!("stopping Broker({type_name:})");
            let _ = broker.lock_mut()?.stop()?;
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
        log::info!("Juiz System({}) Now Started.", self.store.uuid()?);
        // self.setup().context("System::setup() in System::run() failed.")?;
        self.wait_for_singal().context("System::wait_for_signal() in System::run() failed.")?;
        self.stop()?;
        log::debug!("System::run() exit");
        self.cleanup()?;
        Ok(())
    }

    pub fn run_and_do(&mut self,  func: impl FnOnce(&mut System) -> JuizResult<()>) -> JuizResult<()> {
        log::debug!("System::run_and_do() called");
        // self.setup().context("System::setup() in System::run_and_do() failed.")?;
        log::info!("Juiz System({}) Now Started.", self.store.uuid()?);
        (func)(self).context("User function passed for System::run_and_do() failed.")?;
        self.wait_for_singal().context("System::wait_for_signal() in System::run_and_do() failed.")?;
        log::debug!("System::run_and_do() exit");
        self.stop()?;
        self.cleanup()?;
        Ok(())
    }

    pub fn run_and_do_once(&mut self, func: impl FnOnce(&mut System) -> JuizResult<()>) -> JuizResult<()>  {
        log::debug!("System::run_and_do_once() called");
        // self.setup().context("System::setup() in System::run_and_do_once() failed.")?;
        log::debug!("Juiz System Now Started.");
        (func)(self).context("User function passed for System::run_and_do_once() failed.")?;
        //self.wait_for_singal().context("System::wait_for_signal() in System::run_and_do() failed.")?;
        self.stop()?;
        log::debug!("System::run_and_do_once() exit");
        self.cleanup()?;
        Ok(())
    }

    // pub fn broker_proxy(&self, manifest: &Value, create_when_not_found: bool) -> JuizResult<Arc<Mutex<dyn BrokerProxy>>> {
    //     log::trace!("System::broker_proxy({}) called", manifest);
    //     self.core_broker.lock_mut()?.broker_proxy_from_manifest(manifest, create_when_not_found)
    // }

    pub fn register_broker_factories_wrapper(&mut self, bf: Arc<Mutex<BrokerFactoriesWrapper>>) -> JuizResult<Arc<Mutex<BrokerFactoriesWrapper>>> {
        let type_name = juiz_lock(&bf)?.type_name().to_string();
        log::trace!("System::register_broker_factories_wrapper(BrokerFactory(type_name={:?})) called", type_name);
        if self.store.lock()?.broker_factories.contains_key(&type_name) {
            log::error!("system already contains broker factory with type_name='{type_name:}'.");
            return Err(anyhow::Error::from(JuizError::BrokerFactoryOfSameTypeNameAlreadyExistsError{type_name: type_name}));
        }
        self.store.lock_mut()?.broker_factories.insert(type_name.clone(), Arc::clone(&bf));
        self.core_broker().lock_mut()?.worker_mut().store_mut().register_broker_factory_manifest(type_name.as_str(), juiz_lock(&bf)?.profile_full()?.try_into()?)?;
        self.core_broker().lock_mut()?.worker_mut().store_mut().broker_proxies.register_factory(juiz_lock(&bf)?.broker_proxy_factory.clone())?;
        log::trace!("System::register_broker_factories_wrapper(BrokerFactory(type_name={:?})) exit", type_name);
        Ok(bf)
    }

    fn broker_factories_wrapper(&self, type_name: &str) -> JuizResult<Arc<Mutex<BrokerFactoriesWrapper>>> {
        match self.store.lock()?.broker_factories.get(type_name) {
            None => Err(anyhow::Error::from(JuizError::BrokerFactoryCanNotFoundError{type_name: type_name.to_string()})),
            Some(bf) => Ok(bf.clone())
        }
    }

    pub fn create_broker(&mut self, manifest: &Value) -> JuizResult<BrokerPtr> {
        log::trace!("System::create_broker({manifest:}) called");
        let type_name = obj_get_str(manifest, "type_name")?;
        let bf = self.broker_factories_wrapper(type_name)?;
        let b = juiz_lock(&bf)?.create_broker(&manifest).context("BrokerFactoriesWrapper.create_broker() failed in System::create_broker()")?;
        self.register_broker(b)
    }

    pub(crate) fn register_broker(&mut self, broker: BrokerPtr) -> JuizResult<BrokerPtr> {
        let type_name = broker.lock()?.type_name().to_owned();
        log::trace!("System::register_broker(type_name={type_name:}) called");
        self.store.lock_mut()?.brokers.insert(type_name.clone(), broker.clone());
        let p: Value  = broker.lock()?.profile_full()?.try_into()?;
        self.core_broker().lock_mut()?.worker_mut().store_mut().register_broker_manifest(type_name.as_str(), p)?;
        Ok(broker)
    }

    pub fn create_broker_proxy(&mut self, manifest: &Value) -> JuizResult<Arc<Mutex<dyn BrokerProxy>>> {
        log::trace!("System::create_broker_proxy({manifest:}) called");
        //self.core_broker().lock_mut().cre
        let bp = self.core_broker().lock()?.create_broker_proxy(manifest.clone())?;
        self.register_broker_proxy(bp)
    }
    
    pub(crate) fn register_broker_proxy(&mut self, broker_proxy: Arc<Mutex<dyn BrokerProxy>>) -> JuizResult<Arc<Mutex<dyn BrokerProxy>>> {
        let type_name =juiz_lock(&broker_proxy).context("Locking broker to get type_name failed.")?.type_name().to_string();
        log::trace!("System::register_broker(type_name={type_name:}) called");
        self.core_broker().lock_mut()?.worker_mut().store_mut().broker_proxies.register(broker_proxy.clone())?;
        Ok(broker_proxy)
    }

    pub fn process_list(&self, recursive: bool) -> JuizResult<Vec<Value>> {
        log::trace!("System::process_list() called");
        let mut local_processes = self.core_broker().lock()?.worker().store().processes_profile_full()?.as_object().unwrap().values().into_iter().map(|v|{v.clone()}).collect::<Vec<Value>>();
        if recursive {
            for (_, proxy) in self.core_broker().lock()?.worker().store().broker_proxies.objects().iter() {
                log::trace!("process_list for proxy ()");
                for v in get_array(&juiz_lock(proxy)?.process_list(recursive)?)?.iter() {
                    local_processes.push(v.clone());
                }
            }
        }
        log::debug!("ids: {local_processes:?}");    
        return Ok(local_processes);
    }

    pub fn container_list(&self, recursive: bool) -> JuizResult<Vec<Value>> {
        log::trace!("System::container_list() called");
        let mut local_containers = self.core_broker().lock()?.worker().store().containers_profile_full()?.as_object().unwrap().values().into_iter().map(|v|{v.clone()}).collect::<Vec<Value>>();

        if recursive {
            for (_, proxy) in self.core_broker().lock()?.worker().store().broker_proxies.objects().iter() {
                match juiz_lock(proxy) {
                    Err(e) => return Err(e),
                    Ok(p) => {
                        match p.container_list(recursive) {
                            Ok(v) => {
                                for v in get_array(&v)?.iter() {
                                    local_containers.push(v.clone());
                                }
                            }
                            Err(e) => {
                                log::error!("BrokerProxy({:}).container_list() in System::container_list() failed. Error({e:?}) ", p.identifier());
                            }
                        }
                    }
                }
            }
        }
        log::debug!("ids: {local_containers:?}");    
        return Ok(local_containers);
    }

    pub fn container_process_list(&self, recursive: bool) -> JuizResult<Vec<Value>> {
        log::trace!("System::container_process_list() called");
        let mut local_processes = self.core_broker().lock()?.worker().store().container_processes_profile_full()?.as_object().unwrap().values().into_iter().map(|v|{v.clone()}).collect::<Vec<Value>>();
        if recursive {
            for (_, proxy) in self.core_broker().lock()?.worker().store().broker_proxies.objects().iter() {
                for v in get_array(&juiz_lock(proxy)?.container_process_list(recursive)?)?.iter() {
                    local_processes.push(v.clone());
                }
            }
        }
        log::debug!("ids: {local_processes:?}");    
        return Ok(local_processes);
    }

    pub fn any_process_list(&self, recursive: bool) -> JuizResult<Vec<Value>> {
        log::trace!("System::any_process_list() called");
        let mut ps = self.process_list(recursive)?;
        let mut cps = self.container_process_list(recursive)?;
        cps.append(&mut ps);
        return Ok(cps)
    }

    pub fn ec_list(&self, recursive: bool) -> JuizResult<Vec<Value>> {
        log::trace!("System::ec_list() called");
        let mut local_ecs = self.core_broker().lock()?.worker().store().ecs.list_manifests()?;
        if recursive {
            for (_, proxy) in self.core_broker().lock()?.worker().store().broker_proxies.objects().iter() {
                log::trace!("ec_list for proxy ()");
                for v in get_array(&juiz_lock(proxy)?.ec_list(recursive)?)?.iter() {
                    local_ecs.push(v.clone());
                }
            }
        }
        log::debug!("ids: {local_ecs:?}");    
        return Ok(local_ecs);
    }

    //
    // pub fn ec_from_id(&self, id: &Identifier) -> JuizResult<Arc<Mutex<dyn ExecutionContextFunction>>> {
    //     log::trace!("System::ec_from_id(id={id:}) called");
    //     let s = IdentifierStruct::try_from(id.clone()).unwrap();
    //     if s.broker_type_name == "core" {
    //         return self.core_broker().lock()?.worker().store().ecs.get(id);
    //     }
    //     self.ec_proxy(id)
    // }
    
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