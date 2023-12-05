

use std::sync::Arc;
use std::sync::Mutex;


use crate::utils::check_corebroker_manifest;
use crate::utils::juiz_lock;
use crate::utils::manifest_util::type_name;
use crate::{Value, jvalue, Process, Broker, Identifier, JuizError, ProcessFactory, JuizResult, ProcessFunction, core::core_store::CoreStore};

use crate::connection::connect;
use crate::process::process_factory_impl::ProcessFactoryImpl;


pub fn check_broker_create_process_manifest(manifest: Value) -> Result<Value, JuizError> {
    let mut manifest_updated = manifest.clone();
    match manifest_updated.as_object_mut() {
        None => return Err(JuizError::ProcessManifestError{}),
        Some(hash_map) => {
            match hash_map.get("name") {
                None => return Err(JuizError::ManifestNameMissingError{}),
                Some(_) => { /* Do Nothing */ }
            }
            match hash_map.get("type_name") {
                None => return Err(JuizError::ManifestTypeNameMissingError{}),
                Some(_) => { /* Do Nothing */ }
            }
        }
    }
    return Ok(manifest_updated)
}



#[allow(unused)]
pub struct CoreBroker {
    manifest: Value,
    core_store: CoreStore,
}

impl CoreBroker {

    pub fn new(manifest: Value) -> Result<CoreBroker, JuizError> {
        Ok(CoreBroker{
            manifest: check_corebroker_manifest(manifest)?,
            core_store: CoreStore::new()
        })
    }

    pub fn store(&self) -> &CoreStore {
        &self.core_store
    }

    pub fn store_mut(&mut self) -> &mut CoreStore {
        &mut self.core_store
    }

    fn push_process(&mut self, p: Arc<Mutex<dyn Process>>) -> Result<Arc<Mutex<dyn Process>>, JuizError> {
        self.store_mut().register_process(p)
    }

    pub fn push_process_factory(&mut self, process_factory: Arc<Mutex<dyn ProcessFactory>>) -> JuizResult<Arc<Mutex<dyn ProcessFactory>>> {
        self.core_store.register_process_factory(process_factory)
    }

    pub fn register_process_factory(&mut self, manifest: Value, function: ProcessFunction) -> Result<Arc<Mutex<dyn ProcessFactory>>, JuizError> {
        self.core_store.register_process_factory(ProcessFactoryImpl::new(manifest, function)?)
    }

    fn generate_process_name_from_type_name(&self, mut manifest: Value) -> Result<Value, JuizError> {
        if manifest.get("name").is_some() {
            return Ok(manifest);
        }
        let name = type_name(&manifest)?.to_string() + "0";
        manifest.as_object_mut().unwrap().insert("name".to_string(), jvalue!(name));
        return Ok(manifest);
    }

    fn precreate_check<'b>(&'b self, manifest: Value) -> Result<Value, JuizError> {
        self.generate_process_name_from_type_name(check_broker_create_process_manifest(manifest)?)
    }

}

impl<'a> Broker for CoreBroker {

    fn is_in_charge_for_process(&self, id: &Identifier) -> bool {
        self.store().process(id).is_ok()
    }

    fn call_process(&self, id: &Identifier, args: Value) -> Result<Value, JuizError> {
        juiz_lock(&self.process(id)?)?.call(args)
    }

    fn process(&self, id: &Identifier) -> Result<Arc<Mutex<dyn Process>>, JuizError> {
        self.store().process(id)
    }

    fn connect_process_to(&mut self, source_process_id: &Identifier, arg_name: &String, target_process_id: &Identifier, manifest: Value) -> Result<Value, JuizError> {
        connect(self.process(source_process_id)?, self.process(target_process_id)?, arg_name, manifest)
    }

    fn create_process(&mut self, manifest: Value) -> Result<Arc<Mutex<dyn Process>>, JuizError> {
        log::trace!("CoreBroker::create_process(manifest={}) called", manifest);
        let arc_pf = self.core_store.process_factory(type_name(&manifest)?)?;
        let p = juiz_lock(arc_pf)?.create_process(self.precreate_check(manifest)?)?;
        self.push_process(p)
    }

    fn execute_process(&self, id: &Identifier) -> Result<Value, JuizError> {
        match self.process(id)?.try_lock() {
            Err(_) => Err(JuizError::CoreBrokerCanNotLockProcessMutexError{}),
            Ok(p) => p.execute()
        }
    }

    
}