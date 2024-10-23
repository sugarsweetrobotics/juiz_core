
use std::{fs, path::PathBuf};
use pyo3::{prelude::*, types::PyTuple};

use crate::prelude::*;
use crate::containers::ContainerImpl;
use crate::plugin::python::python_plugin::value_to_pytuple;
use crate::{object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore}, utils::check_process_factory_manifest, value::obj_get_str};

pub struct PythonContainerStruct {
    pub pyobj: Py<PyAny>
}

//pub type PythonContainer = ContainerImpl<PythonContainerStruct>;
// pub type PythonContainerConstructFunction = dyn Fn(Value) -> JuizResult<Box<PythonContainerStruct>>;

#[repr(C)]
pub struct PythonContainerFactoryImpl {
    core: ObjectCore,
    manifest: Value,
    fullpath: PathBuf,
    //constructor: PythonContainerConstructFunction
}

impl PythonContainerFactoryImpl {

    pub fn new(manifest: Value, fullpath: PathBuf/*, constructor: PythonContainerConstructFunction*/) -> JuizResult<Self> {
        let type_name = obj_get_str(&manifest, "type_name")?;
        Ok( PythonContainerFactoryImpl{
                core: ObjectCore::create_factory(JuizObjectClass::ContainerFactory("ContainerFactoryImpl"), type_name),
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


impl JuizObjectCoreHolder for PythonContainerFactoryImpl {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}

impl JuizObject for PythonContainerFactoryImpl {}

impl ContainerFactory for PythonContainerFactoryImpl {

    fn create_container(&self, _core_worker: &mut CoreWorker, manifest: ContainerManifest) -> JuizResult<ContainerPtr>{
        //let type_name = self.type_name().to_owned();
        let full_path = self.fullpath.clone();
        
        let pyobj = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
            log::trace!("PythonContainerFactoryImpl::create_container({manifest:?}) called.");
            let py_app = fs::read_to_string(full_path).unwrap();
            let module = PyModule::from_code_bound(py, &py_app.to_string(), "", "")?;
            let app_func: Py<PyAny> = module.getattr(manifest.type_name.as_str())?.into();
            app_func.call1(py, PyTuple::new_bound(py, value_to_pytuple(py, &manifest.clone().into())))
        })?;

        Ok(ContainerPtr::new(ContainerImpl::new(
                //self.apply_default_manifest(manifest.clone())?,
                manifest,
                Box::new(PythonContainerStruct {
                    pyobj
                })
            )?))
    }
    
    fn destroy_container(&mut self, c: ContainerPtr) -> JuizResult<Value> {
        log::warn!("PythonContainerFactoryImpl::destroy_container() called");
        c.lock()?.profile_full()
    }
    
}

