
use std::ffi::c_void;
use std::path::PathBuf;
use std::sync::Arc;
use libloading::{Library, Symbol};
use juiz_sdk::anyhow::{self, anyhow};
//use super::cpp_container_factory_impl::CppContainerFactoryImpl;
//use super::cpp_container_process_factory_impl::CppContainerProcessFactoryImpl;
//use crate::brokers::http::http_router::container;
use crate::containers::{bind_container_function, container_factory_create, container_process_factory_create_from_trait};
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

    /// CppPluginのコンストラクタ
    /// path:
    /// manifest_entry_point: ファイル自体のマニフェストのエントリーポイント
    pub fn new(path: PathBuf, manifest_entry_point: &str) -> JuizResult<CppPlugin> {
        log::trace!("CppPlugin::new({:?}, {:?}) called", path, manifest_entry_point);
        let entry_point = manifest_entry_point.to_owned() + "_entry_point";
        unsafe {
            let lib = Library::new(path.clone()).or_else(|e| {
                log::error!("Library::new({path:?}) failed. Error ({e:?})");
                Err(e)
            })?;
            let mut manif_cap = CapsulePtr::new();
            let manifest_function: Symbol<unsafe fn(*mut CapsulePtr) -> i64> = lib.get(entry_point.as_bytes()).or_else(|e| {
                log::error!("Library::new({path:?}) failed. Error ({e:?})");
                Err(e)
            })?;
            if manifest_function(&mut manif_cap) != 0 {
                return Err(anyhow::Error::from(JuizError::CppProcessFunctionCallError{}));
            }
            Ok(CppPlugin{path, lib, manifest: manif_cap.extract_value()?})
        }
    }

    /// CppPluginのプロファイル
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
    
    pub fn load_process_factory(&self, _working_dir: Option<PathBuf>, symbol_name: &str, type_name_opt: Option<&str>) -> JuizResult<ProcessFactoryPtr> {
        log::trace!("load_process_factory({symbol_name:}) called");
        let full_symbol_name = symbol_name.to_owned() + "_entry_point";
        type SymbolType = libloading::Symbol<'static, unsafe fn() -> unsafe fn(*mut CapsuleMap, *mut Capsule)->i64 >;
        let f = unsafe {
            let symbol = self.load_symbol::<SymbolType>(full_symbol_name.as_bytes())?;
            (symbol)()
        };
        let manifest = match type_name_opt {
            Some(type_name) => {
                let manif: ComponentManifest = self.get_manifest().clone().try_into()?;
                manif.processes.iter().find(|p| { p.type_name == type_name })
                   .ok_or(anyhow!(JuizError::ArgumentError { message: format!("ComponentManifest does not include process(type_name={type_name})") }))?.clone()
            }
            None => self.get_manifest().clone().try_into()?
        };
        create_cpp_process_factory(manifest.into(), f)
    }

    pub fn load_container_factory(&self, _working_dir: Option<PathBuf>, symbol_name: &str, type_name_opt: Option<&str>) -> JuizResult<ContainerFactoryPtr> {
        log::trace!("CppPlugin({:?})::load_container_factory({symbol_name}, {type_name_opt:?}) called", self.path);
        let full_symbol_name = symbol_name.to_owned() + "_entry_point";
        type SymbolType = libloading::Symbol<'static, unsafe fn() -> unsafe fn(*mut CapsuleMap, *mut *mut std::ffi::c_void)->i64>;
        let entry_point = unsafe {
            let symbol = self.load_symbol::<SymbolType>(full_symbol_name.as_bytes())?;
            (symbol)()
        };

        let container_manifest = match type_name_opt {
            Some(type_name) => {
                let manif: ComponentManifest = self.get_manifest().clone().try_into()?;
                manif.containers.iter().find(|p| { p.type_name == type_name })
                   .ok_or(anyhow!(JuizError::ArgumentError { message: format!("ComponentManifest does not include container(type_name={type_name})") }))?.clone()
            }
            None => self.get_manifest().clone().try_into()?
        };
        let constructor = move |cm: ContainerManifest, mut v: CapsuleMap| -> JuizResult<ContainerPtr> {
            let mut pobj: *mut c_void = std::ptr::null_mut();
            let retval = unsafe { (entry_point)(&mut v, &mut pobj) };
            if retval < 0 || pobj == std::ptr::null_mut() {
                return Err(anyhow::Error::from(JuizError::CppPluginFunctionCallError { function_name: "create_container".to_owned(), return_value: retval }));
            }
            Ok(ContainerPtr::new(ContainerImpl::new(cm, Box::new(CppContainerStruct{
                cobj: pobj
            }))?))
        };
        container_factory_create(container_manifest, Arc::new(constructor))
    }

    pub fn load_container_process_factory(&self, _working_dir: Option<PathBuf>, symbol_name: &str, type_name_opt: Option<&str>) -> JuizResult<ContainerProcessFactoryPtr> {
        log::trace!("CppPlugin({:?})::load_container_process_factory({symbol_name}, {type_name_opt:?}) called", self.path);
        let full_symbol_name = symbol_name.to_owned() + "_entry_point";
        type SymbolType = libloading::Symbol<'static, unsafe fn() -> unsafe fn(*mut std::ffi::c_void, *mut CapsuleMap, *mut Capsule) -> i64>;
        let entry_point = unsafe {
            let symbol = self.load_symbol::<SymbolType>(full_symbol_name.as_bytes())?;
            (symbol)()
        };
        let container_process_manifest: ProcessManifest = match type_name_opt {
            Some(type_name) => {
                find_container_process_from_component_manifest(self.get_manifest().clone().try_into()?, type_name)
            }
            None => self.get_manifest().clone().try_into()
        }?;
        let type_name = container_process_manifest.type_name.to_owned();
        let constructor = move |c: &mut ContainerImpl<CppContainerStruct>, mut argument: CapsuleMap| -> JuizResult<Capsule> {
            let mut retval = Capsule::empty();
            let return_value = unsafe { (entry_point)(c.t.cobj, &mut argument, &mut retval) };
            if return_value != 0 {
                return Err(anyhow::Error::from(JuizError::CppPluginFunctionCallError { function_name: format!("container_process({type_name})"), return_value }));
            }
            Ok(retval)
        };
        container_process_factory_create_from_trait(container_process_manifest.into(), bind_container_function(constructor))
        //Ok(ContainerProcessFactoryPtr::new(CppContainerProcessFactoryImpl::new2(self.get_manifest().clone().try_into()?, f)?))
    }

    pub fn load_symbol<T>(&self, symbol_name: &[u8]) -> JuizResult<libloading::Symbol<T>> {
        log::trace!("CppPlugin::load_symbol({:?}) called", std::str::from_utf8(symbol_name));
        unsafe {
            match self.lib.get::<T>(symbol_name) {
                Ok(func) => Ok(func),
                Err(e) => {
                    log::error!("CppPlugin::load_symbol({:?}) failed. Err({e:})", std::str::from_utf8(symbol_name));
                    Err(anyhow::Error::from(JuizError::PluginLoadSymbolFailedError{plugin_path:self.path.display().to_string(), symbol_name: std::str::from_utf8(symbol_name)?.to_string()}))
                }
            }
        }
    }
}


fn find_container_process_from_component_manifest(comp_manif: ComponentManifest, type_name: &str) -> JuizResult<ProcessManifest> {
    for c in comp_manif.containers.iter() {
        for p in c.processes.iter() {
            if p.type_name == type_name {
                return Ok(p.clone());
            }
        }
    }
    Err(anyhow!(JuizError::ArgumentError { message: format!("ComponentManifest does not include container(type_name={type_name})") }))
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
