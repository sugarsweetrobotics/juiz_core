use std::{cell::RefCell, sync::{Arc, Mutex}};

use anyhow::Context;
use crate::{containers::container_lock, prelude::*, value::obj_get_str};
use crate::{plugin::{JuizObjectPlugin, Plugin}, object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore}, utils::juiz_lock, value::obj_merge, ContainerFactory, ContainerPtr, JuizObject, JuizResult, Value};

#[allow(dead_code)]
pub struct ContainerFactoryWrapper {
    core: ObjectCore,
    container_factory: Arc<Mutex<dyn ContainerFactory>>,
    containers: RefCell<Vec<ContainerPtr>>,
    plugin: JuizObjectPlugin,
}

impl ContainerFactoryWrapper {
    
    pub fn new(plugin: JuizObjectPlugin, container_factory: Arc<Mutex<dyn ContainerFactory>>) -> JuizResult<Arc<Mutex<dyn ContainerFactory>>> {
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
        let prof = match &self.plugin {
            JuizObjectPlugin::Rust(p) => p.profile_full().unwrap(),
            JuizObjectPlugin::Python(p) => p.profile_full().unwrap(),
            JuizObjectPlugin::Cpp(p) => p.profile_full().unwrap(),
        };
        //let prof = self.rust_plugin.as_ref().and_then(|p|{ Some(p.profile_full().unwrap()) }).or_else(|| { self.python_plugin.as_ref().and_then( |p| {Some(p.profile_full().unwrap())}).or( Some(self.cpp_plugin.as_ref().unwrap().profile_full().unwrap())) });
        Ok(obj_merge(self.core.profile_full()?, &jvalue!(
            {
                "plugin": prof,
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


    fn destroy_container(&mut self, c: ContainerPtr) -> JuizResult<Value> {
        let prof = container_lock(&c)?.profile_full()?;
        let id = obj_get_str(&prof, "identifier")?;
        log::trace!("ContainerFactoryWrapper::destroy_container(manifest={}) called", prof);
        let index = self.containers.borrow().iter().enumerate().find(|rc| container_lock(&rc.1).unwrap().identifier() == id).unwrap().0;
        self.containers.borrow_mut().remove(index);
        juiz_lock(&self.container_factory)?.destroy_container(c)
    }
}

impl Drop for ContainerFactoryWrapper {

    fn drop(&mut self) {
        log::trace!("ContainerFactoryWrapper()::drop() called");
    }
}