use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::brokers::broker_factories_wrapper::BrokerFactoriesWrapper;
use crate::value::obj_get_str;
use crate::{JuizResult, Container, BrokerProxy, Broker};
use crate::utils::juiz_lock;
use crate::utils::manifest_util::{id_from_manifest, when_contains_do_mut};
use super::system_builder;
use crate::{CoreBroker, Value, JuizError, Identifier, Process, jvalue};
use crate::utils::when_contains_do;

use std::time;

#[allow(dead_code)]
pub struct System {
    manifest: Value,
    core_broker: Arc<Mutex<CoreBroker>>,
    sleep_time: Duration,

    broker_factories: HashMap<String, Arc<Mutex<BrokerFactoriesWrapper>>>,
    brokers: HashMap<String, Arc<Mutex<dyn Broker>>>,
    broker_proxies: HashMap<String, Arc<Mutex<dyn BrokerProxy>>>,
    
}

fn check_system_manifest(manifest: Value) -> JuizResult<Value> {
    if !manifest.is_object() {
        return Err(anyhow::Error::from(JuizError::ValueIsNotObjectError{value:manifest}).context("check_system_manifest failed."));
    }
    return Ok(manifest);
}

impl System {

    pub fn new(manifest: Value) -> JuizResult<System> {
        env_logger::init();
        let updated_manifest = check_system_manifest(manifest)?;
        Ok(System{
            manifest: updated_manifest.clone(),
            core_broker: Arc::new(Mutex::new(CoreBroker::new(jvalue!({"name": "core_broker"}))?)),
            sleep_time: time::Duration::from_millis(100),
            broker_factories: HashMap::new(),
            brokers: HashMap::new(),
            broker_proxies: HashMap::new(),
        })
    }

    pub(crate) fn core_broker(&self) -> &Arc<Mutex<CoreBroker>> {
        &self.core_broker
    }

    pub fn process_from_id(&self, id: &Identifier) -> JuizResult<Arc<Mutex<dyn Process>>> {
        juiz_lock(&self.core_broker)?.store().process(id)
    }

    pub fn container_from_id(&self, id: &Identifier) -> JuizResult<Arc<Mutex<dyn Container>>> {
        juiz_lock(&self.core_broker)?.store().container(id)
    }

    pub fn process_from_manifest(&self, manifest: &Value) -> JuizResult<Arc<Mutex<dyn Process>>> {
        let id = id_from_manifest(manifest)?.to_string();
        self.process_from_id(&id)
    }

    fn setup(&mut self) -> JuizResult<()> {
        log::trace!("System::setup() called");
        let manifest_copied = self.manifest.clone();
        let _ = when_contains_do_mut(&manifest_copied, "plugins", |v| {
            system_builder::system_builder::setup_plugins(self, v)
        }).expect("setup_plugins during System::setup() failed.");

        let _ = when_contains_do(&self.manifest, "processes", |v| {
            system_builder::system_builder::setup_processes(self, v)
        }).expect("setup_processes during System::setup() failed.");

        let _ = when_contains_do(&self.manifest, "containers", |v| {
            system_builder::system_builder::setup_containers(self, v)
        }).expect("setup_containers during System::setup() failed.");

        system_builder::system_builder::setup_local_broker_factory(self).expect("setup_local_broker_factory durin System::setup() failed.");
        system_builder::system_builder::setup_local_broker(self).expect("setup_local_broker during System::setup() failed.");

        let _ = when_contains_do_mut(&manifest_copied, "brokers", |v| {
            system_builder::system_builder::setup_brokers(self, v)
        }).expect("setup_brokers during System::setup() failed.");

        let _ =  when_contains_do(&self.manifest, "connections", |v| {
            system_builder::system_builder::setup_connections(self, v)
        }).expect("setup_connections during System::setup() failed.");
        
        for (type_name, broker) in self.brokers.iter() {
            log::info!("starting Broker({type_name:})");
            let _ = juiz_lock(&broker)?.start()?;
        }

        log::debug!("System::setup() successfully finished.");
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
        self.setup()?;
        self.wait_for_singal()?;
        log::debug!("System::run() exit");
        Ok(())
    }

    pub fn run_and_do(&mut self, func: fn(&mut System) -> JuizResult<()>) -> JuizResult<()> {
        log::debug!("System::run() called");
        log::debug!("Juiz System Now Started.");
        self.setup()?;
        (func)(self)?;
        self.wait_for_singal()?;
        log::debug!("System::run() exit");
        Ok(())
    }

    pub fn profile_full(&self) -> JuizResult<Value> {
        Ok(jvalue!({
            "core_broker": self.core_broker().lock().unwrap().profile_full()?
        }))
    }

    pub fn broker_proxy(&self, manifest: &Value) -> JuizResult<Arc<Mutex<dyn BrokerProxy>>> {
        log::trace!("System::broker_proxy({}) called", manifest);
        let type_name = obj_get_str(manifest, "type_name")?;
        if type_name == "core" {
            let cbf = Arc::clone(self.core_broker());
            return Ok(cbf);
        }
        let bfw = self.broker_factories_wrapper(type_name)?;
        juiz_lock(bfw)?.create_broker_proxy(manifest)
    }

    pub fn register_broker_factories_wrapper(&mut self, bf: Arc<Mutex<BrokerFactoriesWrapper>>) -> JuizResult<Arc<Mutex<BrokerFactoriesWrapper>>> {
        let type_name = juiz_lock(&bf)?.type_name().to_string();
        log::debug!("System::register_broker_factories_wrapper(BrokerFactory(type_name={:?})) called", type_name);
        if self.broker_factories.contains_key(&type_name) {
            return Err(anyhow::Error::from(JuizError::BrokerFactoryOfSameTypeNameAlreadyExistsError{type_name: type_name}));
        }
        self.broker_factories.insert(type_name, Arc::clone(&bf));
        Ok(bf)
    }

    fn broker_factories_wrapper(&self, type_name: &str) -> JuizResult<&Arc<Mutex<BrokerFactoriesWrapper>>> {
        match self.broker_factories.get(type_name) {
            None => Err(anyhow::Error::from(JuizError::BrokerFactoryCanNotFoundError{type_name: type_name.to_string()})),
            Some(bf) => Ok(bf)
        }
    }

    pub fn create_broker(&self, manifest: &Value) -> JuizResult<Arc<Mutex<dyn Broker>>> {
        let type_name = obj_get_str(manifest, "type_name")?;
        let bf = self.broker_factories_wrapper(type_name)?;
        juiz_lock(bf)?.create_broker(&manifest)
    }

    pub(crate) fn register_broker(&mut self, broker: Arc<Mutex<dyn Broker>>) -> JuizResult<Arc<Mutex<dyn Broker>>> {
        let type_name = juiz_lock(&broker)?.type_name().to_string();
        self.brokers.insert(type_name, Arc::clone(&broker));
        Ok(broker)
    }

}