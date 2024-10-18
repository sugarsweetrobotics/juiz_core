
use std::sync::{Arc, Mutex};

use crate::object::JuizObjectClass;
use crate::prelude::*;
use crate::utils::check_process_factory_manifest;
use crate::value::obj_get_str;
use crate::object::{JuizObjectCoreHolder, ObjectCore};
use crate::processes::{process_new, FunctionType};

#[repr(C)]
pub struct ProcessFactoryImpl {
    core: ObjectCore,
    manifest: Value,
    function: FunctionType,
}

///
/// ProcessFactoryImplクラスの実装
impl ProcessFactoryImpl {

    pub fn new(manifest: Value, function: FunctionType) -> JuizResult<Self> {
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

    pub fn create(manifest: Value, function: FunctionType) -> JuizResult<ProcessFactoryPtr> {
       log::trace!("ProcessFactoryImpl::create({:}) called", manifest);
       Ok(Arc::new(Mutex::new(ProcessFactoryImpl::new(manifest, function)?)))
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

    fn create_process(&self, manifest: Value) -> JuizResult<ProcessPtr> {
        log::trace!("ProcessFactoryImpl::create_process(manifest={}) called", manifest);
        Ok(ProcessPtr::new(
            process_new(
                self.apply_default_manifest(manifest)?, 
                self.function)?
        ))
    }    
}


impl Drop for ProcessFactoryImpl {

    fn drop(&mut self) {
        log::trace!("ProcessFactoryImpl({})::drop() called", self.type_name());
    }
}