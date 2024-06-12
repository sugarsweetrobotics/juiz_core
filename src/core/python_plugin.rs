
use std::{collections::HashMap, fs, path::PathBuf, sync::{Arc, Mutex}};
use pyo3::{exceptions::PyTypeError, prelude::*, types::{PyDict, PyFloat, PyInt, PyList, PyNone, PyString}};

use crate::{jvalue, processes::{process_factory_impl::ProcessFactoryImpl, process_impl::FunctionType, python_process_factory_impl::PythonProcessFactoryImpl}, value::obj_get_str, Capsule, CapsuleMap, CapsulePtr, JuizResult, ProcessFactory, Value};

pub struct PythonPlugin {
    path: PathBuf,
}


pub fn pyany_to_value(value: &PyAny) -> PyResult<Value> {
    if value.is_instance_of::<PyString>() {
        Ok(Value::from(value.extract::<String>()?))
    } else if value.is_instance_of::<PyFloat>() {
        Ok(Value::from(value.extract::<f64>()?))
    } else if value.is_instance_of::<PyInt>() {
        Ok(Value::from(value.extract::<i64>()?))
    } else if value.is_instance_of::<PyList>() {
        pylist_to_value(value.extract::<&PyList>()?)
    } else if value.is_instance_of::<PyDict>() {
        pydict_to_value(value.extract::<&PyDict>()?)
    } else if value.is_instance_of::<PyNone>() {
        Ok(Value::Null)
    } else {
        todo!()
    }
}

fn pylist_to_value(pylist: &PyList) -> PyResult<Value> {
    let mut vec: Vec<Value> = Vec::new();
    for value in pylist.into_iter() {
        vec.push(pyany_to_value(value)?);
    }
    Ok(vec.into())
}

pub fn pydict_to_value(pydict: &PyDict) -> PyResult<Value> {
    let mut map: HashMap<String, Value> = HashMap::new();
    for (key, value) in pydict.into_iter() {
        map.insert(key.extract::<String>()?, pyany_to_value(value)?);
    }
    Ok(jvalue!(map))
}

/*
pub type Symbol<'lib, T> = libloading::Symbol<'lib, libloading::Symbol<'lib, T>>;
*/
impl PythonPlugin {

    pub fn profile_full(&self) -> JuizResult<Value> {
        Ok(jvalue!({
            "path": self.path,
        }))
    }

    pub fn load(path: PathBuf) -> JuizResult<PythonPlugin> {
        log::trace!("PythonPlugin::load({:?}) called", path);
        Ok(PythonPlugin{path})
    }

    

    pub fn load_process_factory(&self, working_dir: Option<PathBuf>, symbol_name: &str) -> JuizResult<Arc<Mutex<dyn ProcessFactory>>> {
        log::trace!("PythonPlugin({:?})::load_process_factory() called", self.path);
        
        let mut manifest = jvalue!({});
        let fullpath = working_dir.clone().unwrap_or(env!("CARGO_MANIFEST_DIR").into()).join(self.path.clone());
        let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
            log::trace!("Python:with_gil called (fullpath={:?}", fullpath.clone());
            let py_app = fs::read_to_string(fullpath.clone())?;
            let parent = fullpath.parent().unwrap().to_str().unwrap();
            let _ = PyModule::from_code_bound(py, &format!(r#"
import sys
if not "{parent:}" in sys.path:
    sys.path.append("{parent:}")
            "#), "", "");
            let result_module = PyModule::from_code_bound(py, &py_app, "", "");
            // println!("result_module: {:?}", result_module);
            let module = result_module?;
            let manifest_func: Py<PyAny> = module.getattr("manifest")?.into();
            let result = manifest_func.call0(py)?;
            let pymanifest = result.clone().extract::<&PyDict>(py)?;
            manifest = pydict_to_value(pymanifest)?;

            

            let _func: Py<PyAny> = module.getattr("manifest")?.into();
            Ok(result)
        });

        let pyfunc: Box<dyn Fn(CapsuleMap)->JuizResult<Capsule>> = Box::new(|argument: CapsuleMap| -> JuizResult<Capsule> {
            let mut func_result : Capsule = Capsule::empty();
            let pyobj = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let full_path = argument.get_param("full_path").unwrap();
                let type_name = argument.get_param("type_name").unwrap();
                let py_app = fs::read_to_string(full_path).unwrap();
                let module = PyModule::from_code_bound(py, &py_app.to_string(), "", "")?;
                let app_func: Py<PyAny> = module.getattr(type_name.as_str())?.into();
                let result = app_func.call0(py)?;
                let pymanifest = result.clone().extract::<&PyDict>(py)?;
                func_result = pydict_to_value(pymanifest)?.into();
                Ok(result)
            });
            println!("func_result: {:?}", func_result);
            return Ok(func_result);
        });
        return Ok(Arc::new(Mutex::new(PythonProcessFactoryImpl::new(
            manifest.clone(),
            fullpath,
            Box::new(pyfunc)
        )?)));
    
    }
}