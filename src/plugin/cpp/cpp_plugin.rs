
use std::path::PathBuf;
use libloading::{Library, Symbol};

use super::cpp_container_factory_impl::CppContainerFactoryImpl;
use super::cpp_container_process_factory_impl::CppContainerProcessFactoryImpl;
use super::cpp_process_factory_impl::CppProcessFactoryImpl;
use crate::prelude::*;

pub struct CppPlugin{
    path: PathBuf,
    lib: Library,
    manifest: Value,
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

    pub fn load_component_profile(&self, _working_dir: Option<PathBuf>) -> JuizResult<Value> {
        Ok(self.get_manifest().clone())
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
        Ok(ProcessFactoryPtr::new(CppProcessFactoryImpl::new(self.get_manifest(), f)?))
    }


    pub fn load_container_factory(&self, _working_dir: Option<PathBuf>, symbol_name: &str) -> JuizResult<ContainerFactoryPtr> {
        log::trace!("CppPlugin({:?})::load_container_factory() called", self.path);
        let full_symbol_name = symbol_name.to_owned() + "_entry_point";
        type SymbolType = libloading::Symbol<'static, unsafe fn() -> unsafe fn(*mut Value, *mut *mut std::ffi::c_void)->i64>;
        let f = unsafe {
            let symbol = self.load_symbol::<SymbolType>(full_symbol_name.as_bytes())?;
            (symbol)()
        };
        Ok(ContainerFactoryPtr::new(CppContainerFactoryImpl::new2(self.get_manifest(), f)?))
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
        let f = unsafe {
            let symbol = self.load_symbol::<SymbolType>(full_symbol_name.as_bytes())?;
            (symbol)()
        };
        Ok(ContainerProcessFactoryPtr::new(CppContainerProcessFactoryImpl::new2(self.get_manifest(), f)?))
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