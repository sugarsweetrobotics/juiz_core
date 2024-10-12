
use std::{path::PathBuf, rc::Rc};
use pyo3::{prelude::*, types::PyTuple};
use anyhow::anyhow;
use super::python_plugin::{capsulemap_to_pytuple, get_entry_point, get_python_function_signature, python_process_call};
use crate::prelude::*;
use crate::processes::process_from_clousure_new_with_class_name;
use crate::{
    object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore},
    processes::process_ptr, 
    utils::check_process_factory_manifest, value::obj_get_str};

//pub type PythonFunctionType = dyn Fn(CapsuleMap)->JuizResult<Capsule>;
#[repr(C)]
pub struct PythonProcessFactoryImpl {
    core: ObjectCore,
    manifest: Value,
    fullpath: PathBuf,
    entry_point_signature: Value,
    entry_point: Rc<Py<PyAny>>,
}
impl PythonProcessFactoryImpl {

    pub fn new(manifest: Value, fullpath: PathBuf, symbol_name: &str) -> JuizResult<Self> {
        let type_name = obj_get_str(&manifest, "type_name")?;
        let entry_point = get_entry_point(&fullpath, symbol_name)?;
        Ok(
            PythonProcessFactoryImpl{
                core: ObjectCore::create_factory(JuizObjectClass::ProcessFactory("ProcessFactoryImpl"),
                    type_name
                ),
                fullpath,
                manifest: check_process_factory_manifest(manifest)?, 
                entry_point_signature: get_python_function_signature(&entry_point)?,
                entry_point: Rc::new(entry_point),
            }
        )
    }

    fn apply_default_manifest(&self, manifest: Value) -> Result<Value, JuizError> {
        let mut new_manifest = self.manifest.clone();
        for (k, v) in manifest.as_object().unwrap().iter() {
            new_manifest.as_object_mut().unwrap().insert(k.to_owned(), v.clone());
        }
        return Ok(new_manifest);
    }
}

impl JuizObjectCoreHolder for PythonProcessFactoryImpl {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}


impl JuizObject for PythonProcessFactoryImpl {
}

impl ProcessFactory for PythonProcessFactoryImpl {

    fn create_process(&self, manifest: Value) -> JuizResult<ProcessPtr>{
        log::trace!("PythonProcessFactoryImpl::create_process(manifest={}) called", manifest);
        let entry_point = self.entry_point.clone();
        let signature = self.entry_point_signature.clone();
        let pyfunc: Box<dyn Fn(CapsuleMap)->JuizResult<Capsule>> = Box::new(move |argument: CapsuleMap| -> JuizResult<Capsule> {
            Python::with_gil(|py| {
                python_process_call(py, &entry_point, PyTuple::new_bound(py, capsulemap_to_pytuple(py, &argument, &signature, 0)?))
            }).or_else(|e| { Err(anyhow!(e)) })
        });
        let proc = process_from_clousure_new_with_class_name(
            JuizObjectClass::Process("ProcessImpl"), 
            self.apply_default_manifest(manifest.clone())?, 
            pyfunc,
        )?;
        Ok(process_ptr(proc))
    }    
}
