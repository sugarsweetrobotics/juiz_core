
use std::ffi::c_void;
use std::path::PathBuf;
use std::sync::Arc;
use libloading::{Library, Symbol};

//use super::cpp_container_factory_impl::CppContainerFactoryImpl;
//use super::cpp_container_process_factory_impl::CppContainerProcessFactoryImpl;
//use crate::brokers::http::http_router::container;
use crate::containers::{container_process_factory_create_from_trait};
use crate::plugin::rust::bind_container_function;
use crate::plugin::ContainerFactoryImpl;
//use crate::plugin::cpp::cpp_container_factory_impl::CppContainerStruct;
use crate::prelude::*;
use crate::processes::process_factory_create_from_trait;

pub struct CppPlugin{
    path: PathBuf,
    lib: Library,
    manifest: Value,
}

pub struct CppContainerStruct {
    pub cobj: *mut std::ffi::c_void
}

/*
pub type Symbol<'lib, T> = libloading::Symbol<'lib, libloading::Symbol<'lib, T>>;
*/
impl CppPlugin {

    pub fn profile_full(&self) -> JuizResult<Value> {
        Ok(jvalue!({
            "path": self.path,
        }))
    }

    pub fn get_manifest(&self) -> &Value {
        &self.manifest
    }

    pub fn load_component_manifest(&self, _working_dir: Option<PathBuf>) -> JuizResult<ComponentManifest> {
        Ok(self.get_manifest().clone().try_into()?)
    }
    
    pub fn new(path: PathBuf, manifest_entry_point: &str) -> JuizResult<CppPlugin> {
        log::trace!("CppPlugin::new({:?}, {:?}) called", path, manifest_entry_point);
        let entry_point = manifest_entry_point.to_owned() + "_entry_point";
        unsafe {
            let lib = match Library::new(path.clone()) {
                Ok(l) => Ok(l),
                Err(e) => {
                    log::error!("Library::new({path:?}) failed. Error ({e:?})");
                    Err(e)
                }
            }?;            
            let mut manif_cap = CapsulePtr::new();
            let manifest_function: Symbol<extern "C" fn(*mut CapsulePtr) -> i64> = match lib.get(entry_point.as_bytes()) {
                Ok(f) => Ok(f),
                Err(e) => {
                    log::error!("Library({path:?})::get({entry_point}) failed. Error ({e:?})");
                    Err(e)
                }
            }?;
            let retval = manifest_function(&mut manif_cap);
            if retval != 0 {
                return Err(anyhow::Error::from(JuizError::CppProcessFunctionCallError{}));
            }
            let manifest = manif_cap.extract_value()?;

            
            Ok(CppPlugin{path, lib, manifest})
        }
    }

    pub fn load_process_factory(&self, _working_dir: Option<PathBuf>, symbol_name: &str) -> JuizResult<ProcessFactoryPtr> {
        //let type_name = obj_get_str(self.get_manifest(), "type_name")?;
        let full_symbol_name = symbol_name.to_owned() + "_entry_point";
        type SymbolType = libloading::Symbol<'static, unsafe fn() -> unsafe fn(*mut CapsuleMap, *mut Capsule)->i64>;
        let f = unsafe {
            let symbol = self.load_symbol::<SymbolType>(full_symbol_name.as_bytes())?;
            (symbol)()
        };
        create_cpp_process_factory(self.get_manifest().clone(), f)
        //Ok(ProcessFactoryPtr::new(CppProcessFactoryImpl::new(self.get_manifest(), f)?))
    }


    pub fn load_container_factory(&self, _working_dir: Option<PathBuf>, symbol_name: &str) -> JuizResult<ContainerFactoryPtr> {
        log::trace!("CppPlugin({:?})::load_container_factory() called", self.path);
        let full_symbol_name = symbol_name.to_owned() + "_entry_point";
        type SymbolType = libloading::Symbol<'static, unsafe fn() -> unsafe fn(*mut Value, *mut *mut std::ffi::c_void)->i64>;
        let entry_point = unsafe {
            let symbol = self.load_symbol::<SymbolType>(full_symbol_name.as_bytes())?;
            (symbol)()
        };
        let container_manifest: ContainerManifest = self.get_manifest().clone().try_into()?;
        let container_manifest_clone = container_manifest.clone();
        let constructor = move |cm: ContainerManifest| -> JuizResult<ContainerPtr> {
            let mut pobj: *mut c_void = std::ptr::null_mut();
            unsafe {
                let symbol = entry_point.clone();
                let retval = (symbol)(&mut container_manifest_clone.build_instance_manifest(cm.clone())?.into(), &mut pobj);
                if retval < 0 || pobj == std::ptr::null_mut() {
                    return Err(anyhow::Error::from(JuizError::CppPluginFunctionCallError { function_name: "create_container".to_owned(), return_value: retval }));
                }
                Ok(ContainerPtr::new(ContainerImpl::new(cm, Box::new(CppContainerStruct{
                    cobj: pobj
                }))?))
            }
        };

        //container_factory_create_with_trait(container_manifest, constructor)
        Ok(ContainerFactoryPtr::new(ContainerFactoryImpl::new(container_manifest, Arc::new(constructor))?))
        
        //Ok(ContainerFactoryPtr::new(CppContainerFactoryImpl::new2(self.get_manifest().clone().try_into()?, f)?))
    }

    
    //pub fn load_container_factory_with_manifest(&self, working_dir: Option<PathBuf>, manifest: Value) -> JuizResult<Arc<Mutex<dyn ContainerFactory>>> {
    //    Ok(Arc::new(Mutex::new(PythonContainerFactoryImpl::new(
    //        manifest,
    //        working_dir.unwrap_or(env!("CARGO_MANIFEST_DIR").into()).join(self.path.clone())
    //    )?)))
   // }

    pub fn load_container_process_factory(&self, _working_dir: Option<PathBuf>, symbol_name: &str) -> JuizResult<ContainerProcessFactoryPtr> {
        log::trace!("CppPlugin({:?})::load_container_process_factory() called", self.path);
        let full_symbol_name = symbol_name.to_owned() + "_entry_point";
        type SymbolType = libloading::Symbol<'static, unsafe fn() -> unsafe fn(*mut std::ffi::c_void, *mut CapsuleMap, *mut Capsule) -> i64>;
        let entry_point = unsafe {
            let symbol = self.load_symbol::<SymbolType>(full_symbol_name.as_bytes())?;
            (symbol)()
        };
        let manifest: ProcessManifest = self.get_manifest().clone().try_into()?;
        let type_name = manifest.type_name.clone();

        let constructor = move |c: &mut ContainerImpl<CppContainerStruct>, mut argument: CapsuleMap| -> JuizResult<Capsule> {
            let mut retval = Capsule::empty();
            let return_value = unsafe {
                (entry_point)(c.t.cobj, &mut argument, &mut retval)
            };
            if return_value != 0 {
                return Err(anyhow::Error::from(JuizError::CppPluginFunctionCallError { function_name: format!("container_process({type_name})"), return_value }));
            }
            Ok(retval)
        };
        container_process_factory_create_from_trait(manifest, bind_container_function(constructor))
        //Ok(ContainerProcessFactoryPtr::new(CppContainerProcessFactoryImpl::new2(self.get_manifest().clone().try_into()?, f)?))
    }

    pub fn load_symbol<T>(&self, symbol_name: &[u8]) -> JuizResult<libloading::Symbol<T>> {
        log::trace!("CppPlugin::load_symbol({:?}) called", std::str::from_utf8(symbol_name));
        unsafe {
            match self.lib.get::<T>(symbol_name) {
                Ok(func) => Ok(func),
                Err(_) => {
                    log::error!("CppPlugin::load_symbol({:?}) failed.", std::str::from_utf8(symbol_name));
                    Err(anyhow::Error::from(JuizError::PluginLoadSymbolFailedError{plugin_path:self.path.display().to_string(), symbol_name: std::str::from_utf8(symbol_name)?.to_string()}))
                }
            }
        }
    }
}


fn create_cpp_process_factory(manifest: Value, entry_point: unsafe fn(*mut CapsuleMap, *mut Capsule) -> i64) -> JuizResult<ProcessFactoryPtr> {
    let entry_point_name = "process_entry_point".to_owned();
    let function = move |mut argument: CapsuleMap| -> JuizResult<Capsule> {
        log::trace!("cppfunc (argument={argument:?}) called");
        let mut func_result : Capsule = Capsule::empty();
        unsafe {
            let v = entry_point(&mut argument, &mut func_result);
            if v < 0 {
                return Err(anyhow::Error::from(JuizError::CppPluginFunctionCallError{function_name:entry_point_name.clone(), return_value:v}));
            } 
        }
        return Ok(func_result);
    };

    process_factory_create_from_trait(manifest.try_into()?, function)
}
