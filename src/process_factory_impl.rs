use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};
use crate::process::Process;
use crate::value::*;
use crate::{process_factory::ProcessFactory, process_impl::ProcessImpl, error::JuizError, Value};


pub struct ProcessFactoryImpl {
    manifest: Value,
    function: crate::process::ProcessFunction,
    child_processes: HashMap<String, Rc<RefCell<dyn Process>>>,
}

impl ProcessFactoryImpl {

    pub fn new(manifest: crate::Value, function: crate::process::ProcessFunction) -> Result<std::rc::Rc<std::cell::RefCell<dyn ProcessFactory>> , JuizError> {
        Ok(Rc::new(RefCell::new(ProcessFactoryImpl{manifest, function, child_processes: HashMap::new()})))
    }
}

impl ProcessFactory for ProcessFactoryImpl {

    fn create_process(&mut self, name: String) -> Result<std::rc::Rc<std::cell::RefCell<dyn Process>> , JuizError>{
        let mut manifest = self.manifest.clone();
        manifest["name"] = jvalue!(name);
        match ProcessImpl::new(manifest, self.function) {
            Err(e) => return Err(e),
            Ok(p) => {
                let rp: Rc<RefCell<dyn Process>> = Rc::new(RefCell::new(p));
                self.child_processes.insert(name.clone(), Rc::clone(&rp));
                return Ok(rp)
            }
        }
    }
}
