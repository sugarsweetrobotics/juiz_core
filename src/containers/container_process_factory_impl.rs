use std::sync::{Mutex, Arc};
use super::container_process_impl::ContainerProcessImpl;
use crate::{ContainerProcessFactory, Value, JuizResult, Container, value::obj_get_str, ContainerProcess, JuizObject, object::{JuizObjectCoreHolder, ObjectCore, JuizObjectClass}};

struct ContainerProcessFactoryImpl<T> {
    core: ObjectCore,
    manifest: Value,
    function: fn(&mut Box<T>, Value) -> JuizResult<Value>,
}

impl<T: 'static> ContainerProcessFactoryImpl<T> {
    pub fn new(manifest: crate::Value, function: fn(&mut Box<T>, Value) -> JuizResult<Value>) -> JuizResult<Arc<Mutex<dyn ContainerProcessFactory>>> {
        let type_name = obj_get_str(&manifest, "type_name")?;
        Ok(Arc::new(Mutex::new(
            ContainerProcessFactoryImpl{
                core: ObjectCore::create_factory(JuizObjectClass::ContainerProcessFactory("ContainerProcessFactoryImpl"), 
                type_name),
                function,
                manifest,
            }
        )))
    }

    fn apply_default_manifest(&self, manifest: Value) -> JuizResult<Value> {
        let mut new_manifest = self.manifest.clone();
        for (k, v) in manifest.as_object().unwrap().iter() {
            new_manifest.as_object_mut().unwrap().insert(k.to_owned(), v.clone());
        }
        return Ok(new_manifest);
    }
}

pub fn create_container_process_factory<T: 'static>(manifest: crate::Value, function: fn(&mut Box<T>, Value) -> JuizResult<Value>) -> JuizResult<Arc<Mutex<dyn ContainerProcessFactory>>> {
    log::trace!("create_container_process_factory({}) called", manifest);
    ContainerProcessFactoryImpl::<T>::new(manifest, function)
}

impl<T: 'static> JuizObjectCoreHolder for ContainerProcessFactoryImpl<T> {
    fn core(&self) -> &crate::object::ObjectCore {
        &self.core
    }
}

impl<T: 'static> JuizObject for ContainerProcessFactoryImpl<T> {}

impl<T: 'static> ContainerProcessFactory for ContainerProcessFactoryImpl<T> {
    fn create_container_process(&self, container: Arc<Mutex<dyn Container>>, manifest: crate::Value) -> JuizResult<Arc<Mutex<dyn ContainerProcess>>> {
        log::trace!("ContainerProcessFactoryImpl::create_container_process(container, manifest={}) called", manifest);
        Ok(Arc::new(Mutex::new(
            ContainerProcessImpl::new(
                self.apply_default_manifest(manifest)?, 
                Arc::clone(&container), 
                self.function.clone())?
        )))
    }
}