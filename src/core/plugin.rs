
use libloading::Library;
use std::path::{Path, PathBuf};

use crate::{jvalue, JuizError, JuizResult, Value};

pub struct Plugin {
    path: PathBuf,
    lib: Library,
}

pub type Symbol<'lib, T> = libloading::Symbol<'lib, libloading::Symbol<'lib, T>>;

impl Plugin {

    pub fn profile_full(&self) -> JuizResult<Value> {
        Ok(jvalue!({
            "path": self.path,
        }))
    }

    pub unsafe fn load(path: &Path) -> JuizResult<Plugin> {
        log::debug!("Plugin::load({:?}) called", path);
        unsafe {
            match libloading::Library::new(path) {
                Ok(lib) => Ok(Plugin{lib, path:PathBuf::from(path)}),
                Err(_) => {
                    log::error!("Plugin::load({:?}) failed.", path);
                    Err(anyhow::Error::from(JuizError::PluginLoadFailedError{plugin_path: path.display().to_string()}))
                }
            }
        }
    }

    pub unsafe fn load_symbol<T>(&self, symbol_name: &[u8]) -> JuizResult<libloading::Symbol<T>> {
        unsafe {
            match self.lib.get::<T>(symbol_name) {
                Ok(func) => Ok(func),
                Err(_) => {
                    log::error!("Plugin::load_symbol({:?}) failed.", symbol_name);
                    Err(anyhow::Error::from(JuizError::PluginLoadSymbolFailedError{plugin_path:self.path.display().to_string(), symbol_name: std::str::from_utf8(symbol_name)?.to_string()}))
                }
            }
        }
    }

    
}