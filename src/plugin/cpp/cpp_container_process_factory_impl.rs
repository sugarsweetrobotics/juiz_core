
use std::sync::{Arc, RwLock};


use crate::prelude::*;
use crate::{containers::{container_proc_lock, ContainerProcessImpl}, object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore}, utils::check_process_factory_manifest, value::obj_get_str};

use super::cpp_container_factory_impl::CppContainerStruct;
#[repr(C)]
pub struct CppContainerProcessFactoryImpl {
    core: ObjectCore,
    manifest: Value,
    //plugin: Rc<CppPlugin>,
    entry_point: unsafe fn(*mut std::ffi::c_void, *mut CapsuleMap, *mut Capsule) -> i64,
}

impl CppContainerProcessFactoryImpl {

    pub fn new2(manifest: &Value, entry_point: unsafe fn(*mut std::ffi::c_void, *mut CapsuleMap, *mut Capsule) -> i64) -> JuizResult<Self> {

        log::trace!("new2({manifest:}) called");
        let type_name = obj_get_str(manifest, "type_name")?;
        Ok( CppContainerProcessFactoryImpl{
            core: ObjectCore::create_factory(JuizObjectClass::ContainerFactory("ContainerFactoryImpl"), type_name),
            manifest: check_process_factory_manifest(manifest.clone())?,
            entry_point
        })
    }

    fn apply_default_manifest(&self, manifest: Value) -> Result<Value, JuizError> {
        let mut new_manifest = self.manifest.clone();
        for (k, v) in manifest.as_object().unwrap().iter() {
            new_manifest.as_object_mut().unwrap().insert(k.to_owned(), v.clone());
        }
        return Ok(new_manifest);
    }
}


impl JuizObjectCoreHolder for CppContainerProcessFactoryImpl {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}

impl JuizObject for CppContainerProcessFactoryImpl {}


impl ContainerProcessFactory for CppContainerProcessFactoryImpl {
    fn create_container_process(&self, container: ContainerPtr, manifest: Value) -> JuizResult<ContainerProcessPtr> {
        log::trace!("ContainerProcessFactoryImpl::create_container_process(container, manifest={}) called", manifest);
        

        let type_name = self.type_name().to_owned();
        let entry_point = self.entry_point;

        let function = Arc::new(move |c: &mut ContainerImpl<CppContainerStruct>, mut argument: CapsuleMap | -> JuizResult<Capsule> {
            let mut retval = Capsule::empty();
            let return_value = unsafe {
                (entry_point)(c.t.cobj, &mut argument, &mut retval)
            };
            if return_value != 0 {
                return Err(anyhow::Error::from(JuizError::CppPluginFunctionCallError { function_name: format!("container_process({type_name})"), return_value }));
            }
            Ok(retval)
        });

        Ok(Arc::new(RwLock::new(
            ContainerProcessImpl::new(
                self.apply_default_manifest(manifest)?, 
                Arc::clone(&container), 
                function)?
        )))
        
    }
    
    fn destroy_container_process(&mut self, p: ContainerProcessPtr) -> JuizResult<Value> {
        log::warn!("CppContainerFactoryImpl::destroy_container_process() called");
        let prof = container_proc_lock(&p)?.profile_full()?;
        Ok(prof)
    }
}