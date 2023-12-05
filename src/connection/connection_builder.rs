

pub mod connection_builder {
    use std::sync::{Arc, Mutex};
    use crate::{process::Process, JuizError, Value, System, manifest_util::{get_value, get_str}, sync_util::juiz_lock, connection::source_connection};

    pub fn create_connections(system: &System, manifest: &Value) -> Result<Value, JuizError> {
        log::trace!("connection_builder::create_connections(manifest={:?}) called", manifest);
        connect(
            system.process_from_manifest(get_value(manifest, "source")?)?,
            system.process_from_manifest(get_value(manifest, "destination")?)?,
            &get_str(manifest, "arg_name")?.to_string(),
            manifest.clone()
        )
    }
    
    pub fn connect(source: Arc<Mutex<dyn Process>>, destination: Arc<Mutex<dyn Process>>, arg_name: &String, manifest: Value) -> Result<Value, JuizError> {
        log::trace!("connection_builder::connect() called");
        let source_connect_result_manifest;
        {
            source_connect_result_manifest = juiz_lock(&source)?.connection_to(Arc::clone(&destination), arg_name, manifest)?.clone();
            log::trace!("source_connection, connected!");
        }
        let result = juiz_lock(&destination)?.connected_from(source, arg_name, source_connect_result_manifest);
        log::trace!("destination_connection, connected!");
        Ok(result.expect("destination_connection_failed."))
    }
}