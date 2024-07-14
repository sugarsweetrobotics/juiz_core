
use std::{collections::HashMap, fs, path::PathBuf, sync::{Arc, Mutex}};
use pyo3::{prelude::*, types::{PyDict, PyFloat, PyInt, PyList, PyNone, PyString}};

use crate::{containers::{python_container_process_factory_impl::PythonContainerProcessFactoryImpl, PythonContainerFactoryImpl}, jvalue, processes::python_process_factory_impl::PythonProcessFactoryImpl, ContainerFactory, ContainerProcessFactory, JuizResult, ProcessFactory, Value};

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

    fn get_manifest(&self, working_dir: Option<PathBuf>) -> JuizResult<Value> {
        self.get_manifest_with_name(working_dir, "manifest")
    }

    fn get_manifest_with_name(&self, working_dir: Option<PathBuf>, symbol_name: &str) -> JuizResult<Value> {
        let mut manifest = jvalue!({});
        let fullpath = working_dir.clone().unwrap_or(env!("CARGO_MANIFEST_DIR").into()).join(self.path.clone());
        let _from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
            log::trace!("in get_manifest_with_name(), Python:with_gil called (fullpath={:?}", fullpath.clone());
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
            let manifest_func: Py<PyAny> = module.getattr(symbol_name)?.into();
            let result = manifest_func.call0(py)?;
            let pymanifest = result.extract::<&PyDict>(py)?;
            manifest = pydict_to_value(pymanifest)?;
            //let _func: Py<PyAny> = module.getattr("manifest")?.into();
            Ok(result)
        })?;
        return Ok(manifest);
    }


    pub fn load_container_factory(&self, working_dir: Option<PathBuf>, _symbol_name: &str) -> JuizResult<Arc<Mutex<dyn ContainerFactory>>> {
        log::trace!("PythonPlugin({:?})::load_container_factory() called", self.path);
        self.load_container_factory_with_manifest(working_dir.clone(), self.get_manifest(working_dir)?)
    }
    
    pub fn load_container_factory_with_manifest(&self, working_dir: Option<PathBuf>, manifest: Value) -> JuizResult<Arc<Mutex<dyn ContainerFactory>>> {
        Ok(Arc::new(Mutex::new(PythonContainerFactoryImpl::new(
            manifest,
            working_dir.unwrap_or(env!("CARGO_MANIFEST_DIR").into()).join(self.path.clone())
        )?)))
    }

    pub fn load_container_process_factory(&self, working_dir: Option<PathBuf>, _symbol_name: &str) -> JuizResult<Arc<Mutex<dyn ContainerProcessFactory>>> {
        log::trace!("PythonPlugin({:?})::load_container_factory() called", self.path);
        self.load_container_process_factory_with_manifest(working_dir.clone(), self.get_manifest(working_dir.clone())?)
    }

    pub fn load_container_process_factory_with_manifest(&self, working_dir: Option<PathBuf>, manifest: Value) -> JuizResult<Arc<Mutex<dyn ContainerProcessFactory>>> {
        Ok(Arc::new(Mutex::new(PythonContainerProcessFactoryImpl::new(
            manifest,
            working_dir.unwrap_or(env!("CARGO_MANIFEST_DIR").into()).join(self.path.clone()),
        )?)))
    }

    pub fn load_process_factory(&self, working_dir: Option<PathBuf>, _symbol_name: &str) -> JuizResult<Arc<Mutex<dyn ProcessFactory>>> {
        log::trace!("PythonPlugin({:?})::load_process_factory() called", self.path);
        self.load_process_factory_with_manifest(working_dir.clone(), self.get_manifest(working_dir)?)
    }

    pub fn load_process_factory_with_manifest(&self, working_dir: Option<PathBuf>, manifest: Value) -> JuizResult<Arc<Mutex<dyn ProcessFactory>>> {
        Ok(Arc::new(Mutex::new(PythonProcessFactoryImpl::new(
            manifest,
            working_dir.unwrap_or(env!("CARGO_MANIFEST_DIR").into()).join(self.path.clone()),
        )?)))
    }

    pub fn load_component_profile(&self, working_dir: Option<PathBuf>) -> JuizResult<Value> {
        self.get_manifest_with_name(working_dir, "component_profile")
    }
}