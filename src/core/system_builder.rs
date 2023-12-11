


pub mod system_builder {
    use std::{path::PathBuf, sync::{Mutex, Arc, mpsc}};

    use crate::{jvalue, System, Value, JuizResult, core::Plugin, ProcessFactory, processes::{ProcessFactoryWrapper, container_factory_wrapper::ContainerFactoryWrapper, container_process_factory_wrapper::ContainerProcessFactoryWrapper}, utils::{get_array, get_hashmap, when_contains_do}, 
    connections::connection_builder::connection_builder, utils::{juiz_lock, manifest_util::when_contains_do_mut}, value::obj_get_str, ContainerFactory, ContainerProcessFactory, BrokerFactory, BrokerProxyFactory, brokers::{broker_factories_wrapper::BrokerFactoriesWrapper, local_broker_factory::LocalBrokerFactory, local_broker_proxy_factory::LocalBrokerProxyFactory, local_broker::SenderReceiverPair, http_broker::HTTPBroker}};
 
    pub fn setup_plugins(system: &mut System, manifest: &Value) -> JuizResult<()> {
        log::trace!("system_builder::setup_plugins({}) called", manifest);

        let _ = when_contains_do_mut(manifest, "broker_factories", |v| {
            setup_broker_factories(system, v)
        })?;

        let _ = when_contains_do(manifest, "process_factories", |v| {
            setup_process_factories(system, v)
        })?;
        let _ = when_contains_do(manifest, "container_factories", |v| {
            setup_container_factories(system, v)
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
                    pf = (symbol)()?;
                    let ppf = juiz_lock(&pf)?;
                    log::debug!("ProcessFactory(type_name={:?}) created.", ppf.type_name());
                }
                system.core_broker().lock().unwrap().store_mut().register_process_factory(ProcessFactoryWrapper::new(plugin, pf))?;
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
                    cf = (symbol)()?;
                    let ccf = juiz_lock(&cf)?;
                    log::debug!("ContainerFactory(type_name={:?}) created.", ccf.type_name());
                }
                system.core_broker().lock().unwrap().store_mut().register_container_factory(ContainerFactoryWrapper::new(plugin, cf))?;
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
                    cpf = (symbol)()?;
                    let ccpf = juiz_lock(&cpf)?;
                    log::debug!("ContainerProcessFactory(type_name={:?}) created.", ccpf.type_name());
                }
                system.core_broker().lock().unwrap().store_mut().register_container_process_factory(ContainerProcessFactoryWrapper::new(plugin, cpf))?;
            }
        }
        Ok(())
    }
    

    fn setup_broker_factories(system: &mut System, manifest: &Value) -> JuizResult<()> {
        log::trace!("system_builder::setup_broker_factories() called");
        for (name, v) in get_hashmap(manifest)?.iter() {
            let plugin_filename = concat_dirname(v, plugin_name_to_file_name(name))?;
            let bf;
            let bpf;
            unsafe {
                type BrokerFactorySymbolType<'a> = libloading::Symbol<'a, unsafe extern "Rust" fn() -> JuizResult<Arc<Mutex<dyn BrokerFactory>>>>;
                type BrokerProxyFactorySymbolType<'a> = libloading::Symbol<'a, unsafe extern "Rust" fn() -> JuizResult<Arc<Mutex<dyn BrokerProxyFactory>>>>;
                let plugin: Plugin = Plugin::load(plugin_filename.as_path())?;
                {
                    // println!("!!!!!!!ContainerName: {}", (name.to_owned() + "::container_factory"));
                    //let symbol = plugin.load_symbol::<ContainerFactorySymbolType>((name.to_owned() + "::container_factory").as_bytes())?;
                    let symbol_bf = plugin.load_symbol::<BrokerFactorySymbolType>(b"broker_factory")?;
                    bf = (symbol_bf)()?;
                    log::debug!("BrokerFactory(type_name={:?}) created.", juiz_lock(&bf)?.type_name());
                    let symbol_bpf = plugin.load_symbol::<BrokerProxyFactorySymbolType>(b"broker_proxy_factory")?;
                    bpf = (symbol_bpf)()?;
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

    pub fn setup_local_broker_factory(system: &mut System) -> JuizResult<()> {
        log::debug!("system_builder::setup_local_broker_factory() called");
        let (c_sender, c_receiver) = mpsc::channel::<Value>();
        let (p_sender, p_receiver) = mpsc::channel::<Value>();

        let _wrapper = system.register_broker_factories_wrapper(BrokerFactoriesWrapper::new(None, 
            LocalBrokerFactory::new(system.core_broker().clone(), Arc::new(Mutex::new(SenderReceiverPair(p_sender, c_receiver))))?, 
            LocalBrokerProxyFactory::new(Arc::new(Mutex::new(SenderReceiverPair(c_sender, p_receiver)))
        )?)?)?;
        Ok(())
    }

    pub fn setup_local_broker(system: &mut System) -> JuizResult<()> {
        log::trace!("system_builder::setup_local_broker() called");
        let local_broker = system.create_broker(&jvalue!({
            "type_name": "local"
        }))?;
        system.register_broker(local_broker)?;

        let http_broker = HTTPBroker::new(system.core_broker().clone())?;

        system.register_broker(http_broker)?;
        Ok(())
    }

    pub fn setup_brokers(_system: &System, _manifest: &Value) -> JuizResult<()> {
        log::trace!("system_builder::setup_brokers() called");

        Ok(())
    }

    pub fn setup_connections(system: &System, manifest: &serde_json::Value) -> JuizResult<()> {
        log::trace!("system_builder::setup_connections() called");
        for c in get_array(manifest)?.iter() {
            connection_builder::create_connections(system, &c)?;
        } 
        Ok(())
    }
}