
use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};
use pyo3::{prelude::*, types::{PyDict, PyFloat, PyFunction, PyInt, PyList, PyNone, PySet, PyString, PyTuple}};
use serde_json::Map;

use crate::{containers::container_process_factory_create_from_trait, plugin::{rust::bind_container_function, ContainerFactoryImpl}, prelude::*, processes::process_factory_create_from_trait};

#[cfg(feature="opencv4")]
use crate::opencv::prelude::*;

use crate::anyhow::{self, anyhow};
/// use super::python_process_factory_impl::PythonProcessFactoryImpl;
//use super::python_container_process_factory_impl::PythonContainerProcessFactoryImpl;
//use super::python_container_factory_impl::PythonContainerFactoryImpl;
pub struct PythonPlugin {
    path: PathBuf,
    pythonpaths: Option<Vec<PathBuf>>,
}


pub struct PythonContainerStruct {
    pub pyobj: Py<PyAny>
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

    pub fn load(path: PathBuf, pythonpaths: Option<Vec<PathBuf>>) -> JuizResult<PythonPlugin> {
        log::trace!("PythonPlugin::load({:?}) called", path);
        Ok(PythonPlugin{path, pythonpaths})
    }

    // pub fn get_manifest(&self, working_dir: Option<PathBuf>, symbol_name: &str) -> JuizResult<Value> {
    //     self.get_manifest_with_name(working_dir, symbol_name)
    // }

    fn init_path(&self, working_dir: Option<PathBuf>) -> JuizResult<()> {
        let fullpath = working_dir.clone().unwrap_or(env!("CARGO_MANIFEST_DIR").into()).join(self.path.clone());
        let pythonpaths = self.pythonpaths.clone();
        log::debug!("pythonpaths:{pythonpaths:?}");
        Python::with_gil(|py| -> PyResult<()> {
            log::trace!("in get_manifest_with_name(), Python:with_gil called (fullpath={:?}", fullpath.clone());
            log::debug!("PythonPlugin uses python version={:?}", py.version_info());
            // サブモジュールのためにルートディレクトリをpathに入れとく。
            let parent = fullpath.parent().unwrap().to_str().unwrap();
            let _ = PyModule::from_code_bound(py, &format!(r#"
import sys
if not "{parent:}" in sys.path:
    sys.path.append("{parent:}")
            "#), "", "");
            if let Some(paths) = pythonpaths {
                paths.iter().for_each(|p| {
                    let path = if p.is_absolute() { p.clone() } else { fullpath.parent().unwrap().join(p) };
                    let path_str = path.to_str().unwrap();
                    let _ = PyModule::from_code_bound(py, &format!(r#"
import sys
if not "{path_str:}" in sys.path:
    sys.path.append("{path_str:}")
            "#), "", "");
                });
            }
            Ok(())
        })?;
        Ok(())
    }

    fn get_manifest_with_name(&self, working_dir: Option<PathBuf>, symbol_name: &str) -> JuizResult<Value> {
        let fullpath = working_dir.clone().unwrap_or(env!("CARGO_MANIFEST_DIR").into()).join(self.path.clone());
        let pythonpaths = self.pythonpaths.clone();
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
            if let Some(paths) = pythonpaths {
                paths.iter().for_each(|p| {
                    let path = if p.is_absolute() { p.clone() } else { fullpath.join(p) };
                    let path_str = path.to_str().unwrap();
                    let _ = PyModule::from_code_bound(py, &format!(r#"
import sys
if not "{path_str:}" in sys.path:
    sys.path.append("{path_str:}")
            "#), "", "");
                });
            }
            
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


    
    
    // pub fn load_container_factory_with_manifest(&self, working_dir: Option<PathBuf>, manifest: Value) -> JuizResult<ContainerFactoryPtr> {
    //     Ok(ContainerFactoryPtr::new(PythonContainerFactoryImpl::new(
    //         manifest,
    //         working_dir.unwrap_or(env!("CARGO_MANIFEST_DIR").into()).join(self.path.clone())
    //     )?))
    // }

    pub fn load_container_process_factory(&self, working_dir: Option<PathBuf>, symbol_name: &str) -> JuizResult<ContainerProcessFactoryPtr> {
        log::trace!("PythonPlugin({:?})::load_container_process_factory(symbol_name='{symbol_name}') called", self.path);
        self.init_path(working_dir.clone())?;
        // self.load_container_process_factory_with_manifest(working_dir.clone(), cp_profile.clone(), symbol_name)
        let fullpath = working_dir.unwrap_or(env!("CARGO_MANIFEST_DIR").into()).join(self.path.clone());
        let mut manifest = jvalue!({});
        let py_app = fs::read_to_string(fullpath.clone()).unwrap();
        let pyfunc2 = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
            let module = PyModule::from_code_bound(py, &py_app.to_string(), fullpath.clone().to_str().unwrap(), "")?;
            let tuple = module.getattr(symbol_name)?.into_py(py).call0(py)?;
            let pytuple = tuple.extract::<&PyTuple>(py)?;
            manifest = pyany_to_value(pytuple.get_item(0)?)?;
            let pyfunc = pytuple.get_item(1)?.extract::<&PyFunction>()?;
            //Ok(tuple)
            Ok(pyfunc.to_object(py))
        })?;

        let signature = get_python_function_signature(&pyfunc2)?;
        let function = move |container: &mut ContainerImpl<PythonContainerStruct>, argument: CapsuleMap| -> JuizResult<Capsule> {
            Python::with_gil(|py| {
                let start_index = 1;
                let v  = capsulemap_to_pytuple(py, &argument, &signature, start_index)?;
                let elements = arg_to_pyargs(container, &v);
                python_process_call(py, &pyfunc2, PyTuple::new_bound(py, elements))
            }).or_else(|e| { Err(anyhow!(e)) })
        };
    
        container_process_factory_create_from_trait(manifest.try_into()?, bind_container_function(function))
    
    }

    // fn load_container_process_factory_with_manifest(&self, working_dir: Option<PathBuf>, manifest: Value, symbol_name: &str) -> JuizResult<ContainerProcessFactoryPtr> {
    //     Ok(ContainerProcessFactoryPtr::new(PythonContainerProcessFactoryImpl::new(
    //         manifest,
    //         working_dir.unwrap_or(env!("CARGO_MANIFEST_DIR").into()).join(self.path.clone()),
    //         symbol_name
    //     )?))
    // }

    pub fn load_process_factory(&self, working_dir: Option<PathBuf>, symbol_name: &str) -> JuizResult<ProcessFactoryPtr> {
        log::trace!("PythonPlugin({:?})::load_process_factory(symbol_name='{symbol_name}') called", self.path);
        self.init_path(working_dir.clone())?;
        let fullpath = working_dir.unwrap_or(env!("CARGO_MANIFEST_DIR").into()).join(self.path.clone());
        let mut manifest = jvalue!({});
        let py_app = fs::read_to_string(fullpath.clone()).unwrap();
        let pyfunc2 = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
            let module = PyModule::from_code_bound(py, &py_app.to_string(), fullpath.clone().to_str().unwrap(), "")?;
            let tuple = module.getattr(symbol_name)?.into_py(py).call0(py)?;
            let pytuple = tuple.extract::<&PyTuple>(py)?;
            manifest = pyany_to_value(pytuple.get_item(0)?)?;
            let pyfunc = pytuple.get_item(1)?.extract::<&PyFunction>()?;
            //Ok(tuple)
            Ok(pyfunc.to_object(py))
        })?;

        let signature = get_python_function_signature(&pyfunc2)?;
        let function = move |argument: CapsuleMap| -> JuizResult<Capsule> {
            Python::with_gil(|py| {
                python_process_call(py, &pyfunc2, PyTuple::new_bound(py, capsulemap_to_pytuple(py, &argument, &signature, 0)?))
            }).or_else(|e| { Err(anyhow!(e)) })
        };
        process_factory_create_from_trait(manifest.try_into()?, function)
    }

    pub fn load_container_factory(&self, working_dir: Option<PathBuf>, symbol_name: &str) -> JuizResult<ContainerFactoryPtr> {
        log::trace!("PythonPlugin({:?})::load_container_factory(symbol_name='{symbol_name}') called", self.path);
        
        let fullpath = working_dir.unwrap_or(env!("CARGO_MANIFEST_DIR").into()).join(self.path.clone());
        let mut manifest = jvalue!({});
        let py_app = fs::read_to_string(fullpath.clone()).unwrap();
        let pyfunc2 = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
            let module = PyModule::from_code_bound(py, &py_app.to_string(), fullpath.clone().to_str().unwrap(), "")?;
            let tuple = module.getattr(symbol_name)?.into_py(py).call0(py)?;
            let pytuple = tuple.extract::<&PyTuple>(py)?;
            manifest = pyany_to_value(pytuple.get_item(0)?)?;
            let pyfunc = pytuple.get_item(1)?.extract::<&PyFunction>()?;
            //Ok(tuple)
            Ok(pyfunc.to_object(py))
        })?;
        // let signature = get_python_function_signature(&pyfunc2)?;
        let constructor = move |cm: ContainerManifest| -> JuizResult<ContainerPtr> {
            let pyobj = Python::with_gil(|py| {
                pyfunc2.call1(py, PyTuple::new_bound(py,  [value_to_pyany(py, &cm.clone().into())]))
            })?;
            Ok(ContainerPtr::new(ContainerImpl::new(cm, Box::new(PythonContainerStruct{
                pyobj,
            }))?))
        };
        
        Ok(ContainerFactoryPtr::new(ContainerFactoryImpl::new(manifest.try_into()?, Arc::new(constructor))?))
        
        //container_factory_create_with_trait(manifest.try_into()?, constructor)
    }

    pub fn load_component_manifest(&self, working_dir: Option<PathBuf>) -> JuizResult<ComponentManifest> {
        self.get_manifest_with_name(working_dir, "component_manifest")?.try_into()
    }
}

fn arg_to_pyargs<'a>(c: &'a mut ContainerImpl<PythonContainerStruct>, arg: &'a Vec<Py<PyAny>> ) -> Vec<&'a Py<PyAny>> {
    let mut vec_arg: Vec<&Py<PyAny>> = Vec::new();
    vec_arg.push(&c.t.pyobj);
    vec_arg.extend(arg.iter());
    vec_arg
}

pub fn get_python_function_signature(func: &Py<PyAny>) -> PyResult<Value> {
    Python::with_gil(|py| {
        let py_app = r#"
from inspect import signature
def argument_info(func):
    sig = signature(func)
    sig_dict = []
    for k, v in sig.parameters.items():
        sig_dict.append( {
            "key": k,
            "kind": str(v.kind),
            "name": v.name,
            "annotation": str(v.annotation.__name__) if v.annotation is not v.empty else "empty",
            "default": str(v.default) if v.default is not v.empty else "empty",
        } )
    return sig_dict
        "#;
        let module = PyModule::from_code_bound(py, &py_app.to_string(), "", "")?;
        let value = module.getattr("argument_info")?.into_py(py).call1(py, PyTuple::new_bound(py,[func.clone_ref(py)]))?;
        pyany_to_value(value.extract::<&PyAny>(py)?)
    })
}

// pub fn get_entry_point(fullpath: &PathBuf, symbol_name: &str) -> PyResult<Py<PyAny>> {
//     Python::with_gil(|py| -> PyResult<Py<PyAny>> {
//         let py_app = fs::read_to_string(fullpath).unwrap();
//         let module = PyModule::from_code_bound(py, &py_app.to_string(), fullpath.to_str().unwrap(), "")?;
//         module.getattr(symbol_name)?.into_py(py).call0(py)
//     })
// }



pub fn check_object_is_ndarray(_py: &Python, value: &PyAny) -> bool {
    value.get_type().to_string() == "<class 'numpy.ndarray'>".to_owned()
}

fn function_argument_name_list(signature: &Value) -> JuizResult<Vec<String>> {
    Ok(get_array(signature)?.iter().map(|v| { obj_get_str(v, "name").unwrap().to_owned() }).collect::<Vec<String>>())
}

pub fn capsulemap_to_pytuple<'a>(py: Python, value: &'a CapsuleMap, signature: &Value, start_index: usize) -> JuizResult<Vec<Py<PyAny>>> {
    
    let arg_name_list = function_argument_name_list(signature)?;
    let mut index = start_index;
    let mut vec: Vec<Py<PyAny>> = Vec::new();
    for (i, arg_name) in arg_name_list.iter().enumerate() {
        if i < index { continue; };
        let v = value.get(arg_name.as_str())?;
        vec.push(capsuleptr_to_pyany(py, &v));
        index += 1;
    }
    return Ok(vec);
    // value.iter().map(|(_k, v)| { 
    //     let arg_name = arg_name_list.get(index);
    //     capsuleptr_to_pyany(py, v)

    // } ).collect::<Vec<Py<PyAny>>>()
}
#[allow(unused)]
pub fn value_to_pytuple<'a>(py: Python, value: &'a Value) -> Vec<Py<PyAny>> {
    vec!(value_to_pyany(py, value))
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

// pub fn python_process_call(py: Python, entry_point: &Py<PyAny>, arguments: Vec<Py<PyAny>>) -> JuizResult<Capsule> {
//     match entry_point.call1(py, PyTuple::new_bound(py, arguments)) {
//         Ok(result) => {
//             Ok(pyany_to_value(result.extract::<&PyAny>(py)?)?.into())
//         }
//         Err(e) => {
//             log::error!("Error calling python call. {e:}");
//             Err(anyhow::Error::from(e))
//         }
//     }
// }


#[cfg(feature="opencv4")]
pub fn python_process_call(py: Python, entry_point: &Py<PyAny>, pytuple: pyo3::Bound<PyTuple>) -> JuizResult<Capsule> {
    match entry_point.call1(py, pytuple) {
        Ok(v) => {
            let object = v.extract::<&PyAny>(py)?;
            Ok(if check_object_is_ndarray(&py, object) {
                pyany_to_mat(py, object).unwrap()
            } else {
                pyany_to_value(object)?.into()
            })
        },
        Err(e) => {
            let trace_str = e.traceback_bound(py).and_then(|trace| { Some(format!("{:}", trace.format().unwrap())) }).or(Some("".to_owned())).unwrap();
            log::error!("Error in Python::with_gil for ContainerProcess.call(). Error({e:}). Traceback: {trace_str:}");
            Err(anyhow!(e))
        },
    }

}

#[cfg(not(feature="opencv4"))]
pub fn python_process_call(py: Python, entry_point: &Py<PyAny>, pytuple: pyo3::Bound<PyTuple>) -> JuizResult<Capsule> {
    match entry_point.call1(py, pytuple) {
        Ok(v) => {
            let object = v.extract::<&PyAny>(py)?;
            Ok(if check_object_is_ndarray(&py, object) {
                //pyany_to_mat(py, object).unwrap()
                todo!()
            } else {
                pyany_to_value(object)?.into()
            })
        },
        Err(e) => {
            let trace_str = e.traceback_bound(py).and_then(|trace| { Some(format!("{:}", trace.format().unwrap())) }).or(Some("".to_owned())).unwrap();
            log::error!("Error in Python::with_gil for ContainerProcess.call(). Error({e:}). Traceback: {trace_str:}");
            Err(anyhow!(e))
        },
    }

}


#[cfg(feature="opencv4")]
fn pytuple_to_mat(pytuple: &PyTuple) -> JuizResult<Capsule> {
    let shape = pytuple.get_item(0)?.extract::<&PyTuple>()?.into_iter().map(|v|{v.extract::<i32>().unwrap()}).collect::<Vec<i32>>();
    Ok(opencv::core::Mat::new_rows_cols_with_data(
        shape[0] * shape[2],
        shape[1],
        pytuple.get_item(1)?.extract::<&PyBytes>()?.as_bytes()
    )?.reshape(3, shape[0])?.try_clone()?.into())
}


#[cfg(feature="opencv4")]
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
    } else if value.is_instance_of::<PySet>() {
        pyset_to_value(value.extract::<&PySet>()?)
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
fn pyset_to_value(pyset: &PySet) -> PyResult<Value> {
    let mut vec: Vec<Value> = Vec::new();
    for value in pyset.into_iter() {
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
