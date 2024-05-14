use std::{cell::RefCell, rc::Rc, sync::{Arc, Mutex}};

use anyhow::Context;

use crate::{core::Plugin, jvalue, object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore}, processes::capsule::Capsule, utils::juiz_lock, value::obj_merge, ContainerProcessFactory, ContainerPtr, JuizError, JuizObject, JuizResult, ProcessPtr, Value};

#[allow(dead_code)]
pub struct ContainerProcessFactoryWrapper {
    core: ObjectCore,
    plugin: Rc<Plugin>,
    container_process_factory: Arc<Mutex<dyn ContainerProcessFactory>>,
    container_processes: RefCell<Vec<ProcessPtr>>
}

impl ContainerProcessFactoryWrapper {
    
    pub fn new(plugin: Rc<Plugin>, container_process_factory: Arc<Mutex<dyn ContainerProcessFactory>>) -> JuizResult<Arc<Mutex<dyn ContainerProcessFactory>>> {
        let cpf = juiz_lock(&container_process_factory)?;
        let type_name = cpf.type_name();
        Ok(Arc::new(Mutex::new(ContainerProcessFactoryWrapper{
            core: ObjectCore::create_factory(JuizObjectClass::ContainerProcessFactory("ContainerProcessFactoryWrapper"), 
            type_name),
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
    
    fn profile_full(&self) -> JuizResult<Capsule> {
        Ok(obj_merge(self.core.profile_full()?.try_into()?, &jvalue!({
            "plugin": self.plugin.profile_full()?,
            "container_process_factory": juiz_lock(&self.container_process_factory)?.profile_full()?.as_value().ok_or(anyhow::Error::from(JuizError::CapsuleIsNotValueTypeError{}))?,
            //container_processes: RefCell<Vec<Arc<Mutex<dyn ContainerProcess>>>>
        }))?.into())
    }
}


impl ContainerProcessFactory for ContainerProcessFactoryWrapper {

    fn create_container_process(&self, container: ContainerPtr, manifest: Value) -> JuizResult<ProcessPtr> {
        log::trace!("ContainerProcessFactoryWrapper::create_container_process(manifest={}) called", manifest);
        let p = juiz_lock(&self.container_process_factory).with_context(||format!("ContainerProcessFactoryWrapper::create_container_process(manifest:{manifest:}) failed."))?.create_container_process(container, manifest)?;
        self.container_processes.borrow_mut().push(Arc::clone(&p));
        Ok(Arc::clone(&p))
    }
}