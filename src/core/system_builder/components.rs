

use anyhow::Context;

use crate::{core::system_builder::{containers::{register_container_factory, register_container_process_factory}, processes::register_process_factory}, plugin::JuizObjectPlugin, prelude::*, utils::{get_array, get_hashmap, when_contains_do}, value::obj_get_str};




pub(super) fn setup_components(system: &System, manifest: &Value) -> JuizResult<()> {
    log::trace!("system_builder::setup_component_factories({manifest:?}) called");
    for (name, v) in get_hashmap(manifest)?.iter() {
        log::info!("Component (name={name:}) Loading...");
        setup_component(system, name, v)?;
        log::info!("Component (name={name:}) Fully Loaded")
    }
    Ok(())
}


fn setup_component(system: &System, name: &String, v: &Value) -> JuizResult<()> {
    let manifest_entry_point = "component_profile";
    
    log::trace!("setup_component(name={:}, value={:}) called", name, v);
    let language = obj_get_str(v, "language").or::<JuizResult<&str>>(Ok("rust")).unwrap();
    let plugin = JuizObjectPlugin::new(language, name, v, manifest_entry_point)?;
    let component_profile = plugin.load_component_profile(system.get_working_dir())?;
    when_contains_do(&component_profile, "containers", |container_profiles| -> JuizResult<()> {
        for container_profile in get_array(container_profiles)?.iter() {
            let container_type_name = obj_get_str(container_profile, "type_name")?;
            log::debug!(" - ContainerFactory ({container_type_name:}) Loading...");
            register_container_factory(system, plugin.clone(), obj_get_str(container_profile, "factory")?, container_profile.clone())?;
            log::info!(" - ContainerFactory ({container_type_name:}) Loaded");
            when_contains_do(container_profile, "processes", |container_process_profiles| {
                for container_process_profile in get_array(container_process_profiles)?.iter() {
                    let container_process_type_name = obj_get_str(container_process_profile, "type_name")?;
                    log::debug!(" - ContainerProcessFactory ({container_process_type_name:}:{container_type_name}, prof={container_process_profile:}) Loading...");
                    register_container_process_factory(system, plugin.clone(), obj_get_str(container_process_profile, "factory")?, container_process_profile)?;
                    log::info!(" - ContainerProcessFactory ({container_process_type_name:}:{container_type_name}) Loaded");
                    
                }
                return Ok(());
            })?;
            log::debug!(" - Container ({container_type_name:}) Fully Loaded");
        }
        Ok(())
    })?;

    when_contains_do(&component_profile, "processes", |process_profiles| -> JuizResult<()> {
        for process_profile in get_array(process_profiles)?.iter() {
            let process_type_name = obj_get_str(process_profile, "type_name")?;
            log::debug!(" - ProcessFactory ({process_type_name:}) Loading...");
            register_process_factory(system, plugin.clone(), obj_get_str(process_profile, "factory")?).context("ProcessFactoryWrapper::new()")?;

            log::info!(" - ProcessFactory ({process_type_name:}) Loaded"); 
        }
        Ok(())
    })?;
    Ok(())
}

