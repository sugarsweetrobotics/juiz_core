
use anyhow::Context;
use libloading::Library;
use std::{path::PathBuf, sync::{Arc, Mutex}};

use crate::prelude::*;
use crate::prelude::ProcessFactoryPtr;
use crate::plugin::Plugin;
// use super::plugin::Plugin;

pub trait PluginManager {

}

#[allow(unused)]
pub struct RustPlugin {
    path: PathBuf,
    plugin_manager: Option<Arc<Mutex<dyn PluginManager>>>,
    lib: Option<Library>,
}


impl RustPlugin {

    pub fn load(path: PathBuf) -> JuizResult<RustPlugin> {
        log::trace!("RustPlugin::load({:?}) called", path);
        unsafe {
            match libloading::Library::new(path.clone()) {
                Ok(lib) => {
                    type FunctionType = libloading::Symbol<'static, unsafe extern "Rust" fn() -> Arc<Mutex<dyn PluginManager>> >;
                    let symbol_name = "plugin_manager";
                    let plugin_manager = match lib.get::<FunctionType>(symbol_name.as_bytes()) {
                        Ok(func) => {
                            log::info!("PluginManager is loaded.");
                            Some(func())
                        },
                        Err(_) => None
                    };
                
                    log::debug!("RustPlugin::load({:?}) loaded", path);
                    Ok(RustPlugin{lib:Some(lib), 
                        plugin_manager,
                        path})
                },
                Err(_) => {
                    log::error!("RustPlugin::load({:?}) failed.", path);
                    Err(anyhow::Error::from(JuizError::PluginLoadFailedError{plugin_path: path.display().to_string()}))
                }
            }
        }
    }

    pub fn load_symbol<T>(&self, symbol_name: &[u8]) -> JuizResult<libloading::Symbol<T>> {
        log::trace!("RustPlugin::load_symbol({:?}) called", std::str::from_utf8(symbol_name));
        unsafe {
            match self.lib.as_ref().unwrap().get::<T>(symbol_name) {
                Ok(func) => Ok(func),
                Err(_) => {
                    log::error!("RustPlugin::load_symbol({:?}) failed.", symbol_name);
                    Err(anyhow::Error::from(JuizError::PluginLoadSymbolFailedError{plugin_path:self.path.display().to_string(), symbol_name: std::str::from_utf8(symbol_name)?.to_string()}))
                }
            }
        }
    }

    pub fn load_process_factory(&self, _working_dir: Option<PathBuf>, symbol_name: &str) -> JuizResult<Arc<Mutex<dyn ProcessFactory>>> {
        type SymbolType = libloading::Symbol<'static, unsafe extern "Rust" fn() -> JuizResult<ProcessFactoryPtr>>;
        unsafe {
            let symbol = self.load_symbol::<SymbolType>(symbol_name.as_bytes())?;
            (symbol)().with_context(||format!("calling symbol '{symbol_name}'"))
        }
    }

    pub fn load_component_profile(&self) -> JuizResult<Value> {
        type ComponentProfileFunctionSymbolType<'a> = libloading::Symbol<'a, unsafe extern "Rust" fn() -> Value>;
        let symbol = self.load_symbol::<ComponentProfileFunctionSymbolType>(b"component_profile")?;
        Ok(unsafe {
             (symbol)()//.with_context(||format!("calling symbol 'container_factory'. arg is {manifest:}"))?;
        })
    }

    pub fn load_broker_factory(&self, system: &mut System, ) -> JuizResult<Arc<Mutex<dyn BrokerFactory>>> {
        //log::trace!("BrokerFactory (type_name={:?}) created.", juiz_lock(&bf)?.type_name());
        type BrokerFactorySymbolType<'a> = libloading::Symbol<'a, unsafe extern "Rust" fn(CoreBrokerPtr) -> JuizResult<Arc<Mutex<dyn BrokerFactory>>>>;
        let symbol_bf = self.load_symbol::<BrokerFactorySymbolType>(b"broker_factory")?;
        unsafe {
            (symbol_bf)(system.core_broker().clone())
        }
    }

    pub fn load_broker_proxy_factory(&self, _system: &mut System,) -> JuizResult<Arc<Mutex<dyn BrokerProxyFactory>>> {
        type BrokerProxyFactorySymbolType<'a> = libloading::Symbol<'a, unsafe extern "Rust" fn() -> JuizResult<Arc<Mutex<dyn BrokerProxyFactory>>>>;
        let symbol_bpf = self.load_symbol::<BrokerProxyFactorySymbolType>(b"broker_proxy_factory")?;
        unsafe {
            (symbol_bpf)()
        }
    }
    
}

impl Plugin for RustPlugin {
    fn profile_full(&self) -> JuizResult<Value> {
        Ok(jvalue!({
            "path": self.path,
        }))
    }
}

impl Drop for RustPlugin {
    fn drop(&mut self) {
        log::info!("RustPlugin({})::drop() called", self.path.display());
    }
}