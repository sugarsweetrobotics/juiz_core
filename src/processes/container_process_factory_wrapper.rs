use std::{sync::{Arc, Mutex}, cell::RefCell};

use anyhow::Context;

use crate::{jvalue, core::Plugin, Value, utils::juiz_lock, JuizResult, ContainerProcessFactory, ContainerProcess};

#[allow(dead_code)]
pub struct ContainerProcessFactoryWrapper {
    class_name: String ,
    plugin: Plugin,
    type_name: String,
    container_process_factory: Arc<Mutex<dyn ContainerProcessFactory>>,
    container_processes: RefCell<Vec<Arc<Mutex<dyn ContainerProcess>>>>
}

impl ContainerProcessFactoryWrapper {
    
    pub fn new(plugin: Plugin, container_process_factory: Arc<Mutex<dyn ContainerProcessFactory>>) -> Arc<Mutex<dyn ContainerProcessFactory>> {
        let type_name = container_process_factory.lock().unwrap().type_name().to_string();
        Arc::new(Mutex::new(ContainerProcessFactoryWrapper{
            class_name: "ContainerProcessFactoryWrapper".to_string(),
            plugin, 
            type_name, 
            container_process_factory,
            container_processes: RefCell::new(vec![])
        }))
    }
}


impl ContainerProcessFactory for ContainerProcessFactoryWrapper {
    fn type_name(&self) -> &str {
        self.type_name.as_str()
    }

    fn profile_full(&self) -> JuizResult<Value> {
        Ok(jvalue!({
            "class_name": self.class_name,
            "plugin": self.plugin.profile_full()?,
            "type_name": self.type_name(),
            "container_process_factory": juiz_lock(&self.container_process_factory)?.profile_full()?,
            //container_processes: RefCell<Vec<Arc<Mutex<dyn ContainerProcess>>>>
        }))
    }

    fn create_container_process(&self, container: Arc<Mutex<dyn crate::Container>>, manifest: Value) -> JuizResult<Arc<Mutex<dyn ContainerProcess>>> {
        log::trace!("ContainerProcessFactoryWrapper::create_container_process(manifest={}) called", manifest);
        let p = juiz_lock(&self.container_process_factory).with_context(||format!("ContainerProcessFactoryWrapper::create_container_process(manifest:{manifest:}) failed."))?.create_container_process(container, manifest)?;
        self.container_processes.borrow_mut().push(Arc::clone(&p));
        Ok(Arc::clone(&p))
    }
}