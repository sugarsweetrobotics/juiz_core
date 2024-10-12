

pub mod connection_builder {

    use crate::prelude::*;
    use std::collections::HashMap;
    use anyhow::{anyhow, Context};

    use crate::{processes::{proc_lock, proc_lock_mut}, utils::{get_str, get_value}};

    ///
    pub fn create_connection(system: &System, manifest: &Value) -> JuizResult<Value> {
        
        log::trace!("connection_builder::create_connections(manifest={:?}) called", manifest);
        connect(
            system.core_broker().lock_mut()?.worker_mut().any_process_from_manifest(
                get_value(manifest, "source")
                    .context("When loading 'source' value but not found in connection_builder::create_connectin()")?)
                .context("System::process_from_manifest(source) failed in connection_builder::create_connection()")?,
            system.core_broker().lock_mut()?.worker_mut().any_process_from_manifest(
                get_value(manifest, "destination")
                    .context("When loading 'destination' value but not found in connection_builder::create_connectin()")?)
                .context("System::process_from_manifest(destination) failed in connection_builder::create_connection()")?,
            &get_str(manifest, "arg_name").context("When loading arg_name failed in connection_builder::create_connection()")?.to_string(),
            manifest.clone()
        )
    }
    
    pub fn connect(src: ProcessPtr, dst: ProcessPtr, arg_name: &String, manifest: Value) -> JuizResult<Value> {
        log::debug!("connection_builder::connect({manifest}) called");
        let src_manifest = match proc_lock_mut(&src)?.try_connect_to(dst.clone(), arg_name, manifest) {
            Ok(manif) => {
                log::trace!("source_connection, connected!");
                Ok(manif)
            }
            Err(e) => {
                log::error!("Process(src).try_connect_to() failed. Error({e})");
                Err(anyhow!(e))
            }
        }?;
        match proc_lock_mut(&dst)?.notify_connected_from(src, arg_name, src_manifest) {
            Ok(result) => {
                log::trace!("destination_connection, connected!");
                Ok(result)
            }
            Err(e) => {
                log::error!("Process(dist).notify_connected_from() failed. Error({e})");
                Err(anyhow!(e))
            }
        }
    }

    pub fn list_connection_profiles(core_broker: &CoreBroker) -> JuizResult<Vec<Value>> {
        let mut value_map: HashMap<String, Value> = HashMap::new();
        for p in core_broker.worker().store().processes.objects().into_iter() {
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