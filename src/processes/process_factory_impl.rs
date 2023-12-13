
use std::sync::{Mutex, Arc};
use crate::{jvalue, value::obj_get_str, Process, ProcessFactory, processes::process_impl::ProcessImpl, JuizError, Value, JuizResult, utils::check_process_factory_manifest, JuizObject, Identifier, identifier::{identifier_from_manifest, create_factory_identifier_from_manifest}, object::{ObjectCore, JuizObjectCoreHolder, JuizObjectClass}};

#[repr(C)]
pub struct ProcessFactoryImpl {
    core: ObjectCore,
    manifest: Value,
    function: fn(Value) -> JuizResult<Value>,
}

pub fn create_process_factory(manifest: crate::Value, function: fn(Value) -> JuizResult<Value>) -> JuizResult<Arc<Mutex<dyn ProcessFactory>>> {
    log::trace!("create_process_factory called");
    ProcessFactoryImpl::new(manifest, function)
}

impl ProcessFactoryImpl {

    pub fn new(manifest: crate::Value, function: fn(Value) -> JuizResult<Value>) -> JuizResult<Arc<Mutex<dyn ProcessFactory>>> {
        let type_name = obj_get_str(&manifest, "type_name")?;
        Ok(Arc::new(Mutex::new(
            ProcessFactoryImpl{
                core: ObjectCore::create_factory(JuizObjectClass::ProcessFactory("ProcessFactoryImpl"),
                    type_name
                ),
                manifest: check_process_factory_manifest(manifest)?, 
                function
            }
        )))
    }

    fn apply_default_manifest(&self, manifest: Value) -> Result<Value, JuizError> {
        let mut new_manifest = self.manifest.clone();
        for (k, v) in manifest.as_object().unwrap().iter() {
            new_manifest.as_object_mut().unwrap().insert(k.to_owned(), v.clone());
        }
        return Ok(new_manifest);
    }
}

impl JuizObjectCoreHolder for ProcessFactoryImpl {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}


impl JuizObject for ProcessFactoryImpl {
}

impl ProcessFactory for ProcessFactoryImpl {

    fn create_process(&self, manifest: Value) -> JuizResult<Arc<Mutex<dyn Process>>>{
        log::trace!("ProcessFactoryImpl::create_process(manifest={}) called", manifest);
        Ok(Arc::new(Mutex::new(
            ProcessImpl::new(
                self.apply_default_manifest(manifest)?, 
                self.function)?
        )))
    }    
}
