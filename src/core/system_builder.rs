


pub mod system_builder {
    use std::{path::PathBuf, sync::{Mutex, Arc}};

    use crate::{System, Value, JuizResult, JuizError, core::Plugin, ProcessFactory, process::ProcessFactoryWrapper, manifest_util::{get_array, get_hashmap}, 
    Broker, connection::connection_builder::connection_builder, sync_util::juiz_lock, value::obj_get_str};
 


    pub fn setup_plugins(system: &System, manifest: &Value) -> JuizResult<()> {
        log::trace!("system_builder::setup_plugins() called");
        let plugins_hashmap = manifest.as_object().unwrap();
        if plugins_hashmap.contains_key("process_factories") {
            if !plugins_hashmap.get("process_factories").unwrap().is_object() { 
                return Err(JuizError::ManifestIsNotObjectError {  });
            }
            setup_process_factories(system, manifest.as_object().unwrap().get("process_factories").unwrap())?;
        }
        Ok(())
    }

    #[cfg(target_os = "macos")]
    fn plugin_name_to_file_name(name: &String) -> String {
        "lib".to_string() + name + ".dylib"
    }

    type ProcessFactorySymbolType<'a> = libloading::Symbol<'a, unsafe extern fn() -> JuizResult<Arc<Mutex<dyn ProcessFactory>>>>;

    fn setup_process_factories(system: &System, manifest: &serde_json::Value) -> JuizResult<()> {
        log::trace!("system_builder::setup_process_factories() called");
        for (name, v) in get_hashmap(manifest)?.iter() {
            let plugin_filename = concat_dirname(v, plugin_name_to_file_name(name))?;
            let pf;
            unsafe {
                let plugin: Plugin = Plugin::load(plugin_filename.as_path())?;
                {
                    let symbol = plugin.load_symbol::<ProcessFactorySymbolType>(b"process_factory")?;
                    pf = (symbol)()?;
                    let ppf = juiz_lock(&pf)?;
                    log::debug!("ProcessFactory(type_name={:?}) created.", ppf.type_name());
                }
                system.core_broker().lock().unwrap().push_process_factory(ProcessFactoryWrapper::new(plugin, pf))?;
            }
        }
        Ok(())
    }

    fn concat_dirname(v: &serde_json::Value, name: String) -> JuizResult<PathBuf> {
        Ok(PathBuf::from(obj_get_str(v, "path")?.to_string()).join(name))
    }

     
    pub fn setup_processes(system: &System, manifest: &Value) -> Result<(), JuizError> {
        log::trace!("system_builder::setup_processes() called");
        for p in get_array(manifest)?.iter() {
            juiz_lock(system.core_broker())?.create_process(p.clone())?;
        } 
        Ok(())
    }

    pub fn setup_connections(system: &System, manifest: &serde_json::Value) -> Result<(), JuizError> {
        log::trace!("system_builder::setup_connections() called");
        for c in get_array(manifest)?.iter() {
            connection_builder::create_connections(system, &c)?;
        } 
        Ok(())
    }
}