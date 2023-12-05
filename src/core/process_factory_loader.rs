use std::{path::Path, sync::{Arc, Mutex}, collections::HashMap};

use crate::{JuizResult, ProcessFactory, JuizError};

use super::Plugin;


pub struct ProcessFactoryLoader {
    loaded_plugins: HashMap<String, Plugin>,
}



impl ProcessFactoryLoader {

    pub fn new() -> ProcessFactoryLoader {
        ProcessFactoryLoader{loaded_plugins: HashMap::new()}
    }

    pub fn load_process_factory(&mut self, path: &Path) -> JuizResult<Arc<Mutex<dyn ProcessFactory>>> {
        unsafe {
            let plugin = Plugin::load(path)?;
            let symbol = plugin.
                load_symbol::<libloading::Symbol<unsafe extern fn() 
                    -> JuizResult<Arc<Mutex<dyn ProcessFactory>>>>>(b"process_factory")?;
            let pf = (symbol)()?;
            match pf.try_lock() {
                Err(_) => return Err(JuizError::ProcesssFactoryCanNotLockError{}),
                Ok(pff) => {
                    let type_name = pff.type_name();
                    self.loaded_plugins.insert(type_name.to_string(), plugin);
                }
            };
            Ok(pf)     
        }
    }
}