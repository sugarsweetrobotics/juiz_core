
use std::{collections::HashMap, rc::Rc};
use libloading::Symbol;
use pyo3::prelude::*;
use serde_json::Map;
use crate::{core::cpp_plugin::CppPlugin, object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore}, processes::{process_impl::ProcessImpl, process_ptr}, utils::check_process_factory_manifest, value::obj_get_str, Capsule, CapsuleMap, CapsulePtr, JuizError, JuizObject, JuizResult, ProcessFactory, ProcessPtr, Value};

pub type CppFunctionType = Symbol<'static, extern "C" fn(*mut CapsuleMap, *mut Capsule) -> i64>;

pub type PythonFunctionType = dyn Fn(CapsuleMap)->JuizResult<Capsule>;
#[repr(C)]
pub struct CppProcessFactoryImpl {
    core: ObjectCore,
    manifest: Value,
    plugin: Rc<CppPlugin>,
    entry_point: unsafe fn(*mut CapsuleMap, *mut Capsule) -> i64,
}

impl CppProcessFactoryImpl {

    pub fn new(plugin: Rc<CppPlugin>) -> JuizResult<Self> {
        let type_name = obj_get_str(plugin.get_manifest(), "type_name")?;
        let symbol_name = "process_factory";
        type SymbolType = libloading::Symbol<'static, unsafe fn() -> unsafe fn(*mut CapsuleMap, *mut Capsule)->i64>;
        let f = unsafe {
            let symbol = plugin.load_symbol::<SymbolType>(symbol_name.as_bytes())?;
            (symbol)()
        };

        Ok(
            CppProcessFactoryImpl{
                core: ObjectCore::create_factory(JuizObjectClass::ProcessFactory("ProcessFactoryImpl"),
                    type_name
                ),
                manifest: check_process_factory_manifest(plugin.get_manifest().clone())?, 
                plugin,
                entry_point: f
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

impl JuizObjectCoreHolder for CppProcessFactoryImpl {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}


impl JuizObject for CppProcessFactoryImpl {
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

impl ProcessFactory for CppProcessFactoryImpl {

    fn create_process(&self, manifest: Value) -> JuizResult<ProcessPtr>{
        log::trace!("CppaProcessFactoryImpl::create_process(manifest={}) called", manifest);
        let entry_point_name = "process_entry_point".to_owned();
        let entry_point = self.entry_point;
        let cppfunc: Box<dyn Fn(CapsuleMap)->JuizResult<Capsule>> = Box::new(move |mut argument: CapsuleMap| -> JuizResult<Capsule> {
            log::trace!("cppfunc (argument={argument:?}) called");
            let mut func_result : Capsule = Capsule::empty();
            unsafe {
                let v = entry_point(&mut argument, &mut func_result);
                if v < 0 {
                    return Err(anyhow::Error::from(JuizError::CppPluginFunctionCallError{function_name:entry_point_name.clone(), return_value:v}));
                } 
            }
            return Ok(func_result);
        });

        let proc = ProcessImpl::clousure_new_with_class_name(
            JuizObjectClass::Process("ProcessImpl"), 
            self.apply_default_manifest(manifest.clone())?, 
            cppfunc,
        )?;
        Ok(process_ptr(proc))
    }    
}
