

use std::sync::Arc;

use crate::containers::ContainerConstructFunctionTrait;
use crate::prelude::*;
use crate::{containers::ContainerImpl, object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore}};


#[repr(C)]
pub struct ContainerFactoryImpl<T> {
    core: ObjectCore,
    manifest: ContainerManifest,
    constructor: Arc<ContainerConstructFunctionTrait<T>>,
}

impl<S: 'static> ContainerFactoryImpl<S> {

    pub fn new(manifest: ContainerManifest, constructor: impl Fn(ContainerManifest) -> JuizResult<Box<S>> + 'static) -> JuizResult<Self> {
        Ok(ContainerFactoryImpl::<S>{
                core: ObjectCore::create_factory(JuizObjectClass::ContainerFactory("ContainerFactoryImpl"), manifest.type_name.clone().as_str()),
                manifest,
                constructor: Arc::new(constructor)
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


impl<T: 'static> JuizObjectCoreHolder for ContainerFactoryImpl<T> {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}

impl<T: 'static> JuizObject for ContainerFactoryImpl<T> {}

impl<T: 'static> ContainerFactory for ContainerFactoryImpl<T> {

    fn create_container(&self, _core_worker: &mut CoreWorker, manifest: ContainerManifest) -> JuizResult<ContainerPtr>{
        log::trace!("ContainerFactoryImpl::create_container(manifest={:?}) called", manifest);
        Ok(ContainerPtr::new(ContainerImpl::new(
                // self.apply_default_manifest(manifest.clone())?,
                manifest.clone(),
                (self.constructor)(manifest)?
            )?))
    }
    
    fn destroy_container(&mut self, c: ContainerPtr) -> JuizResult<Value> {
        // todo!()
        log::trace!("ContainerFractoryImpl::destroy_container() called");
        c.lock()?.profile_full()
    }
    
}
