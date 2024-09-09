

pub mod connection_builder {

    use crate::{core::core_broker::CoreBroker, prelude::*};
    use std::{collections::HashMap, sync::Arc};
    use anyhow::Context;

    use crate::{processes::{proc_lock, proc_lock_mut}, utils::{get_str, get_value}};

    ///
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
    
    pub fn connect(source: ProcessPtr, destination: ProcessPtr, arg_name: &String, manifest: Value) -> JuizResult<Value> {
        log::debug!("connection_builder::connect({manifest}) called");
        let source_connect_result_manifest;
        {
            source_connect_result_manifest = proc_lock_mut(&source)?.try_connect_to(Arc::clone(&destination), arg_name, manifest)?.clone();
            log::trace!("source_connection, connected!");
        }
        let result = proc_lock_mut(&destination)?.notify_connected_from(source, arg_name, source_connect_result_manifest);
        log::trace!("destination_connection, connected!");
        result
    }

    pub fn list_connection_profiles(core_broker: &CoreBroker) -> JuizResult<Vec<Value>> {
        let mut value_map: HashMap<String, Value> = HashMap::new();
        for p in core_broker.store().processes.objects().into_iter() {
            for sc in proc_lock(p)?.source_connections()? {
                value_map.insert(sc.identifier().clone(), sc.profile_full()?.try_into()?);
            }
            for dc in proc_lock(p)?.destination_connections()? {
                value_map.insert(dc.identifier().clone(), dc.profile_full()?.try_into()?);
            }
        }
        Ok(value_map.values().map(|v|{v.clone()}).collect::<Vec<Value>>())
    }
}