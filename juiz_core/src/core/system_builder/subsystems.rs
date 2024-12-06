use juiz_sdk::anyhow::anyhow;
use crate::{brokers::broker_proxy::SystemBrokerProxy, prelude::*};

pub(super) fn setup_subsystems(system: &System, manifest: &Value) -> JuizResult<()> { 
    match manifest.as_array() {
        Some(arr) => {
            for v in arr.iter() {
                setup_subsystem(system, v).or_else(|e|{
                    log::error!("setup_subsystem(manifest={v}) failed. Error: {e:?}");
                    Err(e)
                })?
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
    system.core_broker().lock_mut()?.system_add_subsystem(manifest.clone()).or_else(|e| {
        log::error!("system.add_subsystem(manifest={manifest}) failed. Error: {e:?}");
        Err(e)
    })?;
    Ok(())
}

pub(super) fn setup_mastersystem(system: &System, manifest: &Value) -> JuizResult<()> { 
    log::debug!("setup_mastersystem({manifest}) called");
    // let broker_proxy = system.create_broker_proxy(manifest)?;
    // broker_proxy.lock().or_else(|e| { Err(anyhow!(JuizError::ObjectLockError { target: "core_broker".to_owned() }))})?
    //     .system_add_subsystem(manifest)
    let info = jvalue!({
        "subsystem": manifest
    });
    system.core_broker().lock_mut()?.system_add_mastersystem(info).or_else(|e| {
        log::error!("system.add_mastersystem(manifest={manifest}) failed. Error: {e:?}");
        Err(e)
    })?;
    Ok(())
}