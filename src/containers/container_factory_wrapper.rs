use std::{cell::RefCell, rc::Rc, sync::{Arc, Mutex}};

use anyhow::Context;

use crate::{core::Plugin, jvalue, object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore}, utils::juiz_lock, value::obj_merge, ContainerPtr, ContainerFactory, JuizObject, JuizResult, Value};

#[allow(dead_code)]
pub struct ContainerFactoryWrapper {
    core: ObjectCore,
    plugin: Rc<Plugin>,
    container_factory: Arc<Mutex<dyn ContainerFactory>>,
    containers: RefCell<Vec<ContainerPtr>>
}

impl ContainerFactoryWrapper {
    
    pub fn new(plugin: Rc<Plugin>, container_factory: Arc<Mutex<dyn ContainerFactory>>) -> JuizResult<Arc<Mutex<dyn ContainerFactory>>> {
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
        Ok(obj_merge(self.core.profile_full()?, &jvalue!(
            {
                "plugin": self.plugin.profile_full()?,
                "container_factory": juiz_lock(&self.container_factory)?.profile_full()?,
            }
        ))?.into())
    }
}

impl ContainerFactory for ContainerFactoryWrapper {
    
    fn create_container(&self, manifest: Value) -> JuizResult<ContainerPtr> {
        log::trace!("ContainerFactoryWrapper::create_container(manifest={}) called", manifest);
        let p = juiz_lock(&self.container_factory).with_context(||format!("ContainerFactoryWrapper::create_container(manifest:{manifest:}) failed."))?.create_container(manifest)?;
        self.containers.borrow_mut().push(Arc::clone(&p));
        Ok(Arc::clone(&p))
    }
}