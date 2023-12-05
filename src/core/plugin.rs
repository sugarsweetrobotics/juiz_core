
use libloading::Library;
use std::path::Path;

use crate::{JuizError, JuizResult};

pub struct Plugin {
    lib: Library,
}

pub type Symbol<'lib, T> = libloading::Symbol<'lib, libloading::Symbol<'lib, T>>;

impl Plugin {

    pub unsafe fn load(path: &Path) -> JuizResult<Plugin> {
        log::debug!("Plugin::load({:?}) called", path);
        unsafe {
            match libloading::Library::new(path) {
                Ok(lib) => Ok(Plugin{lib}),
                Err(_) => {
                    log::error!("Plugin::load({:?}) failed.", path);
                    Err(JuizError::PluginLoadFailedError{})
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
                    Err(JuizError::PluginLoadFailedError{})
                }
            }
        }
    }

    
}