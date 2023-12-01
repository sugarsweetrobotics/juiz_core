

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;


use crate::ProcessFunction;
use crate::{Value, jvalue, Process, Broker, Identifier, JuizError, ProcessFactory};

use crate::core_store::CoreStore;
use crate::manifest_util::*;
use crate::connection_builder::connection_builder::connect;
use crate::manifest_checker::*;
use crate::process_factory_impl::ProcessFactoryImpl;

#[allow(unused)]
pub struct CoreBroker {
    manifest: Value,
    processes: HashMap<Identifier, Arc<Mutex<dyn Process>>> ,
    core_store: CoreStore,
}

impl CoreBroker {

    pub fn new(manifest: Value) -> Result<CoreBroker, JuizError> {
        Ok(CoreBroker{
            manifest: check_corebroker_manifest(manifest)?,
            processes: HashMap::new(), 
            core_store: CoreStore::new()
        })
    }

    pub fn push_process(&mut self, p: Arc<Mutex<dyn Process>>) -> Result<Arc<Mutex<dyn Process>>, JuizError> {
        let id = match p.try_lock() {
            Err(_) => Err(JuizError::CoreBrokerCanNotLockProcessMutexError{}),
            Ok(proc) => Ok(proc.identifier().clone())
        }?;
        self.processes.insert(id.clone(), Arc::clone(&p));
        self.process(&id)
    }

    pub fn register_process_factory(&mut self, manifest: Value, function: ProcessFunction) -> Result<&Arc<Mutex<dyn ProcessFactory>>, JuizError> {
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

    fn precreate_check(&self, manifest: Value) -> Result<Value, JuizError> {
        self.generate_process_name_from_type_name(check_broker_create_process_manifest(manifest)?)
    }
}

impl<'a> Broker for CoreBroker {

    fn is_in_charge_for_process(&self, id: &Identifier) -> bool {
        self.processes.get(id).is_some()
    }

    fn call_process(&self, id: &Identifier, args: Value) -> Result<Value, JuizError> {
        match self.process(id)?.try_lock() {
            Err(_e) => Err(JuizError::CoreBrokerCanNotLockProcessMutexError{}),
            Ok(proc) => proc.call(args)
        }
    }

    fn process(&self, id: &Identifier) -> Result<Arc<Mutex<dyn Process>>, JuizError> {
        match self.processes.get(id) {
            None => Err(JuizError::ProcessCanNotFoundError{}),
            Some(p) => Ok(Arc::clone(p))
        }
    }

    fn connect_process_to(&mut self, source_process_id: &Identifier, arg_name: &String, target_process_id: &Identifier, manifest: Value) -> Result<Value, JuizError> {
        connect(self.process(source_process_id)?, self.process(target_process_id)?, arg_name, manifest)
    }

    fn create_process(&mut self, manifest: Value) -> Result<Arc<Mutex<dyn Process>>, JuizError> {
        let arc_pf = self.core_store.process_factory(type_name(&manifest)?)?;
        let p = match arc_pf.try_lock() {
            Err(_) => Err(JuizError::CoreBrokerCanNotLockProcessFactoryMutexError{}),
            Ok(pf) => Ok(pf.create_process(self.precreate_check(manifest)?)?)
        }?;
        self.push_process(p)
    }

    fn execute_process(&self, id: &Identifier) -> Result<Value, JuizError> {
        match self.process(id)?.try_lock() {
            Err(_) => Err(JuizError::CoreBrokerCanNotLockProcessMutexError{}),
            Ok(p) => p.execute()
        }
    }
}