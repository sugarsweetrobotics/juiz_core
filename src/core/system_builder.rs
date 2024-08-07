


//pub mod system_builder {
    use std::sync::{mpsc, Arc, Mutex};

    use anyhow::Context;
    use crate::{plugin::{concat_dirname, plugin_name_to_file_name, JuizObjectPlugin, RustPlugin}, prelude::*, value::obj_get_obj};
    use crate::{
        brokers::{broker_factories_wrapper::BrokerFactoriesWrapper, 
        http::{http_broker_factory, http_broker_proxy_factory},  
        ipc::{ipc_broker::create_ipc_broker_factory, ipc_broker_proxy::create_ipc_broker_proxy_factory}, 
        local::{local_broker::{create_local_broker_factory, BrokerSideSenderReceiverPair, ProxySideSenderReceiverPair}, 
        local_broker_proxy::create_local_broker_proxy_factory}, 
        local_broker::ByteSenderReceiverPair, BrokerFactory, BrokerProxy, BrokerProxyFactory}, 
        containers::{ContainerFactoryPtr, ContainerProcessFactoryPtr}, 
        processes::ProcessFactoryPtr, utils::sync_util::juiz_try_lock,
    };
    use crate::{
        connections::connection_builder::connection_builder, 
        containers::{container_factory_wrapper::ContainerFactoryWrapper, 
            container_process_factory_wrapper::ContainerProcessFactoryWrapper}, 
            ecs::{execution_context_holder::ExecutionContextHolder, execution_context_holder_factory::ExecutionContextHolderFactory, ExecutionContextFactory}, 
            processes::ProcessFactoryWrapper, 
            utils::{get_array, get_hashmap, juiz_lock, manifest_util::when_contains_do_mut, when_contains_do}, 
            value::{obj_get, obj_get_str}, System
    };

    pub fn setup_plugins(system: &mut System, manifest: &Value) -> JuizResult<()> {
        log::trace!("system_builder::setup_plugins({}) called", manifest);

        let _ = when_contains_do_mut(manifest, "broker_factories", |v| {
            setup_broker_factories(system, v).with_context(||format!("system_builder::setup_broker_factories(manifest={manifest:}) failed."))
        })?;
        let _ = when_contains_do(manifest, "process_factories", |v| {
            setup_process_factories(system, v).with_context(||format!("system_builder::setup_process_factories(manifest={manifest:}) failed."))
        })?;
        let _ = when_contains_do(manifest, "container_factories", |v| {
            setup_container_factories(system, v).with_context(||format!("system_builder::setup_container_factories(manifest={manifest:}) failed."))
        })?;
        let _ = when_contains_do_mut(manifest, "components", |v| {
            setup_components(system, v).with_context(||format!("system_builder::setup_component_factories(manifest={manifest:}) failed."))
        })?;
        let _ = when_contains_do(manifest, "ec_factories", |v| {
            setup_execution_context_factories(system, v).with_context(||format!("system_builder::setup_execution_context_factories(manifest={manifest:}) failed."))
        })?;
        
        log::trace!("system_builder::setup_plugins() exit");
        Ok(())
    }



    fn setup_process_factories(system: &System, manifest: &serde_json::Value) -> JuizResult<()> {
        log::trace!("system_builder::setup_process_factories() called");
        for (name, v) in get_hashmap(manifest)?.iter() {
            log::debug!("ProcessFactory (name={:}) Loading...", name);
            setup_process_factory(system, name, v).with_context(||{format!("setup_process_factory(name='{name:}')")})?;
            log::info!("ProcessFactory (name={:}) Loaded", name);
        }
        Ok(())
    }

    fn setup_container_factories(system: &System, manifest: &Value) -> JuizResult<()> {
        log::trace!("system_builder::setup_container_factories() called");
        for (name, v) in get_hashmap(manifest)?.iter() {
            log::debug!("ContainerFactory (name={:}) Loading...", name);
            setup_container_factory(system, name, v)?;
            log::debug!("ContainerFactory (name={:}) Fully Loaded", name);
        }
        Ok(())
    }

    fn setup_components(system: &System, manifest: &Value) -> JuizResult<()> {
        log::trace!("system_builder::setup_component_factories({manifest:?}) called");
        for (name, v) in get_hashmap(manifest)?.iter() {
            log::info!("Component (name={name:}) Loading...");
            setup_component(system, name, v)?;
            log::info!("Component (name={name:}) Fully Loaded")
        }
        Ok(())
    }

    fn setup_broker_factories(system: &mut System, manifest: &Value) -> JuizResult<()> {
        log::trace!("system_builder::setup_broker_factories() called");
        for (name, v) in get_hashmap(manifest)?.iter() {
            log::debug!("BrokerFactory (name={name:}) Loading...");
            setup_broker_factory(system, manifest, name, v)?;
            log::info!("BrokerFactory (name={name:}) Loaded");
        }
        Ok(())
    }
    pub fn setup_containers(system: &System, manifest: &Value) -> JuizResult<()> {
        log::trace!("system_builder::setup_containers() called");
        for container_manifest in get_array(manifest)?.iter() {
            let type_name =  obj_get_str(container_manifest, "type_name")?;
            let name =  obj_get_str(container_manifest, "name")?;
            log::debug!("Container ({:}:{:}) Creating...", name, type_name);
            setup_container(system, container_manifest)?;
            log::debug!("Container ({:}:{:}) Fully Created", name, type_name);
        } 
        Ok(())
    }
 
    /// ProcessFactoryをセットアップする。
    /// name: ProcessFactoryの型名
    /// v: manifest。languageタグがあれば、rust, pythonから分岐する。
    fn setup_process_factory(system: &System, name: &String, v: &Value) -> JuizResult<ProcessFactoryPtr> {
        log::trace!("system_builder::setup_process_factory({name:}, {v:}) called");
        let manifest_entry_point = "manifest";
        match v.as_object() {
            None => {
                log::error!("loading process_factories failed. Value is not object type. Invalid config.");
                Err(anyhow::Error::from(JuizError::InvalidSettingError{message: "loading process_factories failed. Value is not object type. Invalid config.".to_owned()}))
            },
            Some(obj) => {
                let language = obj.get("language").and_then(|v| { v.as_str() }).or(Some("rust")).unwrap();
                register_process_factory(system, JuizObjectPlugin::new(language, name, v, manifest_entry_point)?, "process_factory")
            }
        }
    }


    fn setup_container_factory(system: &System, name: &String, container_profile: &Value) -> JuizResult<ContainerFactoryPtr> {
        let manifest_entry_point = "manifest";
        match container_profile.as_object() {
            None => {
                log::error!("loading process_factories failed. Value is not object type. Invalid config.");
                Err(anyhow::Error::from(JuizError::InvalidSettingError{message: "loading process_factories failed. Value is not object type. Invalid config.".to_owned()}))
            },
            Some(obj) => {
                let language = obj.get("language").and_then(|v| { v.as_str() }).or(Some("rust")).unwrap();
                let ctr = register_container_factory(system, JuizObjectPlugin::new(language, name, container_profile, manifest_entry_point)?, "container_factory", container_profile)?;
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
        }
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
                register_container_factory(system, plugin.clone(), obj_get_str(container_profile, "factory")?, container_profile)?;
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

    fn setup_execution_context_factories(system: &System, manifest: &serde_json::Value) -> JuizResult<()> {
        log::trace!("system_builder::setup_execution_context_factories() called");
        for (name, value) in get_hashmap(manifest)?.iter() {
            log::debug!("ExecutionContextFactory (name={name:}) Loading...");
            let plugin_filename = concat_dirname(value, plugin_name_to_file_name(name))?;
            let cpf;
            unsafe {
                type ECFactorySymbolType<'a> = libloading::Symbol<'a, unsafe extern "Rust" fn() -> JuizResult<Arc<Mutex<dyn ExecutionContextFactory>>>>;
                let plugin: RustPlugin = RustPlugin::load(plugin_filename)?;
                {
                    let symbol = plugin.load_symbol::<ECFactorySymbolType>(b"execution_context_factory")?;
                    cpf = (symbol)().with_context(||format!("calling symbol 'execution_context_factory'. arg is {manifest:}"))?;
                    let _ccpf = juiz_lock(&cpf)?;
                }
                system.core_broker().lock().unwrap().store_mut().ecs.register_factory(ExecutionContextHolderFactory::new(plugin, cpf)?)?;
            }
            log::info!("ExecutionContextFactory (name={name:}) Loaded");
        }
        Ok(())
    }

    pub fn setup_http_broker_factory(system: &mut System) -> JuizResult<()> {
        log::trace!("system_builder::setup_http_broker_factory() called");
        let hbf = http_broker_factory(system.core_broker().clone())?;
        let hbpf = http_broker_proxy_factory()?;
        let _wrapper = system.register_broker_factories_wrapper(BrokerFactoriesWrapper::new(None, hbf, hbpf)?)?;
        Ok(())
    }

    pub fn setup_local_broker_factory(system: &mut System) -> JuizResult<()> {
        log::trace!("system_builder::setup_local_broker_factory() called");
        let (c_sender, c_receiver) = mpsc::channel::<CapsuleMap>();
        let (p_sender, p_receiver) = mpsc::channel::<CapsulePtr>();
        let (_c_b_sender, c_b_receiver) = mpsc::channel::<Vec<u8>>();
        let (p_b_sender, _p_b_receiver) = mpsc::channel::<Vec<u8>>();
        let lbf = create_local_broker_factory(system.core_broker().clone(), Arc::new(Mutex::new(BrokerSideSenderReceiverPair(p_sender, c_receiver))), Arc::new(Mutex::new(ByteSenderReceiverPair(p_b_sender, c_b_receiver))))?;
        let lbpf = create_local_broker_proxy_factory(Arc::new(Mutex::new(ProxySideSenderReceiverPair(c_sender, p_receiver))))?;
        //juiz_lock(system.core_broker())?.store_mut().broker_proxies.register_factory(lbpf.clone())?;
        let _wrapper = system.register_broker_factories_wrapper(BrokerFactoriesWrapper::new(None, lbf, lbpf)?)?;
        Ok(())
    }

    #[allow(unused)]
    pub fn setup_ipc_broker_factory(system: &mut System) -> JuizResult<()> {
        log::trace!("system_builder::setup_local_broker_factory() called");
        let lbf = create_ipc_broker_factory(system.core_broker().clone())?;
        let lbpf = create_ipc_broker_proxy_factory()?;
        //juiz_lock(system.core_broker())?.store_mut().broker_proxies.register_factory(lbpf.clone())?;
        let _wrapper = system.register_broker_factories_wrapper(BrokerFactoriesWrapper::new(None, lbf, lbpf)?)?;
        Ok(())
    }

    fn setup_broker_factory(system: &mut System, manifest: &Value, name: &String, v: &Value) -> JuizResult<()> {
        log::trace!("setup_broker_factory(name={name:}) called");
        let plugin_filename = concat_dirname(v, plugin_name_to_file_name(&name.to_string()))?;
        let bf;
        let bpf;
        unsafe {
            type BrokerFactorySymbolType<'a> = libloading::Symbol<'a, unsafe extern "Rust" fn(Arc<Mutex<dyn BrokerProxy>>) -> JuizResult<Arc<Mutex<dyn BrokerFactory>>>>;
            type BrokerProxyFactorySymbolType<'a> = libloading::Symbol<'a, unsafe extern "Rust" fn() -> JuizResult<Arc<Mutex<dyn BrokerProxyFactory>>>>;
            let plugin: RustPlugin = RustPlugin::load(plugin_filename)?;
            {
                let symbol_bf = plugin.load_symbol::<BrokerFactorySymbolType>(b"broker_factory")?;
                bf = (symbol_bf)(system.core_broker().clone()).with_context(||format!("calling symbol 'broker_factory'. arg is {manifest:}"))?;
                log::trace!("BrokerFactory (type_name={:?}) created.", juiz_lock(&bf)?.type_name());
                let symbol_bpf = plugin.load_symbol::<BrokerProxyFactorySymbolType>(b"broker_proxy_factory")?;
                bpf = (symbol_bpf)().with_context(||format!("calling symbol 'broker_proxy_factory'. arg is {manifest:}"))?;
                log::trace!("BrokerProxyFactory (type_name={:?}) created.", juiz_lock(&bpf)?.type_name());
            }
            system.register_broker_factories_wrapper(BrokerFactoriesWrapper::new(Some(plugin), bf, bpf)?)?;
        }
        Ok(())
    }


    pub fn setup_processes(system: &System, manifest: &Value) -> JuizResult<()> {
        log::trace!("system_builder::setup_processes() called");
        for p in get_array(manifest)?.iter() {
            let p_name = obj_get_str(p, "name")?;
            let p_type_name = obj_get_str(p, "type_name")?;
            log::debug!("Process ({:}:{:}) Creating...", p_name, p_type_name);
            juiz_lock(system.core_broker())?.create_process_ref(p.clone())?;
            log::info!("Process ({:}:{:}) Created", p_name, p_type_name);
        } 
        Ok(())
    }
    
    pub fn cleanup_processes(system: &mut System) -> JuizResult<()> {
        log::trace!("system_builder::cleanup_processes() called");
        let r = juiz_try_lock(system.core_broker()).and_then(|mut cb|{
            cb.store_mut().clear()
        });
        log::trace!("system_builder::cleanup_processes() exit");
        r
    }
    
    pub fn setup_connections(system: &System, manifest: &serde_json::Value) -> JuizResult<()> {
        log::trace!("system_builder::setup_connections() called");
        for c in get_array(manifest)?.iter() {
            let srcv = obj_get_obj(c, "source")?;
            let dstv = obj_get_obj(c, "destination")?;
            //let p_type_name = obj_get_str(c, "type_name")?;
            log::debug!("Connection ({:?}->{:?}) Creating...", srcv, dstv);
            connection_builder::create_connection(system, &c).context("connection_builder::create_connections faled in system_builder::setup_connections()")?;
            log::info!("Connection ({:?}->{:?}) Created", srcv, dstv);
        } 
        Ok(())
    }
    

    fn setup_container(system: &System, container_manifest: &Value) -> JuizResult<()> {
        let name = obj_get_str(container_manifest, "name")?;
        let type_name = obj_get_str(container_manifest, "type_name")?;
        let container = juiz_lock(system.core_broker())?.create_container_ref(container_manifest.clone())?;
        log::info!("Container ({:}:{:}) Created", name, type_name);            
        let _ = when_contains_do(container_manifest, "processes", |v| {
            for p in get_array(v)?.iter() {
                let cp_name = obj_get_str(p, "name")?;
                let cp_type_name = obj_get_str(p, "type_name")?;
                log::debug!(" - ContainerProcess ({:}:{:}) Creating...", cp_name, cp_type_name);
                juiz_lock(system.core_broker())?.create_container_process_ref(Arc::clone(&container), p.clone())?;
                log::info!(" - ContainerProcess ({:}:{:}) Created", cp_name, cp_type_name);            
            }
            Ok(())
        })?;
        Ok(())
    }

    pub fn setup_http_broker(system: &mut System, port_number: i64) -> JuizResult<()> {
        log::trace!("system_builder::setup_http_broker() called");
        let http_broker = system.create_broker(&jvalue!({
            "type_name": "http",
            "name": format!("0.0.0.0:{}", port_number),
            "host": "0.0.0.0",
            "port": port_number,
        })).context("system.create_broker() failed in system_builder::setup_http_broker()")?;
        system.register_broker(http_broker)?;
        log::info!("HTTPBroker Created");
        Ok(())
    }

    pub fn setup_local_broker(system: &mut System) -> JuizResult<()> {
        log::trace!("system_builder::setup_local_broker() called");
        let local_broker = system.create_broker(&jvalue!({
            "type_name": "local",
            "name": "local"
        })).context("system.create_broker() failed in system_builder::setup_local_broker()")?;
        system.register_broker(local_broker)?;
        log::info!("LocalBroker Created");
        Ok(())
    }

    #[allow(unused)]
    pub fn setup_ipc_broker(system: &mut System) -> JuizResult<()> {
        log::trace!("system_builder::setup_ipc_broker() called");
        let ipc_broker = system.create_broker(&jvalue!({
            "type_name": "ipc",
            "name": "ipc"
        })).context("system.create_broker() failed in system_builder::setup_ipc_broker()")?;
        system.register_broker(ipc_broker)?;
        log::info!("IpcBroker Created");
        Ok(())
    }

    pub fn setup_brokers(system: &mut System, manifest: &Value) -> JuizResult<()> {
        log::trace!("system_builder::setup_brokers() called");
        for p in get_array(manifest)?.iter() {
            let type_name = obj_get_str(p, "type_name")?;
            let name = obj_get_str(p, "name")?;
            let _ = system.create_broker(&p)?;
            log::info!("Broker({:}:{:}) Created", name, type_name);
        }
        Ok(())
    }

    pub fn setup_broker_proxies(system: &mut System, manifest: &Value) -> JuizResult<()> {
        log::trace!("system_builder::setup_broker_proxies() called");
        for p in get_array(manifest)?.iter() {
            let type_name = obj_get_str(p, "type_name")?;
            let name = obj_get_str(p, "name")?;
            let _ = system.create_broker_proxy(&p)?;
            log::info!("BrokerProxy({:}:{:}) Created", name, type_name);
        }
        Ok(())
    }

    
    pub fn cleanup_containers(system: &mut System) -> JuizResult<()> {
        log::trace!("system_builder::cleanup_containers() called");
        let r = juiz_try_lock(system.core_broker()).and_then(|mut cb|{
            cb.store_mut().clear()
        });
        log::trace!("system_builder::cleanup_containers() exit");
        r
    }



    pub fn cleanup_brokers(system: &mut System) -> JuizResult<()> {
        log::trace!("system_builder::cleanup_brokers() called");
        let r = juiz_try_lock(system.core_broker()).and_then(|mut cb|{
            cb.store_mut().clear()
        });
        system.cleanup_brokers()?;
        log::trace!("system_builder::cleanup_brokers() exit");
        r
    }

    pub fn setup_ecs(system: &mut System, manifest: &Value) -> JuizResult<()> {
        log::trace!("system_builder::setup_ecs({manifest}) called");
        for p in get_array(manifest)?.iter() {
            let name = obj_get_str(p, "name")?;
            let type_name = obj_get_str(p, "type_name")?;
            log::debug!("ExecutionContext ({:}:{:}) Creating...", name, type_name);
            let ec = juiz_lock(system.core_broker())?.create_ec_ref(p.clone())?;
            log::info!("ExecutionContext ({:}:{:}) Created", name, type_name);
            juiz_lock(&ec)?.on_load(system);
            match obj_get(p, "bind") {
                Err(_) => {},
                Ok(binds_obj) => {
                    for b in get_array(binds_obj)?.iter() {
                        setup_ec_bind(system, Arc::clone(&ec), b)?;
                    }
                }
            };
        } 
        Ok(())
    }

    pub fn cleanup_ecs(system: &System) -> JuizResult<()> {
        log::trace!("system_builder::cleanup_ecs() called");
        juiz_lock(system.core_broker())?.cleanup_ecs()
    }

    fn setup_ec_bind(system: &System, ec: Arc<Mutex<ExecutionContextHolder>>, bind_info: &Value) -> JuizResult<()> {
        log::trace!("system_builder::setup_ec_bind() called");
        let ec_prof = juiz_lock(&ec)?.profile_full()?;
        let ec_name = obj_get_str(&ec_prof, "name")?;
        let process_info = obj_get(bind_info, "target")?;
        let p_id = obj_get_str(&process_info, "identifier")?;
        log::trace!("EC({:}) -> Process({:})", ec_name, p_id);
        let target_process = system.any_process_from_manifest(process_info)?;
        let ret = juiz_lock(&ec)?.bind(target_process);

        log::info!("EC({:}) -> Process({:}) Bound", ec_name, p_id);
        ret
    }




    fn register_process_factory(system: &System, plugin: JuizObjectPlugin, symbol_name: &str) -> JuizResult<ProcessFactoryPtr> {
        log::trace!("register_python_process_factory() called");
        let pf = plugin.load_process_factory(system.get_working_dir(), symbol_name)?;
        system.core_broker().lock().unwrap().store_mut().processes.register_factory(ProcessFactoryWrapper::new(plugin, pf)?)
    }

    fn register_container_factory(system: &System, plugin: JuizObjectPlugin, symbol_name: &str, profile: &Value) -> JuizResult<ContainerFactoryPtr> {
        log::trace!("register_container_factory({symbol_name}, {profile}) called");
        let pf = plugin.load_container_factory(system.get_working_dir(), symbol_name, profile)?;
        system.core_broker().lock().unwrap().store_mut().containers.register_factory(ContainerFactoryWrapper::new(plugin, pf)?)
    }

    fn register_container_process_factory(system: &System, plugin: JuizObjectPlugin, symbol_name: &str, profile: &Value) -> JuizResult<ContainerProcessFactoryPtr> {
        log::trace!("register_container_process_factory(prof={profile:}) called");
        let cpf = plugin.load_container_process_factory(system.get_working_dir(), symbol_name, profile)?;
        system.core_broker().lock().unwrap().store_mut().container_processes.register_factory(ContainerProcessFactoryWrapper::new(plugin, cpf)?)
    }


