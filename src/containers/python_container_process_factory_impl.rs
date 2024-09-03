
use std::{fs, path::PathBuf, sync::{Arc, RwLock}};
use anyhow::{anyhow, Context};
use pyo3::{prelude::*, types::PyTuple};

use super::{container_impl::ContainerImpl, container_process_impl::ContainerProcessPtr};
use crate::{containers::{container_process_impl::{container_proc_lock, ContainerProcessImpl}, PythonContainerStruct}, object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore}, plugin::{pyany_to_mat, pyany_to_value}, processes::python_process_factory_impl::capsulemap_to_pytuple, utils::check_process_factory_manifest, value::obj_get_str, Capsule, CapsuleMap, ContainerProcessFactory, ContainerPtr, JuizError, JuizObject, JuizResult, Value};

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
    fn create_container_process(&self, container: ContainerPtr, manifest: crate::Value) -> JuizResult<ContainerProcessPtr> {
        log::trace!("ContainerProcessFactoryImpl::create_container_process(container, manifest={}) called", manifest);
        

        let type_name = self.type_name().to_owned();
        let fullpath = self.fullpath.clone();
        let pyfunc: Arc<dyn Fn(&mut ContainerImpl<PythonContainerStruct>, CapsuleMap)->JuizResult<Capsule>> 
            = Arc::new(move |c: &mut ContainerImpl<PythonContainerStruct>, argument: CapsuleMap| -> JuizResult<Capsule> {
                log::trace!("PythonContainerProcess function (type_name={type_name}) called");
                let mut func_result : Capsule = Capsule::empty();
            // let type_name = self.type_name();
            let tn = type_name.clone();
            let fp = fullpath.clone();
            // ここ、実行するたびにファイルを開くのでよくないかもしれない。
            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                log::trace!(" - with_gil()");
                // log::debug!("PythonPlugin uses python version={:?}", py.version_info());
                let py_app = fs::read_to_string(fp).unwrap();
                let module = PyModule::from_code_bound(py, &py_app.to_string(), "", "")?;
                log::trace!(" - module: {module:?}");
                let app_func: Py<PyAny> = module.getattr(tn.as_str())?.into();
                let mut vec_arg: Vec<&Py<PyAny>> = Vec::new();
                vec_arg.push(&c.t.pyobj);
                let v  = capsulemap_to_pytuple(py, &argument);
                vec_arg.extend(v.iter());

                let result = app_func.call1(py, PyTuple::new_bound(py, vec_arg))?;
                log::trace!(" - returns {:?}", result);
                let object = result.extract::<&PyAny>(py)?;
                let result_value: Capsule = if check_object_is_ndarray(&py, object) {
                    pyany_to_mat(py, object).unwrap()
                } else {
                    pyany_to_value(object)?.into()
                };
                log::trace!(" - returns {:?}", result_value);
                func_result = result_value;
                //println!("func_result: {:?}", func_result);
                Ok(result)
            }).or_else(|e| {
                log::error!("Python ContainerProcess(type_name={type_name}) call failed. Error({e:})");
                Err(anyhow!(e))
            })?;
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
    
    fn destroy_container_process(&mut self, p: ContainerProcessPtr) -> JuizResult<Value> {
        log::warn!("PythonContainerFactoryImpl::destroy_container_process() called");
        let prof = container_proc_lock(&p)?.profile_full()?;
        Ok(prof)
    }
}

fn check_object_is_ndarray(_py: &Python, value: &PyAny) -> bool {
    //log::debug!("check_object_is_ndarray({value:?}");
    let pytype = value.get_type();
    //log::debug!(" - {:}", pytype.to_string());
    pytype.to_string() == "<class 'numpy.ndarray'>".to_owned()
}