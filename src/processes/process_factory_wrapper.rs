use std::{cell::RefCell, rc::Rc, sync::{Arc, Mutex}};

use anyhow::Context;

use crate::{core::{python_plugin::PythonPlugin, RustPlugin}, jvalue, object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore}, utils::juiz_lock, value::obj_merge, JuizObject, JuizResult, ProcessFactory, ProcessPtr, Value};

#[allow(unused)]
enum Plugins {
    Rust(RustPlugin),
    Python(PythonPlugin)
}

#[allow(dead_code)]
pub struct ProcessFactoryWrapper {
    core: ObjectCore,
    rust_plugin: Option<Rc<RustPlugin>>,
    python_plugin: Option<Rc<PythonPlugin>>,
    process_factory: Arc<Mutex<dyn ProcessFactory>>,
    processes: RefCell<Vec<ProcessPtr>>
}

impl ProcessFactoryWrapper {
    
    pub fn new(plugin: Rc<RustPlugin>, process_factory: Arc<Mutex<dyn ProcessFactory>>) -> JuizResult<Arc<Mutex<dyn ProcessFactory>>> {
        let pf = juiz_lock(&process_factory)?;
        let type_name = pf.type_name();
        Ok(Arc::new(Mutex::new(ProcessFactoryWrapper{
            core: ObjectCore::create_factory(JuizObjectClass::ProcessFactory("ProcessFactoryWrapper"), type_name),
            rust_plugin: Some(plugin), 
            python_plugin: None,
            process_factory: Arc::clone(&process_factory),
            processes: RefCell::new(vec![])
        })))
    }

    pub fn new_python(plugin: Rc<PythonPlugin>, process_factory: Arc<Mutex<dyn ProcessFactory>>) -> JuizResult<Arc<Mutex<dyn ProcessFactory>>> {
        let pf = juiz_lock(&process_factory)?;
        let type_name = pf.type_name();
        Ok(Arc::new(Mutex::new(ProcessFactoryWrapper{
            core: ObjectCore::create_factory(JuizObjectClass::ProcessFactory("ProcessFactoryWrapper"), type_name),
            rust_plugin: None,
            python_plugin: Some(plugin), 
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
        let prof = self.rust_plugin.as_ref().and_then(|p|{ Some(p.profile_full().unwrap()) }).or( Some(self.python_plugin.as_ref().unwrap().profile_full().unwrap()) );
        /*
        let profile = if f {
            self.rust_plugin.unwrap().profile_full()?
        } else {
            self.python_plugin.unwrap().profile_full()?
        };*/
        let v = self.core.profile_full()?;
        Ok(obj_merge(v, &jvalue!({
            "plugin": prof.unwrap(),
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