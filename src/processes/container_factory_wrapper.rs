use std::{sync::{Arc, Mutex}, cell::RefCell};

use anyhow::Context;

use crate::{jvalue, ContainerFactory, core::Plugin, Container, Value, utils::juiz_lock, JuizResult};

#[allow(dead_code)]
pub struct ContainerFactoryWrapper {
    plugin: Plugin,
    type_name: String,
    container_factory: Arc<Mutex<dyn ContainerFactory>>,
    containers: RefCell<Vec<Arc<Mutex<dyn Container>>>>
}

impl ContainerFactoryWrapper {
    
    pub fn new(plugin: Plugin, container_factory: Arc<Mutex<dyn ContainerFactory>>) -> Arc<Mutex<dyn ContainerFactory>> {
        let type_name = container_factory.lock().unwrap().type_name().to_string();
        Arc::new(Mutex::new(ContainerFactoryWrapper{
            plugin, 
            type_name, 
            container_factory,
            containers: RefCell::new(vec![])
        }))
    }
}


impl ContainerFactory for ContainerFactoryWrapper {
    fn type_name(&self) -> &str {
        self.type_name.as_str()
    }

    fn profile_full(&self) -> JuizResult<Value> {
        Ok(jvalue!(
            {
                "plugin": self.plugin.profile_full()?,
                "type_name": self.type_name(),
                "container_factory": juiz_lock(&self.container_factory)?.profile_full()?
            }
        ))
    }

    fn create_container(&self, manifest: Value) -> JuizResult<Arc<Mutex<dyn Container>>> {
        log::trace!("ContainerFactoryWrapper::create_container(manifest={}) called", manifest);
        let p = juiz_lock(&self.container_factory).with_context(||format!("ContainerFactoryWrapper::create_container(manifest:{manifest:}) failed."))?.create_container(manifest)?;
        self.containers.borrow_mut().push(Arc::clone(&p));
        Ok(Arc::clone(&p))
    }
}