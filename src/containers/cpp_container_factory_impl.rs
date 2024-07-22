
use std::{ffi::c_void, rc::Rc};


use super::container_impl::ContainerImpl;
use crate::{core::cpp_plugin::CppPlugin, object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore}, utils::check_process_factory_manifest, value::obj_get_str, ContainerFactory, ContainerPtr, JuizError, JuizObject, JuizResult, Value};

pub struct CppContainerStruct {
    pub cobj: *mut std::ffi::c_void
}

pub type CppContainer = ContainerImpl<CppContainerStruct>;
pub type CppContainerConstructFunction = dyn Fn(Value) -> JuizResult<Box<CppContainerStruct>>;

#[repr(C)]
pub struct CppContainerFactoryImpl {
    core: ObjectCore,
    manifest: Value,
    plugin: Rc<CppPlugin>,
    //fullpath: PathBuf,
    //constructor: PythonContainerConstructFunction
}

// pub fn create_cpp_container_factory(manifest: crate::Value, fullpath: PathBuf, /*constructor: PythonContainerConstructFunction */ ) -> JuizResult<CppContainerFactoryImpl> {
//     log::trace!("create_container_factory called");
//     todo!()
//     //CppContainerFactoryImpl::new(manifest, fullpath).context("create_container_factory()")
// }

impl CppContainerFactoryImpl {

    pub fn new(plugin: Rc<CppPlugin>) -> JuizResult<Self>  {
        let manifest = plugin.get_manifest();
        let type_name = obj_get_str(manifest, "type_name")?;
        
        /*
        let symbol_name = "container_factory";
        type SymbolType = libloading::Symbol<'static, unsafe fn() -> unsafe fn(*mut CapsuleMap, *mut Capsule)->i64>;
        let f = unsafe {
            let symbol = plugin.load_symbol::<SymbolType>(symbol_name.as_bytes())?;
            (symbol)()
        };
        */

        Ok( CppContainerFactoryImpl{
                core: ObjectCore::create_factory(JuizObjectClass::ContainerFactory("ContainerFactoryImpl"), type_name),
                manifest: check_process_factory_manifest(plugin.get_manifest().clone())?,
                plugin,
                //constructor
            }
        )
    }

    pub fn new_with_manifest(plugin: Rc<CppPlugin>, manifest: &Value) -> JuizResult<Self>  {
        log::trace!("new_with_manifest(manifest={manifest:?}) called");
        //let manifest = plugin.get_manifest();
        let type_name = obj_get_str(manifest, "type_name")?;
        
        /*
        let symbol_name = "container_factory";
        type SymbolType = libloading::Symbol<'static, unsafe fn() -> unsafe fn(*mut CapsuleMap, *mut Capsule)->i64>;
        let f = unsafe {
            let symbol = plugin.load_symbol::<SymbolType>(symbol_name.as_bytes())?;
            (symbol)()
        };
        */

        Ok( CppContainerFactoryImpl{
                core: ObjectCore::create_factory(JuizObjectClass::ContainerFactory("ContainerFactoryImpl"), type_name),
                manifest: check_process_factory_manifest(manifest.clone())?,
                plugin,
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


impl JuizObjectCoreHolder for CppContainerFactoryImpl {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}

impl JuizObject for CppContainerFactoryImpl {}

impl ContainerFactory for CppContainerFactoryImpl {

    fn create_container(&self, mut manifest: Value) -> JuizResult<ContainerPtr>{
        log::trace!("ContainerFactoryImpl({})::create_container(manifest={}) called", self.manifest, manifest);
        let symbol_name = match self.manifest.as_object() {
            Some(map) => {
                match map.get("factory") {
                    Some(v) => {
                        match v.as_str() {
                            Some(s) => {
                                Ok(s)
                            }
                            None => {
                                log::error!("component_profile check failed. 'factory' is not str");
                                Err(anyhow::Error::from(JuizError::ArgumentError { message: "'factory' is not str".to_owned() }))
                            }
                        }
                    }
                    None => {
                        Ok("create_container")
                    }
                }
            }
            None => {
                log::error!("manifest check failed. Manifest is not map");
                Err(anyhow::Error::from(JuizError::ArgumentError { message: "Manifest is not map".to_owned() }))
            }
        }?;
        type SymbolType = unsafe fn(*mut Value) -> *mut c_void;
        
        let retval = unsafe {
            let symbol = match self.plugin.load_symbol::<SymbolType>(symbol_name.as_bytes()) {
                Ok(s) => {Ok(s)},
                Err(e) => {
                    log::error!("In create_container({manifest:?}), load symbol({symbol_name}) failed.");
                    Err(e)
                }
            }?;
            (symbol)(&mut manifest)
        };
        if retval == std::ptr::null_mut() {
            return Err(anyhow::Error::from(JuizError::CppPluginFunctionCallError { function_name: "create_container".to_owned(), return_value: 0 }));
        }


        Ok(ContainerImpl::new(
            self.apply_default_manifest(manifest.clone())?,
            Box::new(CppContainerStruct {
                cobj: retval,
            })
        )?)
    }
    
}

