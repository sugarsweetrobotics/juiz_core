
use std::sync::{Mutex, Arc};
use super::container_impl::ContainerImpl;
use crate::{jvalue, JuizError, Value, Container, ContainerFactory, JuizResult, utils::check_process_factory_manifest, value::obj_get_str};

use super::container_factory::ContainerConstructFunction;


#[repr(C)]
pub struct ContainerFactoryImpl<T> {
    manifest: Value,
    constructor: ContainerConstructFunction<T>
}

pub fn create_container_factory<S: 'static>(manifest: crate::Value, constructor: ContainerConstructFunction<S>) -> JuizResult<Arc<Mutex<dyn ContainerFactory>>> {
    log::trace!("create_container_factory called");
    ContainerFactoryImpl::new(manifest, constructor)
}

impl<S: 'static> ContainerFactoryImpl<S> {

    pub fn new(manifest: crate::Value, constructor: ContainerConstructFunction<S>) -> JuizResult<Arc<Mutex<dyn ContainerFactory>>> {
        Ok(Arc::new(Mutex::new(
            ContainerFactoryImpl::<S>{
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

impl<T: 'static> ContainerFactory for ContainerFactoryImpl<T> {


    fn profile_full(&self) -> JuizResult<Value> {
        Ok(jvalue!(
            {
                "type_name": self.type_name()
            }
        ))
    }

    fn type_name(&self) -> &str {
        obj_get_str(&self.manifest, "type_name").unwrap()
    }

    fn create_container(&self, manifest: Value) -> JuizResult<Arc<Mutex<dyn Container>>>{
        log::trace!("ContainerFactoryImpl::create_container(manifest={}) called", manifest);
        Ok(ContainerImpl::new(
                self.apply_default_manifest(manifest.clone())?,
                (self.constructor)(manifest)?
            ))
    }
    
}
