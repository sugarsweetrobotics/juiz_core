
use std::sync::{Mutex, Arc};
use crate::JuizResult;
use crate::utils::check_process_factory_manifest;
use crate::process::Process;
use crate::value::obj_get_str;
use crate::{jvalue, ProcessFactory, process::process_impl::ProcessImpl, JuizError, Value};


#[repr(C)]
pub struct ProcessFactoryImpl {
    manifest: Value,
    function: fn(Value) -> JuizResult<Value>,
}

pub fn create_process_factory(manifest: crate::Value, function: fn(Value) -> JuizResult<Value>) -> JuizResult<Arc<Mutex<dyn ProcessFactory>>> {
    log::trace!("create_process_factory called");
    ProcessFactoryImpl::new(manifest, function)
}

impl ProcessFactoryImpl {

    pub fn new(manifest: crate::Value, function: fn(Value) -> JuizResult<Value>) -> JuizResult<Arc<Mutex<dyn ProcessFactory>>> {
        Ok(Arc::new(Mutex::new(
            ProcessFactoryImpl{
                manifest: check_process_factory_manifest(manifest)?, 
                function
            }
        )))
    }

    fn apply_default_manifest(&self, manifest: Value) -> Result<Value, JuizError> {
        let mut new_manifest = self.manifest.clone();
        for (k, v) in manifest.as_object().unwrap().iter() {
            new_manifest.as_object_mut().unwrap().insert(k.to_owned(), v.clone());
        }
        return Ok(new_manifest);
    }
}

impl ProcessFactory for ProcessFactoryImpl {


    fn type_name(&self) -> &str {
        obj_get_str(&self.manifest, "type_name").unwrap()
//        self.manifest.get("type_name").unwrap().as_str().unwrap()
    }

    fn create_process(&self, manifest: Value) -> JuizResult<Arc<Mutex<dyn Process>>>{
        log::trace!("ProcessFactoryImpl::create_process(manifest={}) called", manifest);
        Ok(Arc::new(Mutex::new(
            ProcessImpl::new(
                self.apply_default_manifest(manifest)?, 
                self.function)?
        )))
    }


    fn profile_full(&self) -> JuizResult<Value> {
        Ok(jvalue!({
            "type_name": self.type_name()
        }))
    }
    
}
