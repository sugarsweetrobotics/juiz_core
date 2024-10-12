use std::{cell::RefCell, sync::{Arc, Mutex}};

use anyhow::Context;

use crate::prelude::*;
use crate::{plugin::JuizObjectPlugin, object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore}, utils::juiz_lock, value::obj_merge};

#[allow(dead_code)]
pub struct ProcessFactoryWrapper {
    core: ObjectCore,
    process_factory: Arc<Mutex<dyn ProcessFactory>>,
    processes: RefCell<Vec<ProcessPtr>>,
    plugin: JuizObjectPlugin,
}

impl ProcessFactoryWrapper {
    

    pub fn new(plugin: JuizObjectPlugin, process_factory: Arc<Mutex<dyn ProcessFactory>>) -> JuizResult<Arc<Mutex<dyn ProcessFactory>>> {
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

    fn profile_full(&self) -> JuizResult<Value> {
        log::trace!("ProcessFactoryWrapper({:})::profile_full() called", self.type_name());
        let prof = self.plugin.profile_full().unwrap();
        let v = self.core.profile_full()?;
        Ok(obj_merge(v, &jvalue!({
            "plugin": prof,
            "process_factory": juiz_lock(&self.process_factory)?.profile_full()?,
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

impl Drop for ProcessFactoryWrapper {

    fn drop(&mut self) {
        log::trace!("ProcecssFactoryWrapper()::drop() called");
    }
}