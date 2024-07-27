
use std::{fs, path::PathBuf, sync::{Arc, RwLock}};
use anyhow::Context;
use pyo3::{prelude::*, types::PyTuple};

use super::container_impl::ContainerImpl;
use crate::{containers::{container_process_impl::ContainerProcessImpl, PythonContainerStruct}, plugin::pyany_to_value, object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore}, processes::python_process_factory_impl::capsulemap_to_pytuple, utils::check_process_factory_manifest, value::obj_get_str, Capsule, CapsuleMap, ContainerProcessFactory, ContainerPtr, JuizError, JuizObject, JuizResult, ProcessPtr, Value};

#[repr(C)]
pub struct PythonContainerProcessFactoryImpl {
    core: ObjectCore,
    manifest: Value,
    fullpath: PathBuf,
    //constructor: PythonContainerConstructFunction
}

pub fn create_python_container_process_factory(manifest: crate::Value, fullpath: PathBuf, /*constructor: PythonContainerConstructFunction */ ) -> JuizResult<PythonContainerProcessFactoryImpl> {
    log::trace!("create_container_factory called");
    PythonContainerProcessFactoryImpl::new(manifest, fullpath).context("create_container_factory()")
}

impl PythonContainerProcessFactoryImpl {

    pub fn new(manifest: crate::Value, fullpath: PathBuf/*, constructor: PythonContainerConstructFunction*/) -> JuizResult<Self> {
        let type_name = obj_get_str(&manifest, "type_name")?;
        Ok( PythonContainerProcessFactoryImpl{
                core: ObjectCore::create_factory(JuizObjectClass::ContainerProcessFactory("ContainerProcessFactoryImpl"), type_name),
                manifest: check_process_factory_manifest(manifest)?,
                fullpath,
                //constructor
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


impl JuizObjectCoreHolder for PythonContainerProcessFactoryImpl {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}

impl JuizObject for PythonContainerProcessFactoryImpl {}


impl ContainerProcessFactory for PythonContainerProcessFactoryImpl {
    fn create_container_process(&self, container: ContainerPtr, manifest: crate::Value) -> JuizResult<ProcessPtr> {
        log::trace!("ContainerProcessFactoryImpl::create_container_process(container, manifest={}) called", manifest);
        

        let type_name = self.type_name().to_owned();
        let fullpath = self.fullpath.clone();
        let pyfunc: Arc<dyn Fn(&mut ContainerImpl<PythonContainerStruct>, CapsuleMap)->JuizResult<Capsule>> = Arc::new(move |c: &mut ContainerImpl<PythonContainerStruct>, argument: CapsuleMap| -> JuizResult<Capsule> {
            let mut func_result : Capsule = Capsule::empty();
            // let type_name = self.type_name();
            let tn = type_name.clone();
            let fp = fullpath.clone();
            // ここ、実行するたびにファイルを開くのでよくないかもしれない。
            let _pyobj = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let py_app = fs::read_to_string(fp).unwrap();
                let module = PyModule::from_code_bound(py, &py_app.to_string(), "", "")?;
                let app_func: Py<PyAny> = module.getattr(tn.as_str())?.into();
                let mut vec_arg: Vec<&Py<PyAny>> = Vec::new();
                vec_arg.push(&c.t.pyobj);
                let v  = capsulemap_to_pytuple(py, &argument);
                vec_arg.extend(v.iter());
                // = capsulemap_to_pytuple(py, &argument);
                //vec_arg.
                let result = app_func.call1(py, PyTuple::new_bound(py, vec_arg))?;
                let result_value = pyany_to_value(result.extract::<&PyAny>(py)?)?;
                func_result = result_value.into();
                //println!("func_result: {:?}", func_result);
                Ok(result)
            });
            // println!("result: {:?}", pyobj);

            // println!("wow: func_result: {:?}", func_result);
            return Ok(func_result);
        });


        //let function = | c: &mut ContainerImpl<PythonContainerStruct>, argument: CapsuleMap | {
        //    let retval = Capsule::empty();
        //    Ok(retval)
        //};

        Ok(Arc::new(RwLock::new(
            ContainerProcessImpl::new(
                self.apply_default_manifest(manifest)?, 
                Arc::clone(&container), 
                pyfunc)?
        )))
        
    }
}