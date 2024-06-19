use std::{cell::RefCell, rc::Rc, sync::{Arc, Mutex}};

use anyhow::Context;

use crate::{core::{python_plugin::PythonPlugin, RustPlugin}, jvalue, object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore}, utils::juiz_lock, value::obj_merge, ContainerProcessFactory, ContainerPtr, JuizObject, JuizResult, ProcessPtr, Value};

#[allow(dead_code)]
pub struct ContainerProcessFactoryWrapper {
    core: ObjectCore,
    rust_plugin: Option<Rc<RustPlugin>>,
    python_plugin: Option<Rc<PythonPlugin>>,
    container_process_factory: Arc<Mutex<dyn ContainerProcessFactory>>,
    container_processes: RefCell<Vec<ProcessPtr>>
}

impl ContainerProcessFactoryWrapper {
    
    pub fn new(plugin: Rc<RustPlugin>, container_process_factory: Arc<Mutex<dyn ContainerProcessFactory>>) -> JuizResult<Arc<Mutex<dyn ContainerProcessFactory>>> {
        let cpf = juiz_lock(&container_process_factory)?;
        let type_name = cpf.type_name();
        Ok(Arc::new(Mutex::new(ContainerProcessFactoryWrapper{
            core: ObjectCore::create_factory(JuizObjectClass::ContainerProcessFactory("ContainerProcessFactoryWrapper"), type_name),
            rust_plugin: Some(plugin),
            python_plugin: None, 
            container_process_factory: Arc::clone(&container_process_factory),
            container_processes: RefCell::new(vec![])
        })))
    }

    pub fn new_python(plugin: Rc<PythonPlugin>, container_process_factory: Arc<Mutex<dyn ContainerProcessFactory>>) -> JuizResult<Arc<Mutex<dyn ContainerProcessFactory>>> {
        let pf = juiz_lock(&container_process_factory)?;
        let type_name = pf.type_name();
        Ok(Arc::new(Mutex::new(ContainerProcessFactoryWrapper{
            core: ObjectCore::create_factory(JuizObjectClass::ContainerProcessFactory("ContainerProcessFactoryWrapper"), type_name),
            rust_plugin: None,
            python_plugin: Some(plugin), 
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
        let prof = self.rust_plugin.as_ref().and_then(|p|{ Some(p.profile_full().unwrap()) }).or( Some(self.python_plugin.as_ref().unwrap().profile_full().unwrap()) );
        Ok(obj_merge(self.core.profile_full()?.try_into()?, &jvalue!({
            "plugin": prof.unwrap(),
            "container_process_factory": juiz_lock(&self.container_process_factory)?.profile_full()?,
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