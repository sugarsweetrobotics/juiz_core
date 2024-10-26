
use std::ffi::c_void;


use crate::containers::ContainerImpl;
use crate::prelude::*;
use crate::object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore};

pub struct CppContainerStruct {
    pub cobj: *mut std::ffi::c_void
}

// pub type CppContainer = ContainerImpl<CppContainerStruct>;
// pub type CppContainerConstructFunction = dyn Fn(Value) -> JuizResult<Box<CppContainerStruct>>;

#[repr(C)]
pub struct CppContainerFactoryImpl {
    core: ObjectCore,
    manifest: ContainerManifest,
    entry_point: unsafe fn(*mut Value, *mut *mut std::ffi::c_void) -> i64
}

impl CppContainerFactoryImpl {

    pub fn new2(manifest: ContainerManifest, entry_point: unsafe fn(*mut Value, *mut *mut std::ffi::c_void) -> i64) -> JuizResult<Self> {

        log::trace!("new2({manifest:?}) called");
        //let type_name = obj_get_str(manifest, "type_name")?;
        Ok( CppContainerFactoryImpl{
            core: ObjectCore::create_factory(JuizObjectClass::ContainerFactory("ContainerFactoryImpl"), manifest.type_name.clone()),
            manifest, //: check_process_factory_manifest(manifest.clone())?,
            entry_point
        })
    }

    // fn apply_default_manifest(&self, manifest: Value) -> Result<Value, JuizError> {
    //     let mut new_manifest = self.manifest.clone();
    //     for (k, v) in manifest.as_object().unwrap().iter() {
    //         new_manifest.as_object_mut().unwrap().insert(k.to_owned(), v.clone());
    //     }
    //     return Ok(new_manifest);
    // }
}


impl JuizObjectCoreHolder for CppContainerFactoryImpl {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}

impl JuizObject for CppContainerFactoryImpl {}

impl ContainerFactory for CppContainerFactoryImpl {

    fn create_container(&self, _core_worker: &mut CoreWorker, manifest: ContainerManifest) -> JuizResult<ContainerPtr>{
        log::trace!("ContainerFactoryImpl({:?})::create_container(manifest={:?}) called", self.manifest, manifest);
        let mut pobj: *mut c_void = std::ptr::null_mut();
        unsafe {
            let symbol = self.entry_point.clone();
            let retval = (symbol)(&mut manifest.clone().into(), &mut pobj);
            if retval < 0 || pobj == std::ptr::null_mut() {
                return Err(anyhow::Error::from(JuizError::CppPluginFunctionCallError { function_name: "create_container".to_owned(), return_value: retval }));
            }
        }
        
        Ok(ContainerPtr::new(ContainerImpl::new(
            manifest,
            //self.apply_default_manifest(manifest.clone())?,
            Box::new(CppContainerStruct {
                cobj: pobj,
            })
        )?))
    }
    
    fn destroy_container(&mut self, c: ContainerPtr) -> JuizResult<Value> {
        log::warn!("CppContainerFactoryImpl::destroy_container() called");
        let prof = c.lock()?.profile_full()?;
        Ok(prof)
    }
    
}

