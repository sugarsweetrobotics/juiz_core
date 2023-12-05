use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use crate::result::JuizResult;
use crate::sync_util::juiz_lock;
use super::system_builder;
use crate::{CoreBroker, Value, JuizError, Identifier, Process};
use crate::manifest_util::when_contains_do;

use std::time;
use crate::manifest_util::*;


pub struct System {
    manifest: Value,
    core_broker: Arc<Mutex<CoreBroker>>,
    sleep_time: Duration,
}

fn check_system_manifest(manifest: Value) -> JuizResult<Value> {
    if !manifest.is_object() {
        return Err(JuizError::ManifestIsNotObjectError{});
    }
    return Ok(manifest);
}

impl System {

    pub fn new(manifest: Value) -> JuizResult<System> {
        env_logger::init();
        let updated_manifest = check_system_manifest(manifest)?;
        Ok(System{
            manifest: updated_manifest.clone(),
            core_broker: Arc::new(Mutex::new(CoreBroker::new(updated_manifest)?)),
            sleep_time: time::Duration::from_millis(100) 
        })
    }

    pub fn core_broker(&self) -> &Arc<Mutex<CoreBroker>> {
        &self.core_broker
    }

    pub fn process_from_id(&self, id: &Identifier) -> JuizResult<Arc<Mutex<dyn Process>>> {
        juiz_lock(&self.core_broker)?.store().process(id)
    }

    pub fn process_from_manifest(&self, manifest: &Value) -> JuizResult<Arc<Mutex<dyn Process>>> {
        let id = id_from_manifest(manifest)?.to_string();
        self.process_from_id(&id)
    }

    fn setup_plugins(&mut self) -> JuizResult<()> {
        let manif = self.manifest.as_object_mut().unwrap().get_mut("plugins").unwrap().clone();
        system_builder::system_builder::setup_plugins(self, &manif)?;
        Ok(())
    }

    fn setup(&mut self) -> JuizResult<()> {
        log::trace!("System::setup() called");

        let _ = when_contains_do(&self.manifest, "plugins", |v| {
            system_builder::system_builder::setup_plugins(self, v)
        }).expect("setup_plugins during System::setup() failed.");

        let _ = when_contains_do(&self.manifest, "processes", |v| {
            system_builder::system_builder::setup_processes(self, v)
        }).expect("setup_processes during System::setup() failed.");

        let _ =  when_contains_do(&self.manifest, "connections", |v| {
            system_builder::system_builder::setup_connections(self, v)
        }).expect("setup_connections during System::setup() failed.");
        
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
}