
use juiz_sdk::anyhow::{self, anyhow, Context};
use crate::{containers::{ContainerFactoryWrapper, ContainerProcessFactoryWrapper}, core::system_builder::topics::{setup_publish_topic, setup_subscribe_topic}, plugin::JuizObjectPlugin, prelude::*};


pub(super) fn setup_container_factories(system: &System, manifest: &Value, option: &Value) -> JuizResult<()> {
    log::trace!("setup_container_factories({manifest}) called");
    for (name, v) in get_hashmap(manifest)?.iter() {
        log::debug!("ContainerFactory (name={:}) Loading...", name);
        setup_container_factory(system, name, v, option)?;
        log::debug!("ContainerFactory (name={:}) Fully Loaded", name);
    }
    log::trace!("setup_container_factories() exit");
    Ok(())
}


pub(super) fn setup_containers(system: &System, manifest: &Value) -> JuizResult<()> {
    log::trace!("setup_containers({manifest}) called");
    for container_manifest_value in get_array(manifest)?.iter() {
        let container_manifest: ContainerManifest = container_manifest_value.clone().try_into()?;
        log::debug!("Container ({:?}) Creating...", container_manifest);
        setup_container(system, container_manifest.clone(), container_manifest_value.clone().try_into()?)?;
        log::debug!("Container ({:?}) Fully Created", container_manifest);
    } 
    log::trace!("setup_containers() exit");
    Ok(())
}

fn setup_container_factory(system: &System, name: &String, container_profile: &Value, option: &Value) -> JuizResult<ContainerFactoryPtr> {

    log::trace!("setup_container_factory(name={name}, profile={container_profile}) called");
    let manifest_entry_point = "manifest";
    let result = match container_profile.as_object() {
        None => {
            log::error!("loading process_factories failed. Value is not object type. Invalid config.");
            Err(anyhow::Error::from(JuizError::InvalidSettingError{message: "loading process_factories failed. Value is not object type. Invalid config.".to_owned()}))
        },
        Some(obj) => {
            let language = obj.get("language").and_then(|v| { v.as_str() }).or(Some("rust")).unwrap();
            let ctr = register_container_factory(system, JuizObjectPlugin::new(language, name, container_profile, manifest_entry_point, option)?, "container_factory")?;
            log::info!("ContainerFactory ({name:}) Loaded");
            when_contains_do(container_profile, "processes", |container_process_profile_map| {
                for (cp_name, container_process_profile) in get_hashmap(container_process_profile_map)?.iter() {
                    log::debug!(" - ContainerProcessFactory ({cp_name:}) Loading...");
                    register_container_process_factory(system, JuizObjectPlugin::new(language, cp_name, container_process_profile, manifest_entry_point, option)?, "container_process_factory")?;
                    log::info!(" - ContainerProcessFactory ({cp_name:}) Loaded");
                }
                Ok(())
            })?;
            Ok(ctr)
        }
    };
    log::trace!("setup_container_factory(name={name}, profile={container_profile}) called");
    result
}

/// コンテナをセットアップする
/// 
/// 各コンテナを作成後に対応するコンテナプロセスを作成する。
/// 
fn setup_container(system: &System, container_manifest: ContainerManifest, container_argument: CapsuleMap) -> JuizResult<()> {
    log::trace!("setup_container({container_manifest:?}) called");
    let type_name = container_manifest.type_name;
    let container = system.core_broker().lock_mut()?.worker_mut().create_container_ref(type_name.as_str(), container_argument)?;
    log::info!("Container Created");    
    for container_process_manifest in container_manifest.processes.iter() {
        log::debug!(" - ContainerProcess ({:?}) Creating...", container_process_manifest);
        let cp_ref = system.core_broker().lock_mut()?.worker_mut().create_container_process_ref(container.clone(), container_process_manifest.clone())?;
        log::info!(" - ContainerProcess ({:?}) Created", container_process_manifest);    
        // Topicをpublishするなら
        for pub_topic in container_process_manifest.publishes.iter() {
            setup_publish_topic(system, cp_ref.clone(), pub_topic.clone())?
        }

        for (arg_name, sub_topic) in container_process_manifest.subscribes.iter() {
            setup_subscribe_topic(system, cp_ref.clone(), &arg_name, sub_topic.clone())?
        }
    }   
    log::trace!("setup_container() exit");
    Ok(())
}


pub(super) fn cleanup_containers(system: &mut System) -> JuizResult<()> {
    log::trace!("system_builder::cleanup_containers() called");
    let r = system.core_broker().lock_mut().and_then(|mut cb|{
        cb.worker_mut().store_mut().clear()
    });
    log::trace!("system_builder::cleanup_containers() exit");
    r
}



pub(super) fn register_container_factory(system: &System, plugin: JuizObjectPlugin, symbol_name: &str) -> JuizResult<ContainerFactoryPtr> {
    log::trace!("register_container_factory(symbol_name={symbol_name}) called");
    let pf = plugin.load_container_factory(system.get_working_dir(), symbol_name)?;
    let type_name = pf.lock().or_else(|e|{ Err(anyhow!(JuizError::ObjectLockError{target:e.to_string()}) ) })?.type_name().to_owned();
    let wrapper = ContainerFactoryPtr::new(ContainerFactoryWrapper::new(plugin, pf)?);
    let _result = system.core_broker().lock_mut()?.worker_mut().store_mut().containers.register_factory(type_name.as_str(), wrapper.clone());
    log::trace!("register_container_factory() exit");
    Ok(wrapper)
}


pub(super) fn register_container_process_factory(system: &System, plugin: JuizObjectPlugin, symbol_name: &str) -> JuizResult<ContainerProcessFactoryPtr> {
    log::trace!("register_container_process_factory(symbol_name={symbol_name}) called");
    let cpf = plugin.load_container_process_factory(system.get_working_dir(), symbol_name)?;
    let type_name = cpf.lock().or_else(|e| { Err(anyhow!(JuizError::ObjectLockError { target: e.to_string() }))})?.type_name().to_owned();
    let pfw = ContainerProcessFactoryPtr::new(ContainerProcessFactoryWrapper::new(plugin, cpf )?);
    system.core_broker().lock_mut()?.worker_mut().store_mut().container_processes.register_factory(type_name.as_str(), pfw.clone())?;
    log::trace!("register_container_process_factory() exit");
    Ok(pfw)
}


