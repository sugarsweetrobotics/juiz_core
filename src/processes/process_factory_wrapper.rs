use std::{cell::RefCell, rc::Rc, sync::{Arc, Mutex}};

use anyhow::Context;

use crate::{core::Plugin, jvalue, object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore}, utils::juiz_lock, value::obj_merge, JuizError, JuizObject, JuizResult, ProcessFactory, ProcessPtr, Value};

use super::capsule::Capsule;

#[allow(dead_code)]
pub struct ProcessFactoryWrapper {
    core: ObjectCore,
    plugin: Rc<Plugin>,
    process_factory: Arc<Mutex<dyn ProcessFactory>>,
    processes: RefCell<Vec<ProcessPtr>>
}

impl ProcessFactoryWrapper {
    
    pub fn new(plugin: Rc<Plugin>, process_factory: Arc<Mutex<dyn ProcessFactory>>) -> JuizResult<Arc<Mutex<dyn ProcessFactory>>> {
        let pf = juiz_lock(&process_factory)?;
        let type_name = pf.type_name();
        Ok(Arc::new(Mutex::new(ProcessFactoryWrapper{
            core: ObjectCore::create_factory(JuizObjectClass::ProcessFactory("ProcessFactoryWrapper"), type_name),
            plugin, 
            process_factory: Arc::clone(&process_factory),
            processes: RefCell::new(vec![])
        })))
    }
}

impl JuizObjectCoreHolder for ProcessFactoryWrapper {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}

impl JuizObject for ProcessFactoryWrapper {

    fn profile_full(&self) -> JuizResult<Capsule> {
        let v = self.core.profile_full()?;
        Ok(obj_merge(v, &jvalue!({
            "plugin": self.plugin.profile_full()?,
            "process_factory": juiz_lock(&self.process_factory)?.profile_full()?.as_value().ok_or(anyhow::Error::from(JuizError::CapsuleIsNotValueTypeError{}))?
        }))?.into())
    }

}


impl ProcessFactory for ProcessFactoryWrapper {

    fn create_process(&self, manifest: Value) -> JuizResult<ProcessPtr> {
        log::trace!("ProcessFactoryWrapper::create_process(manifest={}) called", manifest);
        let p = juiz_lock(&self.process_factory).with_context(||format!("ProcessFactoryWrapper::create_process(manifest:{manifest:}) failed."))?.create_process(manifest)?;
        self.processes.borrow_mut().push(Arc::clone(&p));
        Ok(Arc::clone(&p))
    }


    
}