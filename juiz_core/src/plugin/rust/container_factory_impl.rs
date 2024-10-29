

use std::sync::Arc;

use crate::prelude::*;

pub type ContainerConstructor = dyn Fn(ContainerManifest)->JuizResult<ContainerPtr>;
#[repr(C)]
pub struct ContainerFactoryImpl {
    core: ObjectCore,
    manifest: ContainerManifest,
    // constructor: Arc<ContainerConstructFunctionTrait<T>>,
    binded_container_constructor: Arc<ContainerConstructor>,
}

impl ContainerFactoryImpl {

    pub fn new(manifest: ContainerManifest, constructor: Arc<ContainerConstructor>) -> JuizResult<Self> {
        Ok(ContainerFactoryImpl{
                core: ObjectCore::create_factory(JuizObjectClass::ContainerFactory("ContainerFactoryImpl"), manifest.type_name.clone().as_str()),
                manifest,
                // constructor: Arc::new(constructor),
                binded_container_constructor: constructor,
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


impl JuizObjectCoreHolder for ContainerFactoryImpl {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}

impl JuizObject for ContainerFactoryImpl {}

impl ContainerFactory for ContainerFactoryImpl {

    fn create_container(&self, _core_worker: &mut CoreWorker, manifest: ContainerManifest) -> JuizResult<ContainerPtr>{
        log::trace!("ContainerFactoryImpl::create_container(manifest={:?}) called", manifest);
        // Ok(ContainerPtr::new(ContainerImpl::new(
        //         // self.apply_default_manifest(manifest.clone())?,
        //         manifest.clone(),
        //         (self.constructor)(manifest)?
        //     )?))
        (self.binded_container_constructor)(manifest)
    }
    
    fn destroy_container(&mut self, c: ContainerPtr) -> JuizResult<Value> {
        // todo!()
        log::trace!("ContainerFractoryImpl::destroy_container() called");
        c.lock()?.profile_full()
    }
    
}
