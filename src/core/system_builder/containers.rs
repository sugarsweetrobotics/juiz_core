

use std::sync::Arc;

use crate::{containers::{container_factory_wrapper::ContainerFactoryWrapper, container_process_factory_wrapper::ContainerProcessFactoryWrapper}, plugin::JuizObjectPlugin, prelude::*, utils::{get_array, get_hashmap, sync_util::juiz_try_lock, when_contains_do}, value::obj_get_str};


pub(super) fn setup_container_factories(system: &System, manifest: &Value) -> JuizResult<()> {
    log::trace!("setup_container_factories({manifest}) called");
    for (name, v) in get_hashmap(manifest)?.iter() {
        log::debug!("ContainerFactory (name={:}) Loading...", name);
        setup_container_factory(system, name, v)?;
        log::debug!("ContainerFactory (name={:}) Fully Loaded", name);
    }
    log::trace!("setup_container_factories() exit");
    Ok(())
}


pub(super) fn setup_containers(system: &System, manifest: &Value) -> JuizResult<()> {
    log::trace!("setup_containers({manifest}) called");
    for container_manifest in get_array(manifest)?.iter() {
        let type_name =  obj_get_str(container_manifest, "type_name")?;
        let name =  obj_get_str(container_manifest, "name")?;
        log::debug!("Container ({:}:{:}) Creating...", name, type_name);
        setup_container(system, container_manifest)?;
        log::debug!("Container ({:}:{:}) Fully Created", name, type_name);
    } 
    log::trace!("setup_containers() exit");
    Ok(())
}

fn setup_container_factory(system: &System, name: &String, container_profile: &Value) -> JuizResult<ContainerFactoryPtr> {

    log::trace!("setup_container_factory(name={name}, profile={container_profile}) called");
    let manifest_entry_point = "manifest";
    let result = match container_profile.as_object() {
        None => {
            log::error!("loading process_factories failed. Value is not object type. Invalid config.");
            Err(anyhow::Error::from(JuizError::InvalidSettingError{message: "loading process_factories failed. Value is not object type. Invalid config.".to_owned()}))
        },
        Some(obj) => {
            let language = obj.get("language").and_then(|v| { v.as_str() }).or(Some("rust")).unwrap();
            let ctr = register_container_factory(system, JuizObjectPlugin::new(language, name, container_profile, manifest_entry_point)?, "container_factory", container_profile.clone())?;
            log::info!("ContainerFactory ({name:}) Loaded");
            when_contains_do(container_profile, "processes", |container_process_profile_map| {
                for (cp_name, container_process_profile) in get_hashmap(container_process_profile_map)?.iter() {
                    log::debug!(" - ContainerProcessFactory ({cp_name:}) Loading...");
                    register_container_process_factory(system, JuizObjectPlugin::new(language, cp_name, container_process_profile, manifest_entry_point)?, "container_process_factory", container_process_profile)?;
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


fn setup_container(system: &System, container_manifest: &Value) -> JuizResult<()> {
    log::trace!("setup_container({container_manifest}) called");
    let name = obj_get_str(container_manifest, "name")?;
    let type_name = obj_get_str(container_manifest, "type_name")?;
    let container = system.core_broker().lock_mut()?.create_container_ref(container_manifest.clone())?;
    log::info!("Container ({:}:{:}) Created", name, type_name);            
    let _ = when_contains_do(container_manifest, "processes", |v| {
        for p in get_array(v)?.iter() {
            let cp_name = obj_get_str(p, "name")?;
            let cp_type_name = obj_get_str(p, "type_name")?;
            log::debug!(" - ContainerProcess ({:}:{:}) Creating...", cp_name, cp_type_name);
            system.core_broker().lock_mut()?.create_container_process_ref(Arc::clone(&container), p.clone())?;
            log::info!(" - ContainerProcess ({:}:{:}) Created", cp_name, cp_type_name);            
        }
        Ok(())
    })?;
    log::trace!("setup_container() exit");
    Ok(())
}
    
pub(super) fn cleanup_containers(system: &mut System) -> JuizResult<()> {
    log::trace!("system_builder::cleanup_containers() called");
    let r = system.core_broker().lock_mut().and_then(|mut cb|{
        cb.store_mut().clear()
    });
    log::trace!("system_builder::cleanup_containers() exit");
    r
}



pub(super) fn register_container_factory(system: &System, plugin: JuizObjectPlugin, symbol_name: &str, profile: Value) -> JuizResult<ContainerFactoryPtr> {
    log::trace!("register_container_factory(symbol_name={symbol_name}, profile={profile}) called");
    let pf = plugin.load_container_factory(system.get_working_dir(), symbol_name, profile)?;
    let result = system.core_broker().lock_mut()?.store_mut().containers.register_factory(ContainerFactoryWrapper::new(plugin, pf)?);
    log::trace!("register_container_factory() exit");
    result
}


pub(super) fn register_container_process_factory(system: &System, plugin: JuizObjectPlugin, symbol_name: &str, profile: &Value) -> JuizResult<ContainerProcessFactoryPtr> {
    log::trace!("register_container_process_factory(symbol_name={symbol_name}, profile={profile:}) called");
    let cpf = plugin.load_container_process_factory(system.get_working_dir(), symbol_name, profile)?;
    let result = system.core_broker().lock_mut()?.store_mut().container_processes.register_factory(ContainerProcessFactoryWrapper::new(plugin, cpf)?);
    log::trace!("register_container_process_factory() exit");
    result
}


