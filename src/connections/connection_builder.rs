

pub mod connection_builder {
    use std::sync::{Arc, Mutex};
    use anyhow::Context;

    use crate::{processes::Process, Value, System, utils::{get_value, get_str, juiz_lock}, JuizResult};

    pub fn create_connections(system: &System, manifest: &Value) -> JuizResult<Value> {
        log::trace!("connection_builder::create_connections(manifest={:?}) called", manifest);
        connect(
            system.any_process_from_manifest(
                get_value(manifest, "source")
                    .context("When loading 'source' value but not found in connection_builder::create_connectin()")?)
                .context("System::process_from_manifest(source) failed in connection_builder::create_connection()")?,
            system.any_process_from_manifest(
                get_value(manifest, "destination")
                    .context("When loading 'destination' value but not found in connection_builder::create_connectin()")?)
                .context("System::process_from_manifest(destination) failed in connection_builder::create_connection()")?,
            &get_str(manifest, "arg_name").context("When loading arg_name failed in connection_builder::create_connection()")?.to_string(),
            manifest.clone()
        )
    }
    
    pub fn connect(source: Arc<Mutex<dyn Process>>, destination: Arc<Mutex<dyn Process>>, arg_name: &String, manifest: Value) -> JuizResult<Value> {
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