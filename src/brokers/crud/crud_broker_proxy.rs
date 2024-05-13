use std::{collections::HashMap, sync::{Mutex, Arc}};

use crate::{brokers::{broker_proxy::{BrokerBrokerProxy, ConnectionBrokerProxy, ContainerBrokerProxy, ContainerProcessBrokerProxy, ExecutionContextBrokerProxy, ProcessBrokerProxy, SystemBrokerProxy}, BrokerProxy}, jvalue, object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore}, processes::capsule::{Capsule, CapsuleMap}, Identifier, JuizObject, JuizResult, Value};


pub trait CRUDBrokerProxy : Send + Sync {
    fn create(&self, class_name: &str, function_name: &str, payload: Value, param: HashMap<String, String>) -> JuizResult<Capsule>;
    fn delete(&self, class_name: &str, function_name: &str, param: HashMap<String, String>) -> JuizResult<Capsule>;
    fn read(&self, class_name: &str, function_name: &str, param: HashMap<String, String>) -> JuizResult<Capsule>;
    fn update(&self, class_name: &str, function_name: &str, payload: CapsuleMap, param: HashMap<String, String>) -> JuizResult<Capsule>;
}


pub struct CRUDBrokerProxyHolder {
    core: ObjectCore,
    broker: Box<dyn CRUDBrokerProxy>,
}

impl CRUDBrokerProxyHolder {

    pub fn new(impl_class_name: &'static str, type_name: &str, name: &str, broker_proxy: Box<dyn CRUDBrokerProxy>) -> JuizResult<Arc<Mutex<CRUDBrokerProxyHolder>>> {

        Ok(Arc::new(Mutex::new(CRUDBrokerProxyHolder{
            core: ObjectCore::create(JuizObjectClass::BrokerProxy(impl_class_name), type_name, name),
            broker: broker_proxy,
        })))
    }
}

impl JuizObjectCoreHolder for CRUDBrokerProxyHolder {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}

impl JuizObject for CRUDBrokerProxyHolder {
    
}

impl ContainerProcessBrokerProxy for CRUDBrokerProxyHolder {
    fn container_process_profile_full(&self, id: &Identifier) -> JuizResult<Capsule> {
        log::info!("CRUDBrokerProxy.contaienr_process_profile_full({id}) called");
        self.broker.read("container_process", "profile_full", HashMap::from([("identifier".to_string(), id.to_owned())]))
    }

    fn container_process_list(&self) -> JuizResult<Capsule> {
        self.broker.read("container_process", "list", HashMap::new())
    }
    
    fn container_process_call(&self, id: &Identifier, args: CapsuleMap) -> JuizResult<Capsule> {
        self.broker.update("container_process", "call", args, HashMap::from([("identifier".to_string(), id.to_owned())]))
    }
    
    fn container_process_execute(&self, id: &Identifier) -> JuizResult<Capsule> {
        self.broker.update("container_process", "execute", CapsuleMap::new(), HashMap::from([("identifier".to_string(), id.to_owned())]))
    }
}


impl ContainerBrokerProxy for CRUDBrokerProxyHolder {
    fn container_profile_full(&self, id: &Identifier) -> JuizResult<Capsule> {
        self.broker.read("container", "profile_full", HashMap::from([("identifier".to_string(), id.to_owned())]))
    }

    fn container_list(&self) -> JuizResult<Capsule> {
        self.broker.read("container", "list", HashMap::new())
    }
}

impl ProcessBrokerProxy for CRUDBrokerProxyHolder {
    fn process_profile_full(&self, id: &Identifier) -> JuizResult<Capsule> {
        log::info!("CRUDBrokerProxy.process_profile_full({id}) called");
        self.broker.read("process", "profile_full", HashMap::from([("identifier".to_string(), id.to_owned())]))
    }

    fn process_call(&self, id: &Identifier, args: CapsuleMap) -> JuizResult<Capsule> {
        self.broker.update("process", "call", args, HashMap::from([("identifier".to_string(), id.to_owned())]))
    }

    fn process_execute(&self, id: &Identifier) -> JuizResult<Capsule> {
        self.broker.update("process", "execute", CapsuleMap::new(), HashMap::from([("identifier".to_string(), id.to_owned())]))
    }

    fn process_list(&self) -> JuizResult<Capsule> {
        self.broker.read("process", "list", HashMap::new())
    }

    fn process_try_connect_to(&mut self, source_process_id: &Identifier, arg_name: &String, destination_process_id: &Identifier, manifest: Value) -> JuizResult<Capsule> {
        self.broker.update(
            "process", 
            "try_connect_to", 
            CapsuleMap::try_from(jvalue!({
                "source_process_id": source_process_id,
                "arg_name": arg_name,
                "destination_process_id": destination_process_id,
                "manifest": manifest
            }))?, 
            HashMap::from([]))
    }

    fn process_notify_connected_from(&mut self, source_process_id: &Identifier, arg_name: &String, destination_process_id: &Identifier, manifest: Value) -> JuizResult<Capsule> {
        self.broker.update(
            "process", 
            "notify_connected_from", 
            CapsuleMap::try_from(jvalue!({
                "source_process_id": source_process_id,
                "arg_name": arg_name,
                "destination_process_id": destination_process_id,
                "manifest": manifest
            }))?, 
            HashMap::from([]))
    }
}

impl SystemBrokerProxy for CRUDBrokerProxyHolder {
    fn system_profile_full(&self) -> JuizResult<Capsule> {
        self.broker.read("system", "profile_full", HashMap::new())

    }
}

impl BrokerBrokerProxy for CRUDBrokerProxyHolder {
    fn broker_list(&self) -> JuizResult<Capsule> {
        self.broker.read("broker", "list", HashMap::new())
    }

    fn broker_profile_full(&self, id: &Identifier) -> JuizResult<Capsule> {
        self.broker.read("broker", "profile_full", HashMap::from([("id".to_string(), id.to_owned())]))
    }
}

impl ExecutionContextBrokerProxy for CRUDBrokerProxyHolder {
    fn ec_list(&self) -> JuizResult<Capsule> {
        self.broker.read("execution_context", "list", HashMap::new())
    }

    fn ec_profile_full(&self, id: &Identifier) -> JuizResult<Capsule> { 
        self.broker.read("execution_context", "profile_full", HashMap::from([("id".to_string(), id.to_owned())]))
    }

    fn ec_get_state(&self, id: &Identifier) -> JuizResult<Capsule> {
        self.broker.read("execution_context", "get_state", HashMap::from([("id".to_string(), id.to_owned())]))
    }

    fn ec_start(&mut self, id: &Identifier) -> JuizResult<Capsule> {
        self.broker.update("execution_context", "start", CapsuleMap::new(), HashMap::from([("id".to_owned(), id.to_owned())]))
    }

    fn ec_stop(&mut self, id: &Identifier) -> JuizResult<Capsule> {
        self.broker.update("execution_context", "stop",  CapsuleMap::new(), HashMap::from([("id".to_owned(), id.to_owned())]))
    }
}

impl ConnectionBrokerProxy for CRUDBrokerProxyHolder {
    fn connection_list(&self) -> JuizResult<Capsule> {
        self.broker.read("connection", "list", HashMap::new())
    }

    fn connection_profile_full(&self, id: &Identifier) -> JuizResult<Capsule> {
        self.broker.read("connection", "profile_full", HashMap::from([("id".to_string(), id.to_owned())]))
    }

    fn connection_create(&mut self, manifest: Value) -> JuizResult<Capsule> {
        self.broker.create("connection", "create", manifest, HashMap::new())
    }
}

impl BrokerProxy for CRUDBrokerProxyHolder {
    fn is_in_charge_for_process(&self, _id: &Identifier) -> JuizResult<bool> {
        todo!()
    }
}