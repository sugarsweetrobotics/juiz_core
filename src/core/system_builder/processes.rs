
use anyhow::Context;

use crate::{core::system_builder::topics::{setup_publish_topic, setup_subscribe_topic}, plugin::JuizObjectPlugin, prelude::*, processes::ProcessFactoryWrapper, utils::{get_array, get_hashmap}, value::obj_get_str};

pub(super) fn setup_process_factories(system: &System, manifest: &serde_json::Value) -> JuizResult<()> {
    log::trace!("setup_process_factories({manifest:}) called");
    for (name, v) in get_hashmap(manifest)?.iter() {
        log::debug!("ProcessFactory (name={:}) Loading...", name);
        setup_process_factory(system, name, v).with_context(||{format!("setup_process_factory(name='{name:}')")})?;
        log::info!("ProcessFactory (name={:}) Loaded", name);
    }
    log::trace!("setup_process_factories() exit");
    Ok(())
}

/// ProcessFactoryをセットアップする。
/// name: ProcessFactoryの型名
/// v: manifest。languageタグがあれば、rust, pythonから分岐する。
fn setup_process_factory(system: &System, name: &String, v: &Value) -> JuizResult<ProcessFactoryPtr> {
    log::trace!("setup_process_factory({name:}, {v:}) called");
    let manifest_entry_point = "manifest";
    let result = match v.as_object() {
        None => {
            log::error!("loading process_factories failed. Value is not object type. Invalid config.");
            Err(anyhow::Error::from(JuizError::InvalidSettingError{message: "loading process_factories failed. Value is not object type. Invalid config.".to_owned()}))
        },
        Some(obj) => {
            let language = obj.get("language").and_then(|v| { v.as_str() }).or(Some("rust")).unwrap();
            register_process_factory(system, JuizObjectPlugin::new(language, name, v, manifest_entry_point)?, "process_factory")
        }
    };
    log::trace!("setup_process_factory() exit");
    result
}


pub(super) fn setup_processes(system: &System, manifest: &Value) -> JuizResult<()> {
    log::trace!("setup_processes() called");
    for process_manifest in get_array(manifest)?.iter() {
        let p_name = obj_get_str(process_manifest, "name")?;
        let p_type_name = obj_get_str(process_manifest, "type_name")?;
        log::debug!("Process ({:}:{:}) Creating...", p_name, p_type_name);
        let new_process = system.core_broker().lock_mut()?.worker_mut().create_process_ref(process_manifest.clone())?;
        log::info!("Process ({:}:{:}) Created", p_name, p_type_name);

        // Topicをpublishするなら
        let _reslt = obj_get_array(process_manifest, "publish").and_then(|pub_topics| {
            for pub_topic in pub_topics.iter() {
                setup_publish_topic(system, new_process.clone(), pub_topic)?
            };
            Ok(())
        });

        let _reslts = obj_get_obj(process_manifest, "subscribe").and_then(|sub_topic_map| {
            for (arg_name, topic_prof) in sub_topic_map.iter() {
                setup_subscribe_topic(system, new_process.clone(), arg_name, topic_prof)?;
            };
            Ok(())
        });

    } 
    log::trace!("setup_processes() exit");
    Ok(())
}

pub(super) fn cleanup_processes(system: &mut System) -> JuizResult<()> {
    log::trace!("cleanup_processes() called");
    let r = system.core_broker().lock_mut().and_then(|mut cb|{
        cb.worker_mut().store_mut().clear()
    });
    log::trace!("cleanup_processes() exit");
    r
}


pub(super) fn register_process_factory(system: &System, plugin: JuizObjectPlugin, symbol_name: &str) -> JuizResult<ProcessFactoryPtr> {
    log::trace!("register_process_factory() called");
    let pf = plugin.load_process_factory(system.get_working_dir(), symbol_name)?;
    let type_name = pf.lock().or_else(|e| { Err(JuizError::ObjectLockError { target: e.to_string() })})?.type_name().to_owned();
    let pfw = ProcessFactoryWrapper::new(plugin, pf)?;
    system.core_broker().lock_mut()?.worker_mut().store_mut().processes.register_factory(type_name.as_str(), pfw.clone())?;
    log::trace!("register_process_factory() exit");
    Ok(pfw)
}