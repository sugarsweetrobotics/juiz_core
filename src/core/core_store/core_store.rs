use std::collections::HashMap;
use super::buffer_store_worker::BufferStoreWorker;
use super::rw_store_worker::RwStoreWorker;
use super::store_worker::StoreWorker;

use crate::{containers::container_process_impl::ContainerProcessImpl, prelude::*, value::obj_get_str};
use crate::ecs::{execution_context_function::ExecutionContextFunction, execution_context_holder_factory::ExecutionContextHolderFactory};





pub struct CoreStore {
    broker_factories_manifests: HashMap<Identifier, Value>,
    brokers_manifests: HashMap<Identifier, Value>,

    pub processes: Box<RwStoreWorker::<dyn Process, dyn ProcessFactory>>,
    pub containers: Box<RwStoreWorker::<dyn Container, dyn ContainerFactory>>,
    pub container_processes: Box<RwStoreWorker::<ContainerProcessImpl, dyn ContainerProcessFactory>>,
    pub ecs: Box<StoreWorker::<dyn ExecutionContextFunction, ExecutionContextHolderFactory>>,
    pub broker_proxies: Box<BufferStoreWorker::<dyn BrokerProxy, dyn BrokerProxyFactory>>,
}


impl CoreStore {
    pub fn new() -> CoreStore {
        CoreStore{
            brokers_manifests: HashMap::new(),
            broker_proxies: BufferStoreWorker::new("broker_proxy"),
            broker_factories_manifests: HashMap::new(),
            
            processes: RwStoreWorker::new("process"), 
            containers: RwStoreWorker::new("container"), 
            container_processes: RwStoreWorker::new("container_process"), 
            ecs: StoreWorker::new("ecs"),
        }
    }

    pub fn clear(&mut self) -> JuizResult<()> {
        log::trace!("CoreStore::clear() called");
        self.clear_container_process_factories()?;
        self.clear_container_factories()?;
        self.clear_process_factories()?;
        self.clear_broker_factories()?;
        Ok(())
    }
    
    fn clear_container_process_factories(&mut self) -> JuizResult<()> {
        log::trace!("clear_container_process_factories() called");
        self.container_processes.clear()?;
        Ok(())
    }
    
    
    fn clear_container_factories(&mut self) -> JuizResult<()> {
        log::trace!("clear_container_factories() called");
        self.containers.clear()?;
        Ok(())
    }

    fn clear_process_factories(&mut self) -> JuizResult<()> {
        log::trace!("clear_broker_factories() called");
        self.processes.clear()?;
        Ok(())
    }

    fn clear_broker_factories(&mut self) -> JuizResult<()> {
        log::trace!("clear_broker_factories() called");
        self.broker_factories_manifests.clear();
        self.brokers_manifests.clear();
        self.broker_proxies.clear()?;
        Ok(())
    }

    pub fn register_broker_manifest(&mut self, type_name: &str, b: Value) -> JuizResult<()> {
        self.brokers_manifests.insert(type_name.to_string(), b);
        Ok(())
    }

    pub fn register_broker_factory_manifest(&mut self, type_name: &str, b: Value) -> JuizResult<()> {
        log::trace!("core_store::register_broker_factory_manifest(type_name={type_name:?}) called");
        self.broker_factories_manifests.insert(type_name.to_string(), b);
        Ok(())
    }

    
    fn broker_factories_profile_full(&self) -> JuizResult<Value> {
        Ok(jvalue!(self.broker_factories_manifests))
    }

    pub fn broker_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        match self.brokers_manifests.get(id) {
            Some(p) => Ok(p.clone()),
            None => {
                Err(anyhow::Error::from(JuizError::BrokerProfileNotFoundError{id: id.to_string()}))
            }
        }
    }

    pub fn brokers_profile_full(&self) -> JuizResult<Value> {
        Ok(jvalue!(self.brokers_manifests))
    }

    pub fn brokers_list_ids(&self) -> JuizResult<Value> {
        let vec_value = self.brokers_manifests.values().collect::<Vec<&Value>>();
        let vec_str = vec_value.iter().map(|pv| { obj_get_str(*pv, "identifier").unwrap().to_string() }).collect::<Vec<String>>();
        Ok(jvalue!(vec_str))
    }

    pub fn profile_full(&self) -> JuizResult<Value> {
        let r = self.broker_proxies.list_ids()?;
        Ok(jvalue!({
            "process_factories": self.processes.factories_profile_full()?,
            "container_factories": self.containers.factories_profile_full()?,
            "container_process_factories": self.container_processes.factories_profile_full()?,
            "processes": self.processes.objects_profile_full()?,
            "containers": self.containers.objects_profile_full()?,
            "container_processes": self.container_processes.objects_profile_full()?,
            "brokers": self.brokers_profile_full()?,
            "broker_factories": self.broker_factories_profile_full()?,
            "broker_proxies": r,
            "ecs": self.ecs.objects_profile_full()?,
            "ec_factories": self.ecs.factories_profile_full()?,
        }))
    }
}