
use anyhow::Context;

use crate::{core::system_builder::topics::{setup_publish_topic, setup_subscribe_topic}, plugin::JuizObjectPlugin, prelude::*, processes::ProcessFactoryWrapper, utils::{get_array, get_hashmap}};

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
    log::trace!("setup_processes({manifest}) called");
    for process_manifest_value  in get_array(manifest)?.iter() {
        let process_manifest: ProcessManifest = process_manifest_value.clone().try_into()?;
        log::debug!("Process ({:?}) Creating...", process_manifest);
        let new_process = system.core_broker().lock_mut()?.worker_mut().create_process_ref(process_manifest.clone())?;
        log::info!("Process ({:?}) Created", process_manifest);

        // Topicをpublishするなら
        for pub_topic in process_manifest.publishes.iter() {
            setup_publish_topic(system, new_process.clone(), pub_topic.clone())?
        }
        for (arg_name, sub_topic) in process_manifest.subscribes.iter() {
            setup_subscribe_topic(system, new_process.clone(), arg_name, sub_topic.clone())?
        }
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
    let pfw = ProcessFactoryPtr::new(ProcessFactoryWrapper::new(plugin, pf)?);
    system.core_broker().lock_mut()?.worker_mut().store_mut().processes.register_factory(type_name.as_str(), pfw.clone())?;
    log::trace!("register_process_factory() exit");
    Ok(pfw)
}