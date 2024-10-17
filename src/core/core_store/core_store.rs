use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use super::buffer_object_collection::BufferObjectCollection;
use super::rw_object_collection::RwObjectCollection;
use super::object_collection::ObjectCollection;
use super::mutex_object_collection::MutexObjectCollection;

use crate::topics::TopicPtr;
use crate::{containers::ContainerProcessImpl, prelude::*, value::obj_get_str};
use crate::ecs::{execution_context_function::ExecutionContextFunction, execution_context_holder_factory::ExecutionContextHolderFactory};


use anyhow::anyhow;


pub struct CoreStore {
    broker_factories_manifests: HashMap<Identifier, Value>,
    brokers_manifests: HashMap<Identifier, Value>,
    pub topics: HashMap<Identifier, TopicPtr>,

    pub processes: Box<RwObjectCollection::<dyn Process, dyn ProcessFactory>>,
    //pub containers: Box<RwObjectCollection::<dyn Container, dyn ContainerFactory>>,
    pub containers: Box<ObjectCollection::<ContainerPtr, Arc<Mutex<dyn ContainerFactory>>>>,
    pub container_processes: Box<RwObjectCollection::<ContainerProcessImpl, dyn ContainerProcessFactory>>,
    pub ecs: Box<MutexObjectCollection::<dyn ExecutionContextFunction, ExecutionContextHolderFactory>>,
    pub broker_proxies: Box<BufferObjectCollection::<dyn BrokerProxy, dyn BrokerProxyFactory>>,
}


impl CoreStore {
    pub fn new() -> CoreStore {
        CoreStore{
            brokers_manifests: HashMap::new(),
            broker_proxies: BufferObjectCollection::new("broker_proxy"),
            broker_factories_manifests: HashMap::new(),
            topics: HashMap::new(),
            processes: RwObjectCollection::new("process"), 
            //containers: RwObjectCollection::new("container"), 
            containers: ObjectCollection::new("container"), 
            container_processes: RwObjectCollection::new("container_process"), 
            ecs: MutexObjectCollection::new("ecs"),
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

    pub fn topics_list_ids(&self) -> JuizResult<Value> {
        let vec_value = self.topics.values().into_iter().map(|topic| {
            topic.name().to_owned()
        } ).collect::<Vec<String>>();
        Ok(jvalue!(vec_value))
    }

    pub fn topics_profile_full(&self) -> JuizResult<Value> {
        let mut vec : Vec<Value> = Vec::new();
        for topic in self.topics.values().into_iter() {
            vec.push(topic.profile_full()?);
        } 
        Ok(jvalue!(vec))
    }

    pub fn containers_profile_full(&self) -> JuizResult<Value> {
       self.containers.objects().iter().map(|(k, c)| {
        c.lock()
            .and_then(|co| { 
                let id = co.identifier().clone();
                Ok((id, co.profile_full()?))
            })
        } ).collect()
    }

    pub fn containers_id(&self) -> JuizResult<Value> {
        self.containers.objects().iter().map(|(k, c)| {
         c.lock().and_then(|co| { Ok(co.identifier().clone()) })
         } ).collect()
     }

    pub fn container_factories_profile_full(&self) -> JuizResult<Value> {
        self.containers.factories().iter().map(|(k, c)| {
         c.lock().or_else(|e|{Err(anyhow!(JuizError::ObjectLockError{target:e.to_string()}))}).and_then(|co| { co.profile_full() })
         } ).collect()
     }

    pub fn profile_full(&self) -> JuizResult<Value> {
        let r = self.broker_proxies.list_ids()?;
        Ok(jvalue!({
            "process_factories": self.processes.factories_profile_full()?,
            "container_factories": self.container_factories_profile_full()?,
            "container_process_factories": self.container_processes.factories_profile_full()?,
            "processes": self.processes.objects_profile_full()?,
            "containers": self.containers_profile_full()?,
            "container_processes": self.container_processes.objects_profile_full()?,
            "brokers": self.brokers_profile_full()?,
            "broker_factories": self.broker_factories_profile_full()?,
            "broker_proxies": r,
            "ecs": self.ecs.objects_profile_full()?,
            "ec_factories": self.ecs.factories_profile_full()?,
            "topics": self.topics_profile_full()?,
        }))
    }
}