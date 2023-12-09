use std::{sync::{Arc, Mutex}, cell::RefCell};

use anyhow::Context;

use crate::{jvalue, ProcessFactory, core::Plugin, Process, Value, utils::juiz_lock, JuizResult};

#[allow(dead_code)]
pub struct ProcessFactoryWrapper {
    plugin: Plugin,
    type_name: String,
    process_factory: Arc<Mutex<dyn ProcessFactory>>,
    processes: RefCell<Vec<Arc<Mutex<dyn Process>>>>
}

impl ProcessFactoryWrapper {
    
    pub fn new(plugin: Plugin, process_factory: Arc<Mutex<dyn ProcessFactory>>) -> Arc<Mutex<dyn ProcessFactory>> {
        let type_name = process_factory.lock().unwrap().type_name().to_string();
        Arc::new(Mutex::new(ProcessFactoryWrapper{
            plugin, 
            type_name, 
            process_factory,
            processes: RefCell::new(vec![])
        }))
    }
}


impl ProcessFactory for ProcessFactoryWrapper {
    fn type_name(&self) -> &str {
        self.type_name.as_str()
    }

    fn create_process(&self, manifest: Value) -> JuizResult<Arc<Mutex<dyn Process>>> {
        log::trace!("ProcessFactoryWrapper::create_process(manifest={}) called", manifest);
        let p = juiz_lock(&self.process_factory).with_context(||format!("ProcessFactoryWrapper::create_process(manifest:{manifest:}) failed."))?.create_process(manifest)?;
        self.processes.borrow_mut().push(Arc::clone(&p));
        Ok(Arc::clone(&p))
    }


    fn profile_full(&self) -> JuizResult<Value> {
        Ok(jvalue!({
        "plugin": self.plugin.profile_full()?,
        "type_name": self.type_name(),
        "process_factory": juiz_lock(&self.process_factory)?.profile_full()?
        }))
    }
}