use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use std::{cell::RefCell, rc::Rc};
use crate::manifest_checker::check_process_factory_manifest;
use crate::process::Process;
use crate::value::*;
use crate::{process_factory::ProcessFactory, process_impl::ProcessImpl, error::JuizError, Value};


pub struct ProcessFactoryImpl {
    manifest: Value,
    function: crate::process::ProcessFunction,
    //child_processes: HashMap<String, Arc<Mutex<dyn Process>>>,
}

impl ProcessFactoryImpl {

    pub fn new(manifest: crate::Value, function: crate::process::ProcessFunction) -> Result<Arc<Mutex<dyn ProcessFactory>> , JuizError> {
        let manifest_updated = check_process_factory_manifest(manifest)?;
        Ok(Arc::new(Mutex::new(ProcessFactoryImpl{manifest: manifest_updated, function, 
            //child_processes: HashMap::new()
        })))
    }
}

impl ProcessFactory for ProcessFactoryImpl {


    fn type_name(&self) -> &str {
        self.manifest.get("type_name").unwrap().as_str().unwrap()
    }


    fn create_process<&str>(&self, name: &str, manifest: Value) -> Result<Arc<Mutex<dyn Process>>, JuizError> {
        
    }

    fn create_process<T>(&self, name: T, manifest: Value) -> Result<Arc<Mutex<dyn Process>> , JuizError>{
        match ProcessImpl::new(name, manifest, self.function) {
            Err(e) => return Err(e),
            Ok(p) => {
                Ok(Arc::new(Mutex::new(p)))
            }
        }
    }
}
