
use std::{collections::HashMap, fs, path::PathBuf, sync::{Arc, Mutex}};
use pyo3::{prelude::*, types::{PyBytes, PyDict, PyFloat, PyInt, PyList, PyNone, PyString, PyTuple}};
use crate::opencv::prelude::*;
use crate::{anyhow::{self, anyhow}, containers::{python_container_process_factory_impl::PythonContainerProcessFactoryImpl, PythonContainerFactoryImpl}, jvalue, processes::python_process_factory_impl::PythonProcessFactoryImpl, Capsule, ContainerFactory, ContainerProcessFactory, JuizResult, ProcessFactory, Value};

pub struct PythonPlugin {
    path: PathBuf,
}

fn pytuple_to_mat(pytuple: &PyTuple) -> JuizResult<Capsule> {
    let shape = pytuple.get_item(0)?.extract::<&PyTuple>()?.into_iter().map(|v|{v.extract::<i32>().unwrap()}).collect::<Vec<i32>>();
    Ok(opencv::core::Mat::new_rows_cols_with_data(
        shape[0] * shape[2],
        shape[1],
        pytuple.get_item(1)?.extract::<&PyBytes>()?.as_bytes()
    )?.reshape(3, shape[0])?.try_clone()?.into())
}

pub fn pyany_to_mat(py: Python, object: &PyAny) -> JuizResult<Capsule> {
    let py_app = r#"
import cv2
def to_tuple(mat):
    return (mat.shape, mat.data.tobytes())
    "#;
    let symbol_name = "to_tuple";
    let module = PyModule::from_code_bound(py, &py_app, "", "").or_else(|e| { Err(anyhow!(e))})?;
    let func: Py<PyAny> = module.getattr(symbol_name)?.into();
    let result = func.call1(py, PyTuple::new_bound(py,[object]))?;
    //log::error!("pyany_to_mat, resultis {result:?}/{:}", result.to_string());
    pytuple_to_mat(result.extract::<&PyTuple>(py)?)
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
    } else if value.is_instance_of::<PyTuple>() {
        pytuple_to_value(value.extract::<&PyTuple>()?)
    } else if value.is_instance_of::<PyDict>() {
        pydict_to_value(value.extract::<&PyDict>()?)
    } else if value.is_instance_of::<PyNone>() {
        Ok(Value::Null)
    } else {
        let pytype = value.get_type();
        println!("pytype: {pytype:?}");
        log::error!("Error for pyany_to_value. Error({pytype:?} is not available)");
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
fn pytuple_to_value(pytuple: &PyTuple) -> PyResult<Value> {
    let mut vec: Vec<Value> = Vec::new();
    for value in pytuple.into_iter() {
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

    pub fn get_manifest(&self, working_dir: Option<PathBuf>, symbol_name: &str) -> JuizResult<Value> {
        self.get_manifest_with_name(working_dir, symbol_name)
    }

    fn get_manifest_with_name(&self, working_dir: Option<PathBuf>, symbol_name: &str) -> JuizResult<Value> {
        let fullpath = working_dir.clone().unwrap_or(env!("CARGO_MANIFEST_DIR").into()).join(self.path.clone());
        match Python::with_gil(|py| -> PyResult<Value> {
            log::trace!("in get_manifest_with_name(), Python:with_gil called (fullpath={:?}", fullpath.clone());
            log::debug!("PythonPlugin uses python version={:?}", py.version_info());
            // サブモジュールのためにルートディレクトリをpathに入れとく。
            let parent = fullpath.parent().unwrap().to_str().unwrap();
            let _ = PyModule::from_code_bound(py, &format!(r#"
import sys
if not "{parent:}" in sys.path:
    sys.path.append("{parent:}")
            "#), "", "");

            let py_app = fs::read_to_string(fullpath.clone())?;
            let module = PyModule::from_code_bound(py, &py_app, "", "")?;
            let manifest_func: Py<PyAny> = module.getattr(symbol_name)?.into();
            Ok(pydict_to_value(manifest_func.call0(py)?.extract::<&PyDict>(py)?)?) // 関数コールしてPyDictを抽出してvalueに変換する
        }) {
            Ok(manifest) => { Ok(manifest) }
            Err(e) => {
                log::error!("get_manifest_with_name() failed. {e:}");
                Err(anyhow::Error::from(e))
            }
        }
    }


    pub fn load_container_factory(&self, working_dir: Option<PathBuf>, symbol_name: &str, container_profile: Value) -> JuizResult<Arc<Mutex<dyn ContainerFactory>>> {
        log::trace!("PythonPlugin({:?})::load_container_factory(symbol_name='{symbol_name}') called", self.path);
        //self.load_container_factory_with_manifest(working_dir.clone(), self.get_manifest(working_dir, symbol_name)?)
        self.load_container_factory_with_manifest(working_dir.clone(), container_profile)
    }
    
    pub fn load_container_factory_with_manifest(&self, working_dir: Option<PathBuf>, manifest: Value) -> JuizResult<Arc<Mutex<dyn ContainerFactory>>> {
        Ok(Arc::new(Mutex::new(PythonContainerFactoryImpl::new(
            manifest,
            working_dir.unwrap_or(env!("CARGO_MANIFEST_DIR").into()).join(self.path.clone())
        )?)))
    }

    pub fn load_container_process_factory(&self, working_dir: Option<PathBuf>, symbol_name: &str, cp_profile: &Value) -> JuizResult<Arc<Mutex<dyn ContainerProcessFactory>>> {
        log::trace!("PythonPlugin({:?})::load_container_factory(symbol_name='{symbol_name}') called", self.path);
        self.load_container_process_factory_with_manifest(working_dir.clone(), cp_profile.clone())
    }

    pub fn load_container_process_factory_with_manifest(&self, working_dir: Option<PathBuf>, manifest: Value) -> JuizResult<Arc<Mutex<dyn ContainerProcessFactory>>> {
        Ok(Arc::new(Mutex::new(PythonContainerProcessFactoryImpl::new(
            manifest,
            working_dir.unwrap_or(env!("CARGO_MANIFEST_DIR").into()).join(self.path.clone()),
        )?)))
    }

    pub fn load_process_factory(&self, working_dir: Option<PathBuf>, symbol_name: &str) -> JuizResult<Arc<Mutex<dyn ProcessFactory>>> {
        log::trace!("PythonPlugin({:?})::load_process_factory(symbol_name='{symbol_name}') called", self.path);
        self.load_process_factory_with_manifest(working_dir.clone(), self.get_manifest(working_dir, "manifest")?, symbol_name)
    }

    pub fn load_process_factory_with_manifest(&self, working_dir: Option<PathBuf>, manifest: Value, symbol_name: &str) -> JuizResult<Arc<Mutex<dyn ProcessFactory>>> {
        Ok(Arc::new(Mutex::new(PythonProcessFactoryImpl::new(
            manifest,
            working_dir.unwrap_or(env!("CARGO_MANIFEST_DIR").into()).join(self.path.clone()),
            symbol_name,
        )?)))
    }

    pub fn load_component_profile(&self, working_dir: Option<PathBuf>) -> JuizResult<Value> {
        self.get_manifest_with_name(working_dir, "component_profile")
    }
}