use std::{sync::{Arc, Mutex}, cell::RefCell};

use anyhow::Context;

use crate::{jvalue, ProcessFactory, core::Plugin, Process, Value, utils::juiz_lock, JuizResult, JuizObject, Identifier, identifier::{identifier_from_manifest, identifier_new}, object::{ObjectCore, JuizObjectCoreHolder, JuizObjectClass}, value::obj_merge};

#[allow(dead_code)]
pub struct ProcessFactoryWrapper {
    core: ObjectCore,
    plugin: Plugin,
    process_factory: Arc<Mutex<dyn ProcessFactory>>,
    processes: RefCell<Vec<Arc<Mutex<dyn Process>>>>
}

impl ProcessFactoryWrapper {
    
    pub fn new(plugin: Plugin, process_factory: Arc<Mutex<dyn ProcessFactory>>) -> JuizResult<Arc<Mutex<dyn ProcessFactory>>> {
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
        let v = self.core.profile_full()?;
        obj_merge(v, &jvalue!({
            "plugin": self.plugin.profile_full()?,
            "process_factory": juiz_lock(&self.process_factory)?.profile_full()?
        }))
    }

}


impl ProcessFactory for ProcessFactoryWrapper {

    fn create_process(&self, manifest: Value) -> JuizResult<Arc<Mutex<dyn Process>>> {
        log::trace!("ProcessFactoryWrapper::create_process(manifest={}) called", manifest);
        let p = juiz_lock(&self.process_factory).with_context(||format!("ProcessFactoryWrapper::create_process(manifest:{manifest:}) failed."))?.create_process(manifest)?;
        self.processes.borrow_mut().push(Arc::clone(&p));
        Ok(Arc::clone(&p))
    }


    
}