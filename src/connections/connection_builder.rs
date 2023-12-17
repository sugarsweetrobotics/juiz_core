

pub mod connection_builder {
    use std::{sync::{Arc, Mutex}, collections::HashMap};
    use anyhow::Context;

    use crate::{processes::Process, Value, System, utils::{get_value, get_str, juiz_lock}, JuizResult, CoreBroker};

    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use juiz_core::connections::connection_builder::connection_builder::create_connection;
    ///
    /// assert_eq!(create_connection(system, manifest), );
    /// ```
    pub fn create_connection(system: &System, manifest: &Value) -> JuizResult<Value> {
        
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
        log::debug!("connection_builder::connect({manifest}) called");
        let source_connect_result_manifest;
        {
            source_connect_result_manifest = juiz_lock(&source)?.connection_to(Arc::clone(&destination), arg_name, manifest)?.clone();
            log::trace!("source_connection, connected!");
        }
        let result = juiz_lock(&destination)?.connected_from(source, arg_name, source_connect_result_manifest);
        log::trace!("destination_connection, connected!");
        Ok(result.expect("destination_connection_failed."))
    }

    pub fn list_connection_profiles(core_broker: &CoreBroker) -> JuizResult<Vec<Value>> {
        let mut value_map: HashMap<String, Value> = HashMap::new();
        for p in core_broker.store().processes.objects().into_iter() {
            for sc in juiz_lock(p)?.source_connections()? {
                value_map.insert(sc.identifier().clone(), sc.profile_full()?);
            }
            for dc in juiz_lock(p)?.destination_connections()? {
                value_map.insert(dc.identifier().clone(), dc.profile_full()?);
            }
        }
        Ok(value_map.values().map(|v|{v.clone()}).collect::<Vec<Value>>())
    }
}