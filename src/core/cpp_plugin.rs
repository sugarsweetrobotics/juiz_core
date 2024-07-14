
use std::{path::PathBuf, sync::{Arc, Mutex}};
use libloading::{Library, Symbol};

use crate::{containers::{python_container_process_factory_impl::PythonContainerProcessFactoryImpl, PythonContainerFactoryImpl}, jvalue, processes::{cpp_process_factory_impl::CppProcessFactoryImpl, python_process_factory_impl::PythonProcessFactoryImpl}, value::obj_get_str, Capsule, CapsuleMap, CapsulePtr, ContainerFactory, ContainerProcessFactory, JuizError, JuizResult, ProcessFactory, Value};
type CppProcessEntryPointType = Symbol<'static, extern "C" fn(*mut CapsuleMap, *mut Capsule) -> i64>;
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
    
    pub fn new(path: PathBuf) -> JuizResult<CppPlugin> {
        log::trace!("CppPlugin::load({:?}) called", path);
        unsafe {
            let lib = Library::new(path.clone()).unwrap();
            
            let mut manif_cap = CapsulePtr::new();
            let manifest_function: Symbol<extern "C" fn(*mut CapsulePtr) -> i64> = lib.get(b"manifest_entry_point").unwrap();
            let retval = manifest_function(&mut manif_cap);
            if retval != 0 {
                return Err(anyhow::Error::from(JuizError::CppProcessFunctionCallError{}));
            }
            let manifest = manif_cap.extract_value()?;

            
            Ok(CppPlugin{path, lib, manifest})
        }
    }

    pub fn load_container_factory(&self, working_dir: Option<PathBuf>, _symbol_name: &str) -> JuizResult<Arc<Mutex<dyn ContainerFactory>>> {
        log::trace!("PythonPlugin({:?})::load_container_factory() called", self.path);
        self.load_container_factory_with_manifest(working_dir.clone(), self.get_manifest().clone())
    }
    
    pub fn load_container_factory_with_manifest(&self, working_dir: Option<PathBuf>, manifest: Value) -> JuizResult<Arc<Mutex<dyn ContainerFactory>>> {
        Ok(Arc::new(Mutex::new(PythonContainerFactoryImpl::new(
            manifest,
            working_dir.unwrap_or(env!("CARGO_MANIFEST_DIR").into()).join(self.path.clone())
        )?)))
    }

    pub fn load_container_process_factory(&self, working_dir: Option<PathBuf>, _symbol_name: &str) -> JuizResult<Arc<Mutex<dyn ContainerProcessFactory>>> {
        log::trace!("PythonPlugin({:?})::load_container_factory() called", self.path);
        self.load_container_process_factory_with_manifest(working_dir.clone(), self.get_manifest().clone())
    }

    pub fn load_container_process_factory_with_manifest(&self, working_dir: Option<PathBuf>, manifest: Value) -> JuizResult<Arc<Mutex<dyn ContainerProcessFactory>>> {
        Ok(Arc::new(Mutex::new(PythonContainerProcessFactoryImpl::new(
            manifest,
            working_dir.unwrap_or(env!("CARGO_MANIFEST_DIR").into()).join(self.path.clone()),
        )?)))
    }

    pub fn load_process_factory(&mut self, working_dir: Option<PathBuf>, _symbol_name: &str) -> JuizResult<Arc<Mutex<dyn ProcessFactory>>> {
        log::trace!("CppPlugin({:?})::load_process_factory() called", self.path);
        self.load_process_factory_with_manifest(working_dir.clone(), self.manifest.clone())
    }

    pub unsafe fn get_cpp_process_symbol(&self, entry_point_name: &str) -> JuizResult<Symbol<extern "C" fn(*mut CapsuleMap, *mut Capsule) -> i64>> {
        Ok(self.lib.get(entry_point_name.as_bytes())?)
    }

    fn load_process_factory_with_manifest(&mut self, working_dir: Option<PathBuf>, manifest: Value) -> JuizResult<Arc<Mutex<dyn ProcessFactory>>> {
        let type_name = obj_get_str(&self.manifest, "type_name")?.to_owned();
        let entry_point_name = "process_entry_point";
        unsafe {
            //let bcc = self.lib.get(entry_point_name.as_bytes())?;
            /*
            let _entry_point: Symbol<extern "C" fn(*mut CapsuleMap, *mut Capsule) -> i64> = self.lib.get(entry_point_name.as_bytes())?;
            let entry_point_func = |mut cm: CapsuleMap| -> JuizResult<Capsule> {
                let mut cap = Capsule::empty();
                let v = _entry_point(&mut cm, &mut cap);
                return Ok(cap);
            };
            */
            //self.process_entry_point = Some(_entry_point);
            todo!();/*
            let procf = Arc::new(Mutex::new(CppProcessFactoryImpl::new(
                manifest,
                working_dir.unwrap_or(env!("CARGO_MANIFEST_DIR").into()).join(self.path.clone()),
               // _entry_point,
                self,
            )?));
            
            Ok())
            */
        }
        
    }

    pub fn load_component_profile(&self, working_dir: Option<PathBuf>) -> JuizResult<Value> {
        Ok(self.get_manifest().clone())
    }


    pub fn load_symbol<T>(&self, symbol_name: &[u8]) -> JuizResult<libloading::Symbol<T>> {
        log::trace!("CppPlugin::load_symbol({:?}) called", std::str::from_utf8(symbol_name));
        unsafe {
            match self.lib.get::<T>(symbol_name) {
                Ok(func) => Ok(func),
                Err(_) => {
                    log::error!("CppPlugin::load_symbol({:?}) failed.", symbol_name);
                    Err(anyhow::Error::from(JuizError::PluginLoadSymbolFailedError{plugin_path:self.path.display().to_string(), symbol_name: std::str::from_utf8(symbol_name)?.to_string()}))
                }
            }
        }
    }
}