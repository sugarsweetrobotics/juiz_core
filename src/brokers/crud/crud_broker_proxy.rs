use std::{collections::HashMap, sync::{Mutex, Arc}};

use crate::{jvalue, brokers::{BrokerProxy, broker_proxy::{SystemBrokerProxy, ProcessBrokerProxy, ContainerBrokerProxy, ContainerProcessBrokerProxy, BrokerBrokerProxy, ExecutionContextBrokerProxy, ConnectionBrokerProxy}}, JuizObject, object::{JuizObjectCoreHolder, ObjectCore, JuizObjectClass}, Value, JuizResult, Identifier, processes::Output};


pub trait CRUDBrokerProxy : Send + Sync {
    fn create(&self, class_name: &str, function_name: &str, payload: Value, param: HashMap<String, String>) -> JuizResult<Value>;
    fn delete(&self, class_name: &str, function_name: &str, param: HashMap<String, String>) -> JuizResult<Value>;
    fn read(&self, class_name: &str, function_name: &str, param: HashMap<String, String>) -> JuizResult<Value>;
    fn update(&self, class_name: &str, function_name: &str, payload: Value, param: HashMap<String, String>) -> JuizResult<Output>;
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
    fn container_process_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        log::info!("CRUDBrokerProxy.contaienr_process_profile_full({id}) called");
        self.broker.read("container_process", "profile_full", HashMap::from([("identifier".to_string(), id.to_owned())]))
    }

    fn container_process_list(&self) -> JuizResult<Value> {
        self.broker.read("container_process", "list", HashMap::new())
    }
}


impl ContainerBrokerProxy for CRUDBrokerProxyHolder {
    fn container_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        self.broker.read("container", "profile_full", HashMap::from([("identifier".to_string(), id.to_owned())]))
    }

    fn container_list(&self) -> JuizResult<Value> {
        self.broker.read("container", "list", HashMap::new())
    }
}

impl ProcessBrokerProxy for CRUDBrokerProxyHolder {
    fn process_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        log::info!("CRUDBrokerProxy.process_profile_full({id}) called");
        self.broker.read("process", "profile_full", HashMap::from([("identifier".to_string(), id.to_owned())]))
    }

    fn process_call(&self, id: &Identifier, args: Value) -> JuizResult<Output> {
        self.broker.update("process", "list", args, HashMap::from([("identifier".to_string(), id.to_owned())]))
    }

    fn process_execute(&self, id: &Identifier) -> JuizResult<Output> {
        self.broker.update("process", "list", jvalue!({}), HashMap::from([("identifier".to_string(), id.to_owned())]))
    }

    fn process_list(&self) -> JuizResult<Value> {
        self.broker.read("process", "list", HashMap::new())
    }

    fn process_try_connect_to(&mut self, source_process_id: &Identifier, arg_name: &String, destination_process_id: &Identifier, manifest: Value) -> JuizResult<Value> {
        Ok(self.broker.update(
            "process", 
            "try_connect_to", 
            jvalue!({
                "source_process_id": source_process_id,
                "arg_name": arg_name,
                "destination_process_id": destination_process_id,
                "manifest": manifest
            }), 
            HashMap::from([]))?.value)
    }

    fn process_notify_connected_from(&mut self, source_process_id: &Identifier, arg_name: &String, destination_process_id: &Identifier, manifest: Value) -> JuizResult<Value> {
        Ok(self.broker.update(
            "process", 
            "notify_connected_from", 
            jvalue!({
                "source_process_id": source_process_id,
                "arg_name": arg_name,
                "destination_process_id": destination_process_id,
                "manifest": manifest
            }), 
            HashMap::from([]))?.value)
    }
}

impl SystemBrokerProxy for CRUDBrokerProxyHolder {
    fn system_profile_full(&self) -> JuizResult<Value> {
        self.broker.read("system", "profile_full", HashMap::new())

    }
}

impl BrokerBrokerProxy for CRUDBrokerProxyHolder {
    fn broker_list(&self) -> JuizResult<Value> {
        self.broker.read("broker", "list", HashMap::new())
    }

    fn broker_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        self.broker.read("broker", "profile_full", HashMap::from([("id".to_string(), id.to_owned())]))
    }
}

impl ExecutionContextBrokerProxy for CRUDBrokerProxyHolder {
    fn ec_list(&self) -> JuizResult<Value> {
        self.broker.read("execution_context", "list", HashMap::new())
    }

    fn ec_profile_full(&self, id: &Identifier) -> JuizResult<Value> { 
        self.broker.read("execution_context", "profile_full", HashMap::from([("id".to_string(), id.to_owned())]))
    }

    fn ec_get_state(&self, id: &Identifier) -> JuizResult<Value> {
        self.broker.read("execution_context", "get_state", HashMap::from([("id".to_string(), id.to_owned())]))
    }

    fn ec_start(&mut self, id: &Identifier) -> JuizResult<Value> {
        Ok(self.broker.update("execution_context", "start", jvalue!({}), HashMap::from([("id".to_owned(), id.to_owned())]))?.value)
    }

    fn ec_stop(&mut self, id: &Identifier) -> JuizResult<Value> {
        Ok(self.broker.update("execution_context", "stop", jvalue!({}), HashMap::from([("id".to_owned(), id.to_owned())]))?.value)
    }
}

impl ConnectionBrokerProxy for CRUDBrokerProxyHolder {
    fn connection_list(&self) -> JuizResult<Value> {
        self.broker.read("connection", "list", HashMap::new())
    }

    fn connection_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        self.broker.read("connection", "profile_full", HashMap::from([("id".to_string(), id.to_owned())]))
    }

    fn connection_create(&mut self, manifest: Value) -> JuizResult<Value> {
        self.broker.create("connection", "create", manifest, HashMap::new())
    }
}

impl BrokerProxy for CRUDBrokerProxyHolder {
    fn is_in_charge_for_process(&self, _id: &Identifier) -> JuizResult<bool> {
        todo!()
    }
}