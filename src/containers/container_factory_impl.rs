
use std::sync::{Mutex, Arc};
use anyhow::Context;

use super::container_impl::ContainerImpl;
use crate::{JuizError, Value, ContainerPtr, ContainerFactory, JuizResult, utils::check_process_factory_manifest, value::obj_get_str, JuizObject, object::{ObjectCore, JuizObjectClass, JuizObjectCoreHolder}};

use super::container_factory::ContainerConstructFunction;


#[repr(C)]
pub struct ContainerFactoryImpl<T> {
    core: ObjectCore,
    manifest: Value,
    constructor: ContainerConstructFunction<T>
}

pub fn create_container_factory<S: 'static>(manifest: crate::Value, constructor: ContainerConstructFunction<S>) -> JuizResult<Arc<Mutex<dyn ContainerFactory>>> {
    log::trace!("create_container_factory called");
    ContainerFactoryImpl::new(manifest, constructor).context("create_container_factory()")
}

impl<S: 'static> ContainerFactoryImpl<S> {

    pub fn new(manifest: crate::Value, constructor: ContainerConstructFunction<S>) -> JuizResult<Arc<Mutex<dyn ContainerFactory>>> {
        let type_name = obj_get_str(&manifest, "type_name")?;
        Ok(Arc::new(Mutex::new(
            ContainerFactoryImpl::<S>{
                core: ObjectCore::create_factory(JuizObjectClass::ContainerFactory("ContainerFactoryImpl"), type_name),
                manifest: check_process_factory_manifest(manifest)?,
                constructor
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


impl<T: 'static> JuizObjectCoreHolder for ContainerFactoryImpl<T> {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}

impl<T: 'static> JuizObject for ContainerFactoryImpl<T> {}

impl<T: 'static> ContainerFactory for ContainerFactoryImpl<T> {

    fn create_container(&self, manifest: Value) -> JuizResult<ContainerPtr>{
        log::trace!("ContainerFactoryImpl::create_container(manifest={}) called", manifest);
        Ok(ContainerImpl::new(
                self.apply_default_manifest(manifest.clone())?,
                (self.constructor)(manifest)?
            )?)
    }
    
}
