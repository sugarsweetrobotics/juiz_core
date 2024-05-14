
use std::sync::{Mutex, Arc};
use crate::{object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore}, processes::{process_impl::ProcessImpl, process_ptr}, utils::check_process_factory_manifest, value::obj_get_str, JuizError, JuizObject, JuizResult, ProcessFactory, ProcessPtr, Value};
use super::process_impl::FunctionType;

#[repr(C)]
pub struct ProcessFactoryImpl {
    core: ObjectCore,
    manifest: Value,
    function: FunctionType,
}

pub fn create_process_factory(manifest: crate::Value, function: FunctionType) -> JuizResult<Arc<Mutex<dyn ProcessFactory>>> {
    log::trace!("create_process_factory called");
    Ok(Arc::new(Mutex::new(ProcessFactoryImpl::new(manifest, function)?)))
}

impl ProcessFactoryImpl {

    pub fn new(manifest: crate::Value, function: FunctionType) -> JuizResult<Self> {
        let type_name = obj_get_str(&manifest, "type_name")?;
        Ok(
            ProcessFactoryImpl{
                core: ObjectCore::create_factory(JuizObjectClass::ProcessFactory("ProcessFactoryImpl"),
                    type_name
                ),
                manifest: check_process_factory_manifest(manifest)?, 
                function
            }
        )
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

    fn create_process(&self, manifest: Value) -> JuizResult<ProcessPtr>{
        log::trace!("ProcessFactoryImpl::create_process(manifest={}) called", manifest);
        Ok(process_ptr(
            ProcessImpl::new(
                self.apply_default_manifest(manifest)?, 
                self.function)?
        ))
    }    
}
