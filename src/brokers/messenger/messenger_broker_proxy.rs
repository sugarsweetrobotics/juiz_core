use std::{sync::{Arc, Mutex}, time::Duration};

use anyhow::Context;
use serde_json::Map;

use crate::{jvalue, JuizResult, Identifier, Value, JuizError, value::{obj_get_str, obj_get}, JuizObject, object::{ObjectCore, JuizObjectClass, JuizObjectCoreHolder}, brokers::broker_proxy::{ContainerBrokerProxy, ContainerProcessBrokerProxy, ExecutionContextBrokerProxy, BrokerBrokerProxy, ConnectionBrokerProxy}};

use super::super::broker_proxy::{BrokerProxy, SystemBrokerProxy, ProcessBrokerProxy};




pub struct MessengerBrokerProxy {
    core: ObjectCore, 
    messenger: Box<dyn MessengerBrokerProxyCore>,
}

pub type SenderType = dyn Fn(Value) -> JuizResult<()>;
pub type ReceiverType = dyn Fn(Duration) -> JuizResult<Value>;

pub struct SendReceivePair(pub Box<SenderType>, pub Box<ReceiverType>);
pub trait MessengerBrokerProxyCore : Send {
    fn send_and_receive(&self, v: Value, timeout: Duration) -> JuizResult<Value>;
}

pub trait MessengerBrokerProxyCoreFactory { 
    fn create_core(&self, object_name: &str) -> JuizResult<Box<dyn MessengerBrokerProxyCore>>;
}

fn to_map(params: &[(String, String)]) -> Map<String, Value> {
    let mut map : Map<String, Value> = Map::new();
    for (k, v) in params {
        map.insert(k.clone(), jvalue!(v));
    }
    map
}

impl MessengerBrokerProxy {

    pub fn new(class_name: &'static str, type_name: &str, object_name: &str, messenger: Box<dyn MessengerBrokerProxyCore>) -> JuizResult<Arc<Mutex<dyn BrokerProxy>>>{
        Ok(Arc::new(Mutex::new(MessengerBrokerProxy{
            core: ObjectCore::create(JuizObjectClass::BrokerProxy(class_name), type_name, object_name),
            messenger})))
    }

    pub fn send_recv_and<F: Fn(Value)->JuizResult<T>, T>(&self, method_name: &str, class_name: &str, function_name: &str, arguments: Value, params: &[(String, String)], func: F) -> JuizResult<T> {
        log::trace!("MessengerBrokerProxy::send_recv_and({class_name}, {function_name}, {arguments}) called");
        //let SendReceivePair(sndr, recvr) = self.messenger.send_receive()?;
        let value = self.messenger.send_and_receive(jvalue!({
            "method_name": method_name,
            "class_name": class_name,
            "function_name": function_name, 
            "arguments": arguments,
            "params": to_map(params),
        }), Duration::new(3, 0)).context("MessengerBrokerProxyCore.send_and_receive() failed in MessengerBrokerProxy.send_recv_and()")?;
        //let value = (recvr)(timeout)?;
        let response_function_name = obj_get_str(&value, "function_name")?;
        match response_function_name {
            "RequestFunctionNameNotSupported" => {
                return Err(anyhow::Error::from(JuizError::BrokerProxyRequestFunctionNameNotSupportedError{request_function_name: function_name.to_string()}));
            },
            _ => {
                if response_function_name != function_name {
                    return Err(anyhow::Error::from(JuizError::BrokerProxyFunctionNameInResponseDoesNotMatchError{function_name: function_name.to_string(), response_function_name: response_function_name.to_string()}));
                }
                func(value)
            }
        }
    }

    pub fn read(&self, class_name: &str, function_name: &str) -> JuizResult<Value> {
        self.send_recv_and(
            "READ", 
            class_name, 
            function_name,
            jvalue!({}), 
            &[],
            |value| Ok(obj_get(&value, "return")?.clone()))
    }

    pub fn read_by_id(&self, class_name: &str, function_name: &str, id: &Identifier) -> JuizResult<Value> {
        self.send_recv_and(
            "READ", 
            class_name, 
            function_name,
            jvalue!({}), 
            &[("id".to_owned(), id.clone())],
            |value| Ok(obj_get(&value, "return")?.clone()))
    }

    pub fn update_by_id(&self, class_name: &str, function_name: &str, args: Value, id: &Identifier) -> JuizResult<Value>  {
        self.send_recv_and(
            "UPDATE",
            class_name,  
            function_name, 
            jvalue!({"id": id, "args": args}), 
            &[("id".to_owned(), id.clone())],
            |value| Ok(obj_get(&value, "return")?.clone()))
    }

    pub fn create(&self, class_name: &str, function_name: &str, args: Value) -> JuizResult<Value>  {
        self.send_recv_and(
            "CREATE",
            class_name,  
            function_name, 
            args, 
            &[],
            |value| Ok(obj_get(&value, "return")?.clone()))
    }
    
}

impl JuizObjectCoreHolder for MessengerBrokerProxy {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}

impl JuizObject for MessengerBrokerProxy {}

impl SystemBrokerProxy for MessengerBrokerProxy {
    fn system_profile_full(&self) -> JuizResult<Value> {
        self.read("system", "profile_full")
    }
}

impl ProcessBrokerProxy for MessengerBrokerProxy {

    fn process_call(&self, id: &Identifier, args: crate::Value) -> crate::JuizResult<crate::Value> {
        self.update_by_id("process", "execute", args, id)
    }

    fn process_execute(&self, id: &crate::Identifier) -> crate::JuizResult<crate::Value> {
        self.update_by_id("process", "execute", jvalue!({}), id)
    }

    fn process_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        self.read_by_id("process", "profile_full", id)
    }

    fn process_list(&self) -> JuizResult<Value> {
        self.read("process", "list")
    }

    fn process_try_connect_to(&mut self, source_process_id: &Identifier, arg_name: &String, destination_process_id: &Identifier, manifest: Value) -> JuizResult<Value> {
        self.send_recv_and(
            "UPDATE", 
            "process", 
            "try_connect_to", 
            jvalue!({
                "source_process_id": source_process_id,
                "arg_name": arg_name,
                "destination_process_id": destination_process_id,
                "manifest": manifest
            }), 
            &[], 
            |value| Ok(obj_get(&value, "return")?.clone()))
    }

    fn process_notify_connected_from(&mut self, source_process_id: &Identifier, arg_name: &String, destination_process_id: &Identifier, manifest: Value) -> JuizResult<Value> {
        self.send_recv_and(
            "UPDATE", 
            "process", 
            "notify_connected_from", 
            jvalue!({
                "source_process_id": source_process_id,
                "arg_name": arg_name,
                "destination_process_id": destination_process_id,
                "manifest": manifest
            }), 
            &[], 
            |value| Ok(obj_get(&value, "return")?.clone()))
    }
}

impl ContainerBrokerProxy for MessengerBrokerProxy {
    fn container_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        self.read_by_id("container", "profile_full", id)
    }

    fn container_list(&self) -> JuizResult<Value> {
        self.read("container", "list")
    }
}

impl ContainerProcessBrokerProxy for MessengerBrokerProxy {
    fn container_process_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        self.read_by_id("container_process", "profile_full", id)
    }

    fn container_process_list(&self) -> JuizResult<Value> {
        self.read("container_process", "list")
    }
}

impl ExecutionContextBrokerProxy for MessengerBrokerProxy {

    fn ec_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        self.read_by_id("execution_context", "profile_full", id)
    }

    fn ec_list(&self) -> JuizResult<Value> {
        self.read("execution_context", "list")
    }

    fn ec_get_state(&self, id: &Identifier) -> JuizResult<Value> {
        self.read_by_id("execution_context", "get_state", id)
    }

    fn ec_start(&mut self, id: &Identifier) -> JuizResult<Value> {
        self.update_by_id("execution_context", "start", jvalue!({}), id)
    }

    fn ec_stop(&mut self, id: &Identifier) -> JuizResult<Value> {
        self.update_by_id("execution_context", "stop", jvalue!({}), id)
    }
}

impl BrokerBrokerProxy for MessengerBrokerProxy {
    fn broker_list(&self) -> JuizResult<Value> {
        self.read("broker", "list")
    }

    fn broker_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        self.read_by_id("broker", "profile_full", id)
    }
}

impl ConnectionBrokerProxy for MessengerBrokerProxy {
    fn connection_list(&self) -> JuizResult<Value> {
        self.read("connection", "list")
    }

    fn connection_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        self.read_by_id("connection", "profile_full", id)
    }

    fn connection_create(&mut self, manifest: Value) -> JuizResult<Value> {
        self.create("connection", "create", manifest)
    }
}

impl BrokerProxy for MessengerBrokerProxy {
    fn is_in_charge_for_process(&self, _id: &Identifier) -> JuizResult<bool> {
        todo!()
        //self.send_recv_and("process", "is_in_charge_for_process", jvalue!({"id": id}), |value| obj_get_bool(&value, "return"))
    }

    //fn profile_full(&self) -> crate::JuizResult<crate::Value> {
    //    self.send_recv_and("profile_full", jvalue!({}), |value| Ok(obj_get(&value, "return")?.clone()))
    //}

}