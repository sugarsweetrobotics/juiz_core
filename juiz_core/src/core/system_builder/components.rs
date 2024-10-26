


use crate::{core::system_builder::{containers::{register_container_factory, register_container_process_factory}, processes::register_process_factory}, plugin::JuizObjectPlugin, prelude::*};




pub(super) fn setup_components(system: &System, manifest: &Value, option: &Value) -> JuizResult<()> {
    log::trace!("system_builder::setup_component_factories({manifest:?}) called");
    for (name, v) in get_hashmap(manifest)?.iter() {
        log::info!("Component (name={name:}) Loading...");
        setup_component(system, name, v, option)?;
        log::info!("Component (name={name:}) Fully Loaded")
    }
    Ok(())
}


fn setup_component(system: &System, name: &String, v: &Value, option: &Value) -> JuizResult<()> {
    let manifest_entry_point = "component_manifest";
    
    log::trace!("setup_component(name={:}, value={:}) called", name, v);
    let language = obj_get_str(v, "language").or::<JuizResult<&str>>(Ok("rust")).unwrap();
    let plugin = JuizObjectPlugin::new(language, name, v, manifest_entry_point, option)?;
    let component_manifest = plugin.load_component_manifest(system.get_working_dir())?;
    for container_profile in component_manifest.containers.iter() {
        log::debug!(" - ContainerFactory ({container_profile:?}) Loading...");
        register_container_factory(system, plugin.clone(), container_profile.factory.as_str())?;
        log::info!(" - ContainerFactory ({container_profile:?}) Loaded");
        for container_process_profile in container_profile.processes.iter() {
            log::debug!(" - ContainerProcessFactory ({container_process_profile:?}) Loading...");
            register_container_process_factory(system, plugin.clone(), container_process_profile.factory.as_str())?;
            log::debug!(" - ContainerProcessFactory ({container_process_profile:?}) Loaded");
            
        }
    }
    // when_contains_do(&component_manifest, "containers", |container_profiles| -> JuizResult<()> {
    //     for container_profile in get_array(container_profiles)?.iter() {
    //         let container_type_name = obj_get_str(container_profile, "type_name")?;
    //         log::debug!(" - ContainerFactory ({container_type_name:}) Loading...");
    //         register_container_factory(system, plugin.clone(), obj_get_str(container_profile, "factory")?, container_profile.clone())?;
    //         log::info!(" - ContainerFactory ({container_type_name:}) Loaded");
    //         when_contains_do(container_profile, "processes", |container_process_profiles| {
    //             for container_process_profile in get_array(container_process_profiles)?.iter() {
    //                 let container_process_type_name = obj_get_str(container_process_profile, "type_name")?;
    //                 log::debug!(" - ContainerProcessFactory ({container_process_type_name:}:{container_type_name}, prof={container_process_profile:}) Loading...");
    //                 register_container_process_factory(system, plugin.clone(), obj_get_str(container_process_profile, "factory")?, container_process_profile)?;
    //                 log::info!(" - ContainerProcessFactory ({container_process_type_name:}:{container_type_name}) Loaded");
                    
    //             }
    //             return Ok(());
    //         })?;
    //         log::debug!(" - Container ({container_type_name:}) Fully Loaded");
    //     }
    //     Ok(())
    // })?;

    for process_manifest in component_manifest.processes.iter() {
        log::debug!(" - ProcessFactory ({process_manifest:?}) Loading...");
        register_process_factory(system, plugin.clone(), process_manifest.factory.as_str())?;
        log::info!(" - ProcessFactory ({process_manifest:?}) Loaded"); 
    }
    // when_contains_do(&component_manifest, "processes", |process_profiles| -> JuizResult<()> {
    //     for process_profile in get_array(process_profiles)?.iter() {
    //         let process_type_name = obj_get_str(process_profile, "type_name")?;
    //         log::debug!(" - ProcessFactory ({process_type_name:}) Loading...");
    //         register_process_factory(system, plugin.clone(), obj_get_str(process_profile, "factory")?).context("ProcessFactoryWrapper::new()")?;

    //         log::info!(" - ProcessFactory ({process_type_name:}) Loaded"); 
    //     }
    //     Ok(())
    // })?;
    Ok(())
}

