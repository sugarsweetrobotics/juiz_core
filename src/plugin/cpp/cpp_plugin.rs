
use std::{path::PathBuf, sync::{Arc, Mutex}};
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

    pub fn load_process_factory(&self, _working_dir: Option<PathBuf>, symbol_name: &str) -> JuizResult<Arc<Mutex<dyn ProcessFactory>>> {
        //let type_name = obj_get_str(self.get_manifest(), "type_name")?;
        let full_symbol_name = symbol_name.to_owned() + "_entry_point";
        type SymbolType = libloading::Symbol<'static, unsafe fn() -> unsafe fn(*mut CapsuleMap, *mut Capsule)->i64>;
        let f = unsafe {
            let symbol = self.load_symbol::<SymbolType>(full_symbol_name.as_bytes())?;
            (symbol)()
        };
        Ok(Arc::new(Mutex::new(CppProcessFactoryImpl::new(self.get_manifest(), f)?)))
    }


    pub fn load_container_factory(&self, _working_dir: Option<PathBuf>, symbol_name: &str) -> JuizResult<Arc<Mutex<dyn ContainerFactory>>> {
        log::trace!("CppPlugin({:?})::load_container_factory() called", self.path);
        let full_symbol_name = symbol_name.to_owned() + "_entry_point";
        type SymbolType = libloading::Symbol<'static, unsafe fn() -> unsafe fn(*mut Value, *mut *mut std::ffi::c_void)->i64>;
        let f = unsafe {
            let symbol = self.load_symbol::<SymbolType>(full_symbol_name.as_bytes())?;
            (symbol)()
        };
        Ok(Arc::new(Mutex::new(CppContainerFactoryImpl::new2(self.get_manifest(), f)?)))
    }
    
    //pub fn load_container_factory_with_manifest(&self, working_dir: Option<PathBuf>, manifest: Value) -> JuizResult<Arc<Mutex<dyn ContainerFactory>>> {
    //    Ok(Arc::new(Mutex::new(PythonContainerFactoryImpl::new(
    //        manifest,
    //        working_dir.unwrap_or(env!("CARGO_MANIFEST_DIR").into()).join(self.path.clone())
    //    )?)))
   // }

    pub fn load_container_process_factory(&self, _working_dir: Option<PathBuf>, symbol_name: &str) -> JuizResult<Arc<Mutex<dyn ContainerProcessFactory>>> {
        log::trace!("CppPlugin({:?})::load_container_process_factory() called", self.path);

        let full_symbol_name = symbol_name.to_owned() + "_entry_point";
        type SymbolType = libloading::Symbol<'static, unsafe fn() -> unsafe fn(*mut std::ffi::c_void, *mut CapsuleMap, *mut Capsule) -> i64>;
        let f = unsafe {
            let symbol = self.load_symbol::<SymbolType>(full_symbol_name.as_bytes())?;
            (symbol)()
        };
        Ok(Arc::new(Mutex::new(CppContainerProcessFactoryImpl::new2(self.get_manifest(), f)?)))
    }

    //pub fn load_container_process_factory_with_manifest(&self, working_dir: Option<PathBuf>, manifest: Value) -> JuizResult<Arc<Mutex<dyn ContainerProcessFactory>>> {
    //    Ok(Arc::new(Mutex::new(PythonContainerProcessFactoryImpl::new(
    //        manifest,
    //        working_dir.unwrap_or(env!("CARGO_MANIFEST_DIR").into()).join(self.path.clone()),
    //    )?)))
    //}

    //pub fn load_process_factory(&mut self, working_dir: Option<PathBuf>, _symbol_name: &str) -> JuizResult<Arc<Mutex<dyn ProcessFactory>>> {
    //    log::trace!("CppPlugin({:?})::load_process_factory() called", self.path);
    //    self.load_process_factory_with_manifest(working_dir.clone(), self.manifest.clone())
    // }

    // pub unsafe fn get_cpp_process_symbol(&self, entry_point_name: &str) -> JuizResult<Symbol<extern "C" fn(*mut CapsuleMap, *mut Capsule) -> i64>> {
    //     Ok(self.lib.get(entry_point_name.as_bytes())?)
    // }
    
    // fn load_process_factory_with_manifest(&mut self, working_dir: Option<PathBuf>, manifest: Value) -> JuizResult<Arc<Mutex<dyn ProcessFactory>>> {
    //     let type_name = obj_get_str(&self.manifest, "type_name")?.to_owned();
    //     let entry_point_name = "process_entry_point";
    //     unsafe {
    //         //let bcc = self.lib.get(entry_point_name.as_bytes())?;
    //         /*
    //         let _entry_point: Symbol<extern "C" fn(*mut CapsuleMap, *mut Capsule) -> i64> = self.lib.get(entry_point_name.as_bytes())?;
    //         let entry_point_func = |mut cm: CapsuleMap| -> JuizResult<Capsule> {
    //             let mut cap = Capsule::empty();
    //             let v = _entry_point(&mut cm, &mut cap);
    //             return Ok(cap);
    //         };
    //         */
    //         //self.process_entry_point = Some(_entry_point);
    //         todo!();/*
    //         let procf = Arc::new(Mutex::new(CppProcessFactoryImpl::new(
    //             manifest,
    //             working_dir.unwrap_or(env!("CARGO_MANIFEST_DIR").into()).join(self.path.clone()),
    //            // _entry_point,
    //             self,
    //         )?));
            
    //         Ok())
    //         */
    //     }
        
    // }


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