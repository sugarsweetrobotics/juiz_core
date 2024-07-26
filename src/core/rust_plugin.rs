
use libloading::Library;
use std::path::PathBuf;

use crate::{jvalue, JuizError, JuizResult, Value};

use super::plugin::Plugin;

pub struct RustPlugin {
    path: PathBuf,
    lib: Option<Library>,
}

impl RustPlugin {

    pub fn load(path: PathBuf) -> JuizResult<RustPlugin> {
        log::trace!("RustPlugin::load({:?}) called", path);
        unsafe {
            match libloading::Library::new(path.clone()) {
                Ok(lib) => {
                    log::debug!("RustPlugin::load({:?}) loaded", path);
                    Ok(RustPlugin{lib:Some(lib), path})
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

    pub fn load_component_profile(&self) -> JuizResult<Value> {
        type ComponentProfileFunctionSymbolType<'a> = libloading::Symbol<'a, unsafe extern "Rust" fn() -> Value>;
        let symbol = self.load_symbol::<ComponentProfileFunctionSymbolType>(b"component_profile")?;
        Ok(unsafe {
             (symbol)()//.with_context(||format!("calling symbol 'container_factory'. arg is {manifest:}"))?;
        })
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