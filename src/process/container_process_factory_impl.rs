use std::sync::{Mutex, Arc};

use crate::{jvalue, ContainerProcessFactory, Value, JuizResult, utils::check_process_factory_manifest, Container, value::obj_get_str, process::container_process_impl::ContainerProcessImpl, ContainerProcess};




struct ContainerProcessFactoryImpl<T> {
    manifest: Value,
    function: fn(&mut Box<T>, Value) -> JuizResult<Value>,
}

impl<T: 'static> ContainerProcessFactoryImpl<T> {
    pub fn new(manifest: crate::Value, function: fn(&mut Box<T>, Value) -> JuizResult<Value>) -> JuizResult<Arc<Mutex<dyn ContainerProcessFactory>>> {
        Ok(Arc::new(Mutex::new(
            ContainerProcessFactoryImpl{
                manifest: check_process_factory_manifest(manifest)?, 
                function
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


impl<T: 'static> ContainerProcessFactory for ContainerProcessFactoryImpl<T> {
    fn type_name(&self) -> &str {
        obj_get_str(&self.manifest, "type_name").unwrap()
    }

    fn profile_full(&self) -> JuizResult<Value> {
        Ok(jvalue!({
            "type_name": self.type_name()
        }))
    }

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