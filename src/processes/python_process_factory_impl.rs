
use std::{collections::HashMap, fs, path::PathBuf, rc::Rc};
use pyo3::{prelude::*, types::PyTuple};
use serde_json::Map;

use crate::prelude::*;
use crate::{
    plugin::pyany_to_value, 
    object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore},
    processes::{process_impl::ProcessImpl, process_ptr}, 
    utils::check_process_factory_manifest, value::obj_get_str, JuizObject};

pub type PythonFunctionType = dyn Fn(CapsuleMap)->JuizResult<Capsule>;
#[repr(C)]
pub struct PythonProcessFactoryImpl {
    core: ObjectCore,
    manifest: Value,
    fullpath: PathBuf,
    entry_point: Rc<Py<PyAny>>,
}
impl PythonProcessFactoryImpl {

    pub fn new(manifest: crate::Value, fullpath: PathBuf, symbol_name: &str) -> JuizResult<Self> {
        let type_name = obj_get_str(&manifest, "type_name")?;
        //let symbol_name = "process_factory";
        let fp = fullpath.clone();
        let entry_point = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
            let py_app = fs::read_to_string(fp).unwrap();
            let module = PyModule::from_code_bound(py, &py_app.to_string(), "", "")?;
            let app_func: Py<PyAny> = module.getattr(symbol_name)?.into();
            let result = app_func.call0(py)?;
            Ok(result)
        })?;

        Ok(
            PythonProcessFactoryImpl{
                core: ObjectCore::create_factory(JuizObjectClass::ProcessFactory("ProcessFactoryImpl"),
                    type_name
                ),
                fullpath,
                manifest: check_process_factory_manifest(manifest)?, 
                entry_point: Rc::new(entry_point),
                //function
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

fn valuearray_to_pyany(py: Python, arr: &Vec<Value>) -> Py<PyAny> {
    arr.iter().map(|v| { value_to_pyany(py, v) }).collect::<Vec<Py<PyAny>>>().into_py(py)
}

fn valueobj_to_pyany(py: Python, map: &Map<String, Value>) -> Py<PyAny> {
    map.iter().map(|(k, v)| { (k.clone(), value_to_pyany(py, v)) }).collect::<HashMap<String, Py<PyAny>>>().into_py(py)
}

fn value_to_pyany(py: Python, value: &Value) -> Py<PyAny> {
    if value.is_i64() {
        return (value.as_i64().unwrap()).into_py(py);
    } else if value.is_f64() {
        return (value.as_f64().unwrap()).into_py(py);
    } else if value.is_boolean() {
        return (value.as_bool().unwrap()).into_py(py);
    } else if value.is_string() {
        return (value.as_str().unwrap()).into_py(py);
    } else if value.is_null() {
        return (value.as_null().unwrap()).into_py(py);
    } else if value.is_u64() { 
        return (value.as_u64().unwrap()).into_py(py);
    } else if value.is_array() { 
        return valuearray_to_pyany(py, value.as_array().unwrap());
    } else if value.is_object() {
        return valueobj_to_pyany(py, value.as_object().unwrap());
    }
    todo!()
}

fn capsuleptr_to_pyany(py: Python, value: &CapsulePtr) -> Py<PyAny> {
    if value.is_value().unwrap() {
        return value.lock_as_value(|v| {
            value_to_pyany(py, v)
        }).unwrap();
    }
    todo!()
}

pub fn capsulemap_to_pytuple<'a>(py: Python, value: &'a CapsuleMap) -> Vec<Py<PyAny>> {
    value.iter().map(|(_k, v)| { 
        capsuleptr_to_pyany(py, v)
    } ).collect::<Vec<Py<PyAny>>>()
}

pub fn value_to_pytuple<'a>(py: Python, value: &'a Value) -> Vec<Py<PyAny>> {
    vec!(value_to_pyany(py, value))
}

impl ProcessFactory for PythonProcessFactoryImpl {

    fn create_process(&self, manifest: Value) -> JuizResult<ProcessPtr>{
        log::trace!("PythonProcessFactoryImpl::create_process(manifest={}) called", manifest);
        let entry_point = self.entry_point.clone();
        let pyfunc: Box<dyn Fn(CapsuleMap)->JuizResult<Capsule>> = Box::new(move |argument: CapsuleMap| -> JuizResult<Capsule> {
            Python::with_gil(|py| {
                match entry_point.call1(py, PyTuple::new_bound(py, capsulemap_to_pytuple(py, &argument))) {
                    Ok(result) => {
                        Ok(pyany_to_value(result.extract::<&PyAny>(py)?)?.into())
                    }
                    Err(e) => {
                        log::error!("Error calling python call. {e:}");
                        Err(anyhow::Error::from(e))
                    }
                }
            })
        });


        let proc = ProcessImpl::clousure_new_with_class_name(
            JuizObjectClass::Process("ProcessImpl"), 
            self.apply_default_manifest(manifest.clone())?, 
            pyfunc,
        )?;
        Ok(process_ptr(proc))
    }    
}
