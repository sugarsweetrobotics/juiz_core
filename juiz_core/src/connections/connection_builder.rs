

pub mod connection_builder {

    use crate::prelude::*;
    use std::collections::HashMap;
    use juiz_sdk::anyhow::{anyhow, Context};

    ///
    pub fn create_connection(system: &System, manifest: &Value) -> JuizResult<Value> {
        
        log::trace!("connection_builder::create_connection(manifest={:?}) called", manifest);
        let src = system.core_broker().lock()?.worker().any_process_from_manifest(
            get_value(manifest, "source")?)?;
        let dst = system.core_broker().lock()?.worker().any_process_from_manifest(
            get_value(manifest, "destination")?)?;
        let arg_name =  get_str(manifest, "arg_name")?.to_owned();
        
        connect(
            src, dst, &arg_name,
            manifest.clone()
        ).context("connection_builder::connect()")
    }
    
    pub fn connect(src: ProcessPtr, dst: ProcessPtr, arg_name: &String, manifest: Value) -> JuizResult<Value> {
        log::trace!("connection_builder::connect({manifest}) called");
        let src_manifest = match src.lock_mut()?.try_connect_to(dst.clone(), arg_name, manifest) {
            Ok(manif) => {
                log::trace!("source_connection, connected!");
                Ok(manif)
            }
            Err(e) => {
                log::error!("Process(src).try_connect_to() failed. Error({e})");
                Err(anyhow!(e))
            }
        }?;
        match dst.lock_mut()?.notify_connected_from(src, arg_name, src_manifest) {
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
        for (_k, p) in core_broker.worker().store().processes.objects().into_iter() {
            for sc in p.lock()?.source_connections()? {
                value_map.insert(sc.identifier().clone(), sc.profile_full()?.try_into()?);
            }
            for dc in p.lock()?.destination_connections()? {
                value_map.insert(dc.identifier().clone(), dc.profile_full()?.try_into()?);
            }
        }
        Ok(value_map.values().map(|v|{v.clone()}).collect::<Vec<Value>>())
    }
}