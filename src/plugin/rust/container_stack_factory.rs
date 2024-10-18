
use std::sync::{Arc, Mutex};

use crate::prelude::*;
use crate::{containers::ContainerImpl, object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore}, utils::check_process_factory_manifest, value::obj_get_str};

pub type ContainerStackConstructFunction<S>=fn(ContainerPtr, Value) -> JuizResult<Box<S>>;



#[repr(C)]
pub struct ContainerStackFactoryImpl<T: 'static> {
    core: ObjectCore,
    manifest: Value,
    constructor: ContainerStackConstructFunction<T>
}

impl<T: 'static> ContainerStackFactoryImpl<T> {

    pub fn new(manifest: Value, constructor: ContainerStackConstructFunction<T>) -> JuizResult<Self> {
        let type_name = obj_get_str(&manifest, "type_name")?;
        Ok(ContainerStackFactoryImpl::<T>{
                core: ObjectCore::create_factory(JuizObjectClass::ContainerFactory("ContainerStackFactoryImpl"), type_name),
                manifest: check_process_factory_manifest(manifest)?,
                constructor
        })
    }

    pub fn create(manifest: Value, constructor: ContainerStackConstructFunction<T>) -> JuizResult<Arc<Mutex<dyn ContainerFactory>>> {
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


impl<T: 'static> JuizObjectCoreHolder for ContainerStackFactoryImpl<T> {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}

impl<T: 'static> JuizObject for ContainerStackFactoryImpl<T> {}

impl<T: 'static> ContainerFactory for ContainerStackFactoryImpl<T> {

    fn create_container(&self, core_worker: &mut CoreWorker, manifest: Value) -> JuizResult<ContainerPtr>{
        log::trace!("ContainerStackFactory::create_container(manifest={}) called", manifest);
        //let parent_id = obj_get_str(&manifest, "parent_container")?.to_owned();
        let parent_manifest = obj_get(&manifest, "parent_container")?;
        //let parent_container = core_worker.container_from_identifier(&parent_id)?;
        let parent_container = core_worker.container_from_manifest(parent_manifest)?;
        Ok(ContainerPtr::new(ContainerImpl::new_with_parent(
            self.apply_default_manifest(manifest.clone())?,
            (self.constructor)(parent_container.clone(), manifest)?,
            parent_container,
        )?))
    }
    
    fn destroy_container(&mut self, c: ContainerPtr) -> JuizResult<Value> {
        log::trace!("ContainerFractoryImpl::destroy_container() called");
        c.lock()?.profile_full()
    }
    
}
