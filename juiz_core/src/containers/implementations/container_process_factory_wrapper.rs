use std::cell::RefCell;


use crate::prelude::*;
use crate::plugin::{JuizObjectPlugin, Plugin};


#[allow(dead_code)]
pub struct ContainerProcessFactoryWrapper {
    core: ObjectCore,
    container_process_factory: ContainerProcessFactoryPtr,
    container_processes: RefCell<Vec<ProcessPtr>>,
    plugin: JuizObjectPlugin,
}

impl ContainerProcessFactoryWrapper {


    pub fn new(plugin: JuizObjectPlugin, container_process_factory: ContainerProcessFactoryPtr) -> JuizResult<Self> {
        Ok(Self{
            core: ObjectCore::create_factory(JuizObjectClass::ContainerProcessFactory("ContainerProcessFactoryWrapper"), container_process_factory.type_name()),
            plugin,
            container_process_factory,
            container_processes: RefCell::new(vec![])
        })
    }

}

impl JuizObjectCoreHolder for ContainerProcessFactoryWrapper {
    fn core(&self) -> &ObjectCore {
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
            "container_process_factory": self.container_process_factory.lock()?.profile_full()?,
            //container_processes: RefCell<Vec<Arc<Mutex<dyn ContainerProcess>>>>
        }))?.into())
    }
}


impl ContainerProcessFactory for ContainerProcessFactoryWrapper {

    fn create_container_process(&self, container: ContainerPtr, manifest: ProcessManifest) -> JuizResult<ProcessPtr> {
        log::trace!("ContainerProcessFactoryWrapper::create_container_process(manifest={:?}) called", manifest);
        let p = self.container_process_factory.lock()?.create_container_process(container, manifest)?;
        self.container_processes.borrow_mut().push(p.clone());
        Ok(p.clone())
    }
    
    fn destroy_container_process(&mut self, p: ProcessPtr) -> JuizResult<Value> {
        let prof = p.lock()?.profile_full()?;
        let id = obj_get_str(&prof, "identifier")?;
        log::trace!("ContainerProcessFactoryWrapper::destroy_container_process(identifier={}) called", id);
        let index = self.container_processes.borrow().iter().enumerate().find(|r| r.1.lock().unwrap().identifier() == id).unwrap().0;
        self.container_processes.borrow_mut().remove(index);
        let r = self.container_process_factory.lock_mut()?.destroy_container_process(p);
        log::trace!("ContainerProcessFactoryWrapper::destroy_container_process(identifier={}) exit", id);
        r
    }
}

impl Drop for ContainerProcessFactoryWrapper {

    fn drop(&mut self) {
        log::trace!("ContainerProcessFactoryWrapper()::drop() called");
    }
}