
use std::{path::PathBuf, rc::Rc, sync::Arc};
use anyhow::anyhow;
use pyo3::{prelude::*, types::PyTuple};

use crate::prelude::*;
use crate::containers::ContainerImpl;
use super::python_container_factory_impl::PythonContainerStruct;
use crate::{containers::ContainerProcessImpl, object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore}, utils::check_process_factory_manifest, value::obj_get_str};
use super::python_plugin::{capsulemap_to_pytuple, get_entry_point, get_python_function_signature, python_process_call};

#[repr(C)]
pub struct PythonContainerProcessFactoryImpl {
    core: ObjectCore,
    manifest: Value,
    fullpath: PathBuf,
    entry_point: Rc<Py<PyAny>>,
    entry_point_signature: Value,
}

impl PythonContainerProcessFactoryImpl {

    pub fn new(manifest: Value, fullpath: PathBuf, symbol_name: &str) -> JuizResult<Self> {
        log::trace!("PythonContainerProcessFactoryImpl::new(manifest='{manifest:}', fullpath='{fullpath:?}', symbol_name='{symbol_name:}') called");
        let type_name = obj_get_str(&manifest, "type_name")?;
        // factoryメソッドを動かして関数本体の参照を得る。
        let entry_point = get_entry_point(&fullpath, symbol_name)?;
        Ok( PythonContainerProcessFactoryImpl{
                core: ObjectCore::create_factory(JuizObjectClass::ContainerProcessFactory("ContainerProcessFactoryImpl"), type_name),
                manifest: check_process_factory_manifest(manifest)?,
                fullpath,
                entry_point_signature: get_python_function_signature(&entry_point)?,
                entry_point: Rc::new(entry_point)
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


pub fn arg_to_pyargs<'a>(c: &'a mut ContainerImpl<PythonContainerStruct>, arg: &'a Vec<Py<PyAny>> ) -> Vec<&'a Py<PyAny>> {
    let mut vec_arg: Vec<&Py<PyAny>> = Vec::new();
    vec_arg.push(&c.t.pyobj);
    vec_arg.extend(arg.iter());
    vec_arg
}


impl ContainerProcessFactory for PythonContainerProcessFactoryImpl {
    fn create_container_process(&self, container: ContainerPtr, manifest: Value) -> JuizResult<ProcessPtr> {
        log::trace!("ContainerProcessFactoryImpl::create_container_process(container, manifest={}) called", manifest);
    
        let type_name = self.type_name().to_owned();
        let entry_point = self.entry_point.clone();
        let signature = self.entry_point_signature.clone();
        let pyfunc: Arc<dyn Fn(&mut ContainerImpl<PythonContainerStruct>, CapsuleMap)->JuizResult<Capsule>> 
            = Arc::new(move |c: &mut ContainerImpl<PythonContainerStruct>, argument: CapsuleMap| -> JuizResult<Capsule> {
            log::trace!("PythonContainerProcess function (type_name={type_name}) called");
            Python::with_gil(|py| {
                let start_index = 1;
                let v  = capsulemap_to_pytuple(py, &argument, &signature, start_index)?;
                let elements = arg_to_pyargs(c, &v);
                python_process_call(py, &entry_point, PyTuple::new_bound(py, elements))
            }).or_else(|e| { Err(anyhow!(e)) })
        });

        Ok(ProcessPtr::new(
            ContainerProcessImpl::new(
                self.apply_default_manifest(manifest)?, 
                container, 
                pyfunc)?
        ))
        
    }
    
    fn destroy_container_process(&mut self, p: ProcessPtr) -> JuizResult<Value> {
        log::warn!("PythonContainerFactoryImpl::destroy_container_process() called");
        p.lock()?.profile_full()
    }
}
