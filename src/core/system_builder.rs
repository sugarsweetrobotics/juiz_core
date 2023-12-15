


pub mod system_builder {
    use std::{path::PathBuf, sync::{Mutex, Arc, mpsc}};

    use anyhow::Context;

    use crate::{jvalue, 
        brokers::{BrokerFactory, BrokerProxyFactory, broker_factories_wrapper::BrokerFactoriesWrapper, 
        local::local_broker::{SenderReceiverPair, create_local_broker_factory}, 
            // http_broker::HTTPBroker, 
            local::local_broker_proxy::create_local_broker_proxy_factory, BrokerProxy}, 
            System, Value, JuizResult, core::Plugin, ProcessFactory, 
            processes::ProcessFactoryWrapper, 
            containers::{container_factory_wrapper::ContainerFactoryWrapper, container_process_factory_wrapper::ContainerProcessFactoryWrapper}, utils::{get_array, get_hashmap, when_contains_do, juiz_lock, manifest_util::when_contains_do_mut}, connections::connection_builder::connection_builder, value::{obj_get_str, obj_get}, ContainerFactory, ContainerProcessFactory, ecs::{ExecutionContextFactory, execution_context_holder::ExecutionContextHolder, execution_context_holder_factory::ExecutionContextHolderFactory}};
 
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
        let _ = when_contains_do(manifest, "ec_factories", |v| {
            setup_execution_context_factories(system, v).with_context(||format!("system_builder::setup_execution_context_factories(manifest={manifest:}) failed."))
        })?;
        Ok(())
    }

    #[cfg(target_os = "macos")]
    fn plugin_name_to_file_name(name: &String) -> String {
        "lib".to_string() + name + ".dylib"
    }

    fn concat_dirname(v: &serde_json::Value, name: String) -> JuizResult<PathBuf> {
        Ok(PathBuf::from(obj_get_str(v, "path")?.to_string()).join(name))
    }


    fn setup_process_factories(system: &System, manifest: &serde_json::Value) -> JuizResult<()> {
        log::trace!("system_builder::setup_process_factories() called");
        for (name, v) in get_hashmap(manifest)?.iter() {
            let plugin_filename = concat_dirname(v, plugin_name_to_file_name(name))?;
            let pf;
            unsafe {
                type ProcessFactorySymbolType<'a> = libloading::Symbol<'a, unsafe extern "Rust" fn() -> JuizResult<Arc<Mutex<dyn ProcessFactory>>>>;
                let plugin: Plugin = Plugin::load(plugin_filename.as_path())?;
                {
                    let symbol = plugin.load_symbol::<ProcessFactorySymbolType>(b"process_factory")?;
                    pf = (symbol)().with_context(||format!("calling symbol 'process_factory'. arg is {manifest:}"))?;
                    let ppf = juiz_lock(&pf)?;
                    log::debug!("ProcessFactory(type_name={:?}) created.", ppf.type_name());
                }
                system.core_broker().lock().unwrap().store_mut().processes.register_factory(ProcessFactoryWrapper::new(plugin, pf)?)?;
            }
        }
        Ok(())
    }

    fn setup_container_factories(system: &System, manifest: &Value) -> JuizResult<()> {
        log::trace!("system_builder::setup_container_factories() called");
        for (name, v) in get_hashmap(manifest)?.iter() {
            let plugin_filename = concat_dirname(v, plugin_name_to_file_name(name))?;
            let cf;
            unsafe {
                type ContainerFactorySymbolType<'a> = libloading::Symbol<'a, unsafe extern "Rust" fn() -> JuizResult<Arc<Mutex<dyn ContainerFactory>>>>;
                let plugin: Plugin = Plugin::load(plugin_filename.as_path())?;
                {
                    // println!("!!!!!!!ContainerName: {}", (name.to_owned() + "::container_factory"));
                    //let symbol = plugin.load_symbol::<ContainerFactorySymbolType>((name.to_owned() + "::container_factory").as_bytes())?;
                    let symbol = plugin.load_symbol::<ContainerFactorySymbolType>(b"container_factory")?;
                    cf = (symbol)().with_context(||format!("calling symbol 'container_factory'. arg is {manifest:}"))?;
                    let ccf = juiz_lock(&cf)?;
                    log::debug!("ContainerFactory(type_name={:?}) created.", ccf.type_name());
                }
                system.core_broker().lock().unwrap().store_mut().containers.register_factory(ContainerFactoryWrapper::new(plugin, cf).context("ContainerFactoryWrapper::new()")?)?;
                when_contains_do(v, "processes", |vv| {
                    setup_container_process_factories(system, vv)
                })?;
            }
        }
        Ok(())
    }

    fn setup_container_process_factories(system: &System, manifest: &serde_json::Value) -> JuizResult<()> {
        for (name, value) in get_hashmap(manifest)?.iter() {
            let plugin_filename = concat_dirname(value, plugin_name_to_file_name(name))?;
            let cpf;
            unsafe {
                type ContainerProcessFactorySymbolType<'a> = libloading::Symbol<'a, unsafe extern "Rust" fn() -> JuizResult<Arc<Mutex<dyn ContainerProcessFactory>>>>;
                let plugin: Plugin = Plugin::load(plugin_filename.as_path())?;
                {
                    let symbol = plugin.load_symbol::<ContainerProcessFactorySymbolType>(b"container_process_factory")?;
                    cpf = (symbol)().with_context(||format!("calling symbol 'container_process_factory'. arg is {manifest:}"))?;
                    let ccpf = juiz_lock(&cpf)?;
                    log::debug!("ContainerProcessFactory(type_name={:?}) created.", ccpf.type_name());
                }
                system.core_broker().lock().unwrap().store_mut().container_processes.register_factory(ContainerProcessFactoryWrapper::new(plugin, cpf)?)?;
            }
        }
        Ok(())
    }


    fn setup_execution_context_factories(system: &System, manifest: &serde_json::Value) -> JuizResult<()> {
        for (name, value) in get_hashmap(manifest)?.iter() {
            let plugin_filename = concat_dirname(value, plugin_name_to_file_name(name))?;
            let cpf;
            unsafe {
                type ECFactorySymbolType<'a> = libloading::Symbol<'a, unsafe extern "Rust" fn() -> JuizResult<Arc<Mutex<dyn ExecutionContextFactory>>>>;
                let plugin: Plugin = Plugin::load(plugin_filename.as_path())?;
                {
                    let symbol = plugin.load_symbol::<ECFactorySymbolType>(b"execution_context_factory")?;
                    cpf = (symbol)().with_context(||format!("calling symbol 'execution_context_factory'. arg is {manifest:}"))?;
                    let ccpf = juiz_lock(&cpf)?;
                    log::debug!("ExecutionContextFactory(type_name={:?}) created.", ccpf.type_name());
                }
                system.core_broker().lock().unwrap().store_mut().ecs.register_factory(ExecutionContextHolderFactory::new(plugin, cpf)?)?;
            }
        }
        Ok(())
    }

    pub fn setup_local_broker_factory(system: &mut System) -> JuizResult<()> {
        log::debug!("system_builder::setup_local_broker_factory() called");
        let (c_sender, c_receiver) = mpsc::channel::<Value>();
        let (p_sender, p_receiver) = mpsc::channel::<Value>();
        let lbf = create_local_broker_factory(system.core_broker().clone(), Arc::new(Mutex::new(SenderReceiverPair(p_sender, c_receiver))))?;
        let _wrapper = system.register_broker_factories_wrapper(BrokerFactoriesWrapper::new(None, 
            lbf, 
            create_local_broker_proxy_factory(Arc::new(Mutex::new(SenderReceiverPair(c_sender, p_receiver)))
        )?)?)?;
        Ok(())
    }

    fn setup_broker_factories(system: &mut System, manifest: &Value) -> JuizResult<()> {
        log::trace!("system_builder::setup_broker_factories() called");
        for (name, v) in get_hashmap(manifest)?.iter() {
            let plugin_filename = concat_dirname(v, plugin_name_to_file_name(name))?;
            let bf;
            let bpf;
            unsafe {
                type BrokerFactorySymbolType<'a> = libloading::Symbol<'a, unsafe extern "Rust" fn(Arc<Mutex<dyn BrokerProxy>>) -> JuizResult<Arc<Mutex<dyn BrokerFactory>>>>;
                type BrokerProxyFactorySymbolType<'a> = libloading::Symbol<'a, unsafe extern "Rust" fn() -> JuizResult<Arc<Mutex<dyn BrokerProxyFactory>>>>;
                let plugin: Plugin = Plugin::load(plugin_filename.as_path())?;
                {
                    // println!("!!!!!!!ContainerName: {}", (name.to_owned() + "::container_factory"));
                    //let symbol = plugin.load_symbol::<ContainerFactorySymbolType>((name.to_owned() + "::container_factory").as_bytes())?;
                    let symbol_bf = plugin.load_symbol::<BrokerFactorySymbolType>(b"broker_factory")?;
                    bf = (symbol_bf)(system.core_broker().clone()).with_context(||format!("calling symbol 'broker_factory'. arg is {manifest:}"))?;
                    log::debug!("BrokerFactory(type_name={:?}) created.", juiz_lock(&bf)?.type_name());
                    let symbol_bpf = plugin.load_symbol::<BrokerProxyFactorySymbolType>(b"broker_proxy_factory")?;
                    bpf = (symbol_bpf)().with_context(||format!("calling symbol 'broker_proxy_factory'. arg is {manifest:}"))?;
                    log::debug!("BrokerProxyFactory(type_name={:?}) created.", juiz_lock(&bpf)?.type_name());
                }
                system.register_broker_factories_wrapper(BrokerFactoriesWrapper::new(Some(plugin), bf, bpf)?)?;
            }
        }
        Ok(())
    }

     
    pub fn setup_processes(system: &System, manifest: &Value) -> JuizResult<()> {
        log::trace!("system_builder::setup_processes() called");
        for p in get_array(manifest)?.iter() {
            juiz_lock(system.core_broker())?.create_process_ref(p.clone())?;
        } 
        Ok(())
    }

    pub fn setup_containers(system: &System, manifest: &Value) -> JuizResult<()> {
        log::trace!("system_builder::setup_containers() called");
        for container_manifest in get_array(manifest)?.iter() {
            let container = juiz_lock(system.core_broker())?.create_container_ref(container_manifest.clone())?;
            let _ = when_contains_do(container_manifest, "processes", |v| {
                for p in get_array(v)?.iter() {
                    juiz_lock(system.core_broker())?.create_container_process_ref(Arc::clone(&container), p.clone())?;
                }
                Ok(())
            })?;
        } 
        Ok(())
    }

    

    pub fn setup_local_broker(system: &mut System) -> JuizResult<()> {
        log::trace!("system_builder::setup_local_broker() called");
        let local_broker = system.create_broker(&jvalue!({
            "type_name": "local",
            "name": "local"
        })).context("system.create_broker() failed in system_builder::setup_local_broker()")?;
        system.register_broker(local_broker)?;

        //let http_broker = HTTPBroker::new(
        //    system.core_broker().clone(), "http_broker_0")?;
        //system.register_broker(http_broker)?;
        Ok(())
    }

    pub fn setup_brokers(system: &mut System, manifest: &Value) -> JuizResult<()> {
        log::trace!("system_builder::setup_brokers() called");
        for p in get_array(manifest)?.iter() {
            let _ = system.create_broker(&p)?;
        }
        Ok(())
    }

    pub fn setup_ecs(system: &System, manifest: &Value) -> JuizResult<()> {
        log::trace!("system_builder::setup_ecs({manifest}) called");
        for p in get_array(manifest)?.iter() {
            let ec = juiz_lock(system.core_broker())?.create_ec_ref(p.clone())?;
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

    fn setup_ec_bind(system: &System, ec: Arc<Mutex<ExecutionContextHolder>>, bind_info: &Value) -> JuizResult<()> {
        let process_info = obj_get(bind_info, "target")?;
        let target_process = system.any_process_from_manifest(process_info)?;
        juiz_lock(&ec)?.bind(target_process)
    }

    pub fn setup_connections(system: &System, manifest: &serde_json::Value) -> JuizResult<()> {
        log::trace!("system_builder::setup_connections() called");
        for c in get_array(manifest)?.iter() {
            connection_builder::create_connections(system, &c).context("connection_builder::create_connections faled in system_builder::setup_connections()")?;
        } 
        Ok(())
    }
}