

use std::sync::Arc;

use crate::connections::ConnectionFactoryImpl;
use crate::prelude::*;
use crate::processes::implementations::process_impl::ProcessImpl;
use crate::processes::{ProcessBodyFunctionTrait, ProcessBodyFunctionType};

#[repr(C)]
pub struct ProcessFactoryImpl {
    core: ObjectCore,
    manifest: ProcessManifest,
    function: Arc<ProcessBodyFunctionTrait>,
}

///
/// ProcessFactoryImplクラスの実装
impl ProcessFactoryImpl {

    pub fn new(manifest: ProcessManifest, function: ProcessBodyFunctionType) -> JuizResult<Self> {
        Ok(ProcessFactoryImpl{
            core: ObjectCore::create_factory(
                JuizObjectClass::ProcessFactory("ProcessFactoryImpl"),
                manifest.type_name.clone()),
            manifest, 
            function: Arc::new(function)
        })
    }

    pub fn new_from_clousure(manifest: ProcessManifest, function: impl Fn(CapsuleMap) -> JuizResult<Capsule> + 'static) -> JuizResult<Self> {
        Ok(ProcessFactoryImpl{
            core: ObjectCore::create_factory(
                JuizObjectClass::ProcessFactory("ProcessFactoryImpl"),
                manifest.type_name.clone()),
            manifest, 
            function: Arc::new(function)
        })
    }

    // fn apply_default_manifest(&self, manifest: Value) -> Result<Value, JuizError> {
    //     let mut new_manifest = self.manifest.clone();
    //     for (k, v) in manifest.as_object().unwrap().iter() {
    //         new_manifest.as_object_mut().unwrap().insert(k.to_owned(), v.clone());
    //     }
    //     return Ok(new_manifest);
    // }
}

impl JuizObjectCoreHolder for ProcessFactoryImpl {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}


impl JuizObject for ProcessFactoryImpl {
}

impl ProcessFactory for ProcessFactoryImpl {

    fn create_process(&self, manifest: ProcessManifest) -> JuizResult<ProcessPtr> {
        log::trace!("ProcessFactoryImpl::create_process(manifest={:?}) called", manifest);
        Ok(ProcessPtr::new(
            ProcessImpl::new_from_clousure_ref(
                self.manifest.build_instance_manifest(manifest)?, 
                self.function.clone(), 
            Box::new(ConnectionFactoryImpl::new()))?
        ))
    }
}


impl Drop for ProcessFactoryImpl {

    fn drop(&mut self) {
        log::trace!("ProcessFactoryImpl({})::drop() called", self.type_name());
    }
}