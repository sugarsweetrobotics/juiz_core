use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::Context;

use crate::jvalue;
use crate::brokers::{BrokerProxy, Broker,  broker_factories_wrapper::BrokerFactoriesWrapper};
use crate::object::{ObjectCore, JuizObjectCoreHolder, JuizObjectClass};

use crate::value::{obj_get_str, obj_merge};
use crate::{ContainerPtr, CoreBroker, Identifier, JuizError, JuizObject, JuizResult, ProcessPtr, Value};
use crate::utils::juiz_lock;
use crate::utils::manifest_util::{id_from_manifest, when_contains_do_mut, construct_id};
use super::system_builder::system_builder;
use crate::utils::when_contains_do;

use std::time;

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
        let updated_manifest = check_system_manifest(manifest)?;
        Ok(System{
            core: ObjectCore::create(JuizObjectClass::System("System"), "system", "system"),
            manifest: updated_manifest.clone(),
            core_broker: Arc::new(Mutex::new(CoreBroker::new(jvalue!({"type_name": "CoreBroker", "name": "core_broker"}))?)),
            sleep_time: time::Duration::from_millis(100),
            broker_factories: HashMap::new(),
            brokers: HashMap::new(),
            broker_proxies: HashMap::new(),
            tokio_runtime: tokio::runtime::Builder::new_multi_thread().thread_name("juiz_core::System").worker_threads(4).enable_all().build().unwrap(),

        })
    }

    pub(crate) fn core_broker(&self) -> &Arc<Mutex<CoreBroker>> {
        &self.core_broker
    }


    ///
    pub fn process_from_id(&self, id: &Identifier) -> JuizResult<ProcessPtr> {
        juiz_lock(&self.core_broker)?.store().processes.get(id)
    }

    pub fn process_from_typename_and_name(&self, type_name: &str, name: &str) -> JuizResult<ProcessPtr> {
        let id = construct_id("Process", type_name, name, "core", "core");
        juiz_lock(&self.core_broker)?.store().processes.get(&id)
    }
    
    
    pub fn container_from_id(&self, id: &Identifier) -> JuizResult<ContainerPtr> {
        juiz_lock(&self.core_broker)?.store().containers.get(id)
    }

    pub fn container_from_typename_and_name(&self, type_name: &str, name: &str) -> JuizResult<ContainerPtr> {
        let id = construct_id("Container", type_name, name, "core", "core");
        juiz_lock(&self.core_broker)?.store().containers.get(&id)
    }

    pub fn container_process_from_id(&self, id: &Identifier) -> JuizResult<ProcessPtr> {
        juiz_lock(&self.core_broker)?.store().container_processes.get(id)
    }

    pub fn any_process_from_id(&self, id: &Identifier) -> JuizResult<ProcessPtr> {
        let result = self.process_from_id(id);
        if result.is_ok() {
            return result;
        }
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

        let _ = when_contains_do_mut(&manifest_copied, "brokers", |v| {
            system_builder::setup_brokers(self, v).context("system_builder::setup_brokers in System::setup() failed.")
        })?;

        for (type_name, broker) in self.brokers.iter() {
            log::info!("starting Broker({type_name:})");
            let _ = juiz_lock(&broker)?.start().context("Broker(type_name={type_name:}).start() in System::setup() failed.")?;
        }

        let _ = when_contains_do(&self.manifest, "ecs", |v| {
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

    fn spin(&mut self) -> () {
        // log::debug!("System::spin() called");
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

    pub fn run_and_do_once(&mut self,  func: fn(&mut System) -> JuizResult<()>) -> JuizResult<()> {
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


    
}