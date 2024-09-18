use anyhow::anyhow;
use crate::{brokers::broker_proxy::SystemBrokerProxy, prelude::*};

pub(super) fn setup_subsystems(system: &System, manifest: &Value) -> JuizResult<()> { 
    match manifest.as_array() {
        Some(arr) => {
            for v in arr.iter() {
                setup_subsystem(system, v)?;
            }
        },
        None => {
            log::error!("setup_subsystem failed. Record 'subsystems' must be array type.");
            return Err(anyhow!(JuizError::InvalidValueError{message: "setup_subsystem failed. Record 'subsystems' must be array type.".to_owned()}));
        }
    }
    Ok(())
}

fn setup_subsystem(system: &System, manifest: &Value) -> JuizResult<()> { 
    let _ = juiz_lock(& system.core_broker())?.system_add_subsystem(manifest.clone())?;
    Ok(())
}