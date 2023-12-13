use std::{sync::{Arc, Mutex}, time::Duration, collections::HashMap};

use crate::{jvalue, BrokerProxy, JuizResult, Identifier, Value, JuizError, value::{obj_get_str, obj_get_bool, obj_get}, JuizObject, object::{ObjectCore, JuizObjectClass, JuizObjectCoreHolder}};

use super::broker_proxy::{SystemBrokerProxy, ProcessBrokerProxy};




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
            "params": params,
        }), Duration::new(1, 0))?;
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
}

impl JuizObjectCoreHolder for MessengerBrokerProxy {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}

impl JuizObject for MessengerBrokerProxy {}

impl SystemBrokerProxy for MessengerBrokerProxy {
    fn system_profile_full(&self) -> JuizResult<Value> {
        self.send_recv_and("READ", "system", "profile_full", 
            jvalue!({}), &[], |value| Ok(obj_get(&value, "return")?.clone()))
    }
}

impl ProcessBrokerProxy for MessengerBrokerProxy {

    fn process_call(&self, id: &Identifier, args: crate::Value) -> crate::JuizResult<crate::Value> {
        self.send_recv_and(
            "UPDATE",
            "process", 
            "call", 
            jvalue!({"id": id, "args": args}), 
            &[("id".to_owned(), id.clone())],
            |value| Ok(obj_get(&value, "return")?.clone()))
    }

    fn process_execute(&self, id: &crate::Identifier) -> crate::JuizResult<crate::Value> {
        self.send_recv_and(
            "UPDATE",
            "process", 
            "execute", 
            jvalue!({"id": id}), 
            &[("id".to_owned(), id.clone())],
            |value| Ok(obj_get(&value, "return")?.clone()))
    }

    fn process_connect_to(&mut self, 
        source_process_id: &crate::Identifier, 
        arg_name: &String, target_process_id: &crate::Identifier,
        manifest: crate::Value) -> crate::JuizResult<crate::Value> {
            
        self.send_recv_and(
            "UPDATE", 
            "process", "connect_to", jvalue!({
            "source_process_id": source_process_id,
            "arg_name": arg_name,
            "target_process_id": target_process_id,
            "manifest": manifest
        }), 
        &[], 
        |value| Ok(obj_get(&value, "return")?.clone()))
    }

    fn process_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        self.send_recv_and(
            "READ", 
            "process", 
            "profile_full",
            jvalue!({"id": id}),
            &[("id".to_owned(), id.clone())], |value| Ok(obj_get(&value, "return")?.clone()))
    }
}

impl BrokerProxy for MessengerBrokerProxy {
    fn is_in_charge_for_process(&self, id: &Identifier) -> JuizResult<bool> {
        todo!()
        //self.send_recv_and("process", "is_in_charge_for_process", jvalue!({"id": id}), |value| obj_get_bool(&value, "return"))
    }

    //fn profile_full(&self) -> crate::JuizResult<crate::Value> {
    //    self.send_recv_and("profile_full", jvalue!({}), |value| Ok(obj_get(&value, "return")?.clone()))
    //}

}