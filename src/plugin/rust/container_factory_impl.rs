
use std::sync::{Mutex, Arc};

use crate::prelude::*;
use crate::{containers::{ContainerImpl, ContainerConstructFunction}, object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore}, utils::check_process_factory_manifest, value::obj_get_str};


#[repr(C)]
pub struct ContainerFactoryImpl<T> {
    core: ObjectCore,
    manifest: Value,
    constructor: ContainerConstructFunction<T>
}

impl<S: 'static> ContainerFactoryImpl<S> {

    pub fn new(manifest: Value, constructor: ContainerConstructFunction<S>) -> JuizResult<Self> {
        let type_name = obj_get_str(&manifest, "type_name")?;
        Ok(ContainerFactoryImpl::<S>{
                core: ObjectCore::create_factory(JuizObjectClass::ContainerFactory("ContainerFactoryImpl"), type_name),
                manifest: check_process_factory_manifest(manifest)?,
                constructor
        })
    }

    pub fn create(manifest: Value, constructor: ContainerConstructFunction<S>) -> JuizResult<Arc<Mutex<dyn ContainerFactory>>> {
        Ok(Arc::new(Mutex::new(Self::new(manifest, constructor)?)))
    }

    fn apply_default_manifest(&self, manifest: Value) -> Result<Value, JuizError> {
        let mut new_manifest = self.manifest.clone();
        for (k, v) in manifest.as_object().unwrap().iter() {
            new_manifest.as_object_mut().unwrap().insert(k.to_owned(), v.clone());
        }
        return Ok(new_manifest);
    }
}


impl<T: 'static> JuizObjectCoreHolder for ContainerFactoryImpl<T> {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}

impl<T: 'static> JuizObject for ContainerFactoryImpl<T> {}

impl<T: 'static> ContainerFactory for ContainerFactoryImpl<T> {

    fn create_container(&self, core_worker: &mut CoreWorker, manifest: Value) -> JuizResult<ContainerPtr>{
        log::trace!("ContainerFactoryImpl::create_container(manifest={}) called", manifest);
        Ok(ContainerPtr::new(ContainerImpl::new(
                self.apply_default_manifest(manifest.clone())?,
                (self.constructor)(manifest)?
            )?))
    }
    
    fn destroy_container(&mut self, c: ContainerPtr) -> JuizResult<Value> {
        // todo!()
        log::trace!("ContainerFractoryImpl::destroy_container() called");
        c.lock()?.profile_full()
    }
    
}
