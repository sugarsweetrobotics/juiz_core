use crate::prelude::*;

use anyhow::anyhow;




pub(super) fn setup_subscribe_topic(system: &System, process: ProcessPtr, arg_name: &String, sub_topic_info: &Value) -> JuizResult<()> {
    log::trace!("setup_subscrie_topic(process, {arg_name}, {sub_topic_info:}) called");
    if let Some(_topic_name) = sub_topic_info.as_str() {
        let r = system.core_broker().lock_mut().and_then(|mut cb| {
            let _id = process.read()
            .and_then(|p|{ Ok(p.identifier().clone()) })
            .or_else(|_e|{Err(anyhow!(JuizError::ObjectLockError{target: "Process".to_owned()}))})?;
            cb.process_subscribe_topic(process, arg_name, sub_topic_info)
        } );
        r
    } else {
        log::error!("Error in s etup_subscribe_topic() function. topic_info is invalid type. {sub_topic_info}");
        Ok(())
    }
}

pub(super) fn setup_publish_topic(system: &System, process: ProcessPtr, pub_topic_info: &Value) -> JuizResult<()> {
    log::trace!("setup_publish_topic(process, {pub_topic_info:}) called");
    if let Some(_topic_name) = pub_topic_info.as_str() {
        let r = system.core_broker().lock_mut().and_then(|mut cb| {
            let _id = process.read()
            .and_then(|p|{ Ok(p.identifier().clone()) })
            .or_else(|_e|{Err(anyhow!(JuizError::ObjectLockError{target: "Process".to_owned()}))})?;
            cb.process_publish_topic(process, pub_topic_info)
        } );
        r
    } else {
        log::error!("Error in setup_publish_topic() function. topic_info is invalid type. {pub_topic_info}");
        Ok(())
    }
}