
use std::{rc::Rc, sync::{Arc, RwLock}};


use super::container_impl::ContainerImpl;
use crate::{containers::{container_process_impl::ContainerProcessImpl, cpp_container_factory_impl::CppContainerStruct}, core::cpp_plugin::CppPlugin, object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore}, utils::check_process_factory_manifest, value::obj_get_str, Capsule, CapsuleMap, ContainerProcessFactory, ContainerPtr, JuizError, JuizObject, JuizResult, ProcessPtr, Value};

#[repr(C)]
pub struct CppContainerProcessFactoryImpl {
    core: ObjectCore,
    manifest: Value,
    plugin: Rc<CppPlugin>,
    entry_point: unsafe fn(*mut std::ffi::c_void, *mut CapsuleMap, *mut Capsule) -> i64,
}

// pub fn create_python_container_process_factory(manifest: crate::Value, fullpath: PathBuf, /*constructor: PythonContainerConstructFunction */ ) -> JuizResult<CppContainerProcessFactoryImpl> {
//     log::trace!("create_container_factory called");
//     todo!();//CppContainerProcessFactoryImpl::new(manifest, fullpath).context("create_container_factory()")
// }

impl CppContainerProcessFactoryImpl {

    pub fn new(plugin: Rc<CppPlugin>, symbol_name: &str) -> JuizResult<Self> {
        log::trace!("new(symbol_name={symbol_name:}) called");
        let manifest = plugin.get_manifest();
        let type_name = obj_get_str(&manifest, "type_name")?;
        //let symbol_name = "container_process_factory";
        type SymbolType = libloading::Symbol<'static, unsafe fn() -> unsafe fn(*mut std::ffi::c_void, *mut CapsuleMap, *mut Capsule)->i64>;
        let f = unsafe {
            let symbol = plugin.load_symbol::<SymbolType>(symbol_name.as_bytes())?;
            (symbol)()
        };

        Ok( CppContainerProcessFactoryImpl{
                core: ObjectCore::create_factory(JuizObjectClass::ContainerProcessFactory("ContainerProcessFactoryImpl"), type_name),
                manifest: check_process_factory_manifest(manifest.clone())?,
                plugin,
                entry_point: f,
            }
        )
    }


    pub fn new_with_manifest(plugin: Rc<CppPlugin>, symbol_name: &str, manifest: &Value) -> JuizResult<Self> {
        log::trace!("new_with_manifest(manifest={manifest:?}, symbol_name={symbol_name:}) called");
        //let manifest = plugin.get_manifest();
        let type_name = obj_get_str(&manifest, "type_name")?;
        //let symbol_name = "container_process_factory";
        type SymbolType = libloading::Symbol<'static, unsafe fn() -> unsafe fn(*mut std::ffi::c_void, *mut CapsuleMap, *mut Capsule)->i64>;
        let f = unsafe {
            let symbol = plugin.load_symbol::<SymbolType>(symbol_name.as_bytes())?;
            (symbol)()
        };

        Ok( CppContainerProcessFactoryImpl{
                core: ObjectCore::create_factory(JuizObjectClass::ContainerProcessFactory("ContainerProcessFactoryImpl"), type_name),
                manifest: check_process_factory_manifest(manifest.clone())?,
                plugin,
                entry_point: f,
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


impl JuizObjectCoreHolder for CppContainerProcessFactoryImpl {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}

impl JuizObject for CppContainerProcessFactoryImpl {}


impl ContainerProcessFactory for CppContainerProcessFactoryImpl {
    fn create_container_process(&self, container: ContainerPtr, manifest: crate::Value) -> JuizResult<ProcessPtr> {
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
}