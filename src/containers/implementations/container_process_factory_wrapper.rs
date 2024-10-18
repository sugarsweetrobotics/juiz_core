use std::{cell::RefCell, sync::{Arc, Mutex}};

use anyhow::Context;

use crate::prelude::*;
use crate::{object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore}, plugin::{JuizObjectPlugin, Plugin}, utils::juiz_lock, value::{obj_get_str, obj_merge}};

use super::container_process_impl::ContainerProcessPtr;

#[allow(dead_code)]
pub struct ContainerProcessFactoryWrapper {
    core: ObjectCore,
    container_process_factory: Arc<Mutex<dyn ContainerProcessFactory>>,
    container_processes: RefCell<Vec<ProcessPtr>>,
    plugin: JuizObjectPlugin,
}

impl ContainerProcessFactoryWrapper {


    pub fn new(plugin: JuizObjectPlugin, container_process_factory: Arc<Mutex<dyn ContainerProcessFactory>>) -> JuizResult<Arc<Mutex<dyn ContainerProcessFactory>>> {
        let cpf = juiz_lock(&container_process_factory)?;
        let type_name = cpf.type_name();
        Ok(Arc::new(Mutex::new(ContainerProcessFactoryWrapper{
            core: ObjectCore::create_factory(JuizObjectClass::ContainerProcessFactory("ContainerProcessFactoryWrapper"), type_name),
            plugin,
            container_process_factory: Arc::clone(&container_process_factory),
            container_processes: RefCell::new(vec![])
        })))
    }

}

impl JuizObjectCoreHolder for ContainerProcessFactoryWrapper {
    fn core(&self) -> &crate::object::ObjectCore {
        &self.core
    }
}

impl JuizObject for ContainerProcessFactoryWrapper {
    
    fn profile_full(&self) -> JuizResult<Value> {
        //let prof = self.rust_plugin.as_ref().and_then(|p|{ Some(p.profile_full().unwrap()) }).or( Some(self.python_plugin.as_ref().unwrap().profile_full().unwrap()) );
        let prof = match &self.plugin {
            JuizObjectPlugin::Rust(p) => p.profile_full().unwrap(),
            JuizObjectPlugin::Python(p) => p.profile_full().unwrap(),
            JuizObjectPlugin::Cpp(p) => p.profile_full().unwrap(),
        };
        
        Ok(obj_merge(self.core.profile_full()?.try_into()?, &jvalue!({
            "plugin": prof,
            "container_process_factory": juiz_lock(&self.container_process_factory)?.profile_full()?,
            //container_processes: RefCell<Vec<Arc<Mutex<dyn ContainerProcess>>>>
        }))?.into())
    }
}


impl ContainerProcessFactory for ContainerProcessFactoryWrapper {

    fn create_container_process(&self, container: ContainerPtr, manifest: Value) -> JuizResult<ProcessPtr> {
        log::trace!("ContainerProcessFactoryWrapper::create_container_process(manifest={}) called", manifest);
        let p = juiz_lock(&self.container_process_factory).with_context(||format!("ContainerProcessFactoryWrapper::create_container_process(manifest:{manifest:}) failed."))?.create_container_process(container, manifest)?;
        self.container_processes.borrow_mut().push(p.clone());
        Ok(p.clone())
    }
    
    fn destroy_container_process(&mut self, p: ProcessPtr) -> JuizResult<Value> {
        let prof = p.lock()?.profile_full()?;
        let id = obj_get_str(&prof, "identifier")?;
        log::trace!("ContainerProcessFactoryWrapper::destroy_container_process(identifier={}) called", id);
        let index = self.container_processes.borrow().iter().enumerate().find(|r| r.1.lock().unwrap().identifier() == id).unwrap().0;
        self.container_processes.borrow_mut().remove(index);
        let r = juiz_lock(&self.container_process_factory)?.destroy_container_process(p);
        log::trace!("ContainerProcessFactoryWrapper::destroy_container_process(identifier={}) exit", id);
        r
    }
}

impl Drop for ContainerProcessFactoryWrapper {

    fn drop(&mut self) {
        log::trace!("ContainerProcessFactoryWrapper()::drop() called");
    }
}