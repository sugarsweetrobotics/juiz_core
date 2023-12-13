use std::{sync::{Arc, Mutex}, cell::RefCell};

use anyhow::Context;

use crate::{jvalue, ContainerFactory, core::Plugin, Container, Value, utils::juiz_lock, JuizResult, JuizObject, object::{ObjectCore, JuizObjectClass, JuizObjectCoreHolder}, value::obj_merge};

#[allow(dead_code)]
pub struct ContainerFactoryWrapper {
    core: ObjectCore,
    plugin: Plugin,
    container_factory: Arc<Mutex<dyn ContainerFactory>>,
    containers: RefCell<Vec<Arc<Mutex<dyn Container>>>>
}

impl ContainerFactoryWrapper {
    
    pub fn new(plugin: Plugin, container_factory: Arc<Mutex<dyn ContainerFactory>>) -> JuizResult<Arc<Mutex<dyn ContainerFactory>>> {
        let cf = juiz_lock(&container_factory)?;
        let type_name = cf.type_name();
        Ok(Arc::new(Mutex::new(ContainerFactoryWrapper{
            core: ObjectCore::create_factory(JuizObjectClass::ContainerFactory("ContainerFactoryWrapper"), type_name),
            plugin, 
            container_factory: Arc::clone(&container_factory),
            containers: RefCell::new(vec![])
        })))
    }
}

impl JuizObjectCoreHolder for ContainerFactoryWrapper {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}

impl JuizObject for ContainerFactoryWrapper {

    fn profile_full(&self) -> JuizResult<Value> {
        obj_merge(self.core.profile_full()?, &jvalue!(
            {
                "plugin": self.plugin.profile_full()?,
                "container_factory": juiz_lock(&self.container_factory)?.profile_full()?
            }
        ))
    }
}

impl ContainerFactory for ContainerFactoryWrapper {
    
    fn create_container(&self, manifest: Value) -> JuizResult<Arc<Mutex<dyn Container>>> {
        log::trace!("ContainerFactoryWrapper::create_container(manifest={}) called", manifest);
        let p = juiz_lock(&self.container_factory).with_context(||format!("ContainerFactoryWrapper::create_container(manifest:{manifest:}) failed."))?.create_container(manifest)?;
        self.containers.borrow_mut().push(Arc::clone(&p));
        Ok(Arc::clone(&p))
    }
}