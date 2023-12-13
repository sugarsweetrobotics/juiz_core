use std::{sync::{Arc, Mutex}, time::Duration, ops::Deref};

use crate::{jvalue, BrokerProxy, JuizResult, Identifier, Value, JuizError, value::{obj_get_str, obj_get_bool, obj_get, obj_merge}, JuizObject, object::{ObjectCore, JuizObjectClass, JuizObjectCoreHolder}};

use super::{local_broker::SenderReceiverPair, broker_proxy::{SystemBrokerProxy, ProcessBrokerProxy}};




pub struct LocalBrokerProxy {
    core: ObjectCore, 
    sender_receiver: Arc<Mutex<SenderReceiverPair>>,
    
}

impl LocalBrokerProxy {

    pub fn new(object_name: &str, sender_receiver: Arc<Mutex<SenderReceiverPair>> ) -> JuizResult<Arc<Mutex<dyn BrokerProxy>>>{
        let type_name = "local";
        
        Ok(Arc::new(Mutex::new(LocalBrokerProxy{
            core: ObjectCore::create(JuizObjectClass::BrokerProxy("LocalBrokerFactory"), type_name, object_name),
            sender_receiver})))
    }

    pub fn send_recv_and<F: Fn(Value)->JuizResult<T>, T>(&self, class_name: &str, function_name: &str, arguments: Value, func: F) -> JuizResult<T> {
        let sndr_recvr = self.sender_receiver.try_lock().map_err(|_e| return anyhow::Error::from(JuizError::BrokerSendCanNotLockSenderError{}))?;
        let SenderReceiverPair(sndr, recvr) = sndr_recvr.deref();
        let _ = sndr.send(jvalue!({
            "class_name": class_name,
            "function_name": function_name, 
            "arguments": arguments})).map_err(|e| return anyhow::Error::from(JuizError::LocalBrokerProxySendError{send_error: e}))?;
        
        let timeout = Duration::new(1, 0);
        let value = recvr.recv_timeout(timeout).map_err(|e|
                return anyhow::Error::from(JuizError::LocalBrokerProxyReceiveTimeoutError{error: e}))?;
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

impl JuizObjectCoreHolder for LocalBrokerProxy {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}

impl JuizObject for LocalBrokerProxy {}

impl SystemBrokerProxy for LocalBrokerProxy {
    fn system_profile_full(&self) -> JuizResult<Value> {
        self.send_recv_and("system", "profile_full", jvalue!({}), |value| Ok(obj_get(&value, "return")?.clone()))
    }
}

impl ProcessBrokerProxy for LocalBrokerProxy {

    fn process_call(&self, id: &Identifier, args: crate::Value) -> crate::JuizResult<crate::Value> {
        self.send_recv_and("process", "call", jvalue!({"id": id, "args": args}), |value| Ok(obj_get(&value, "return")?.clone()))
    }

    fn process_execute(&self, id: &crate::Identifier) -> crate::JuizResult<crate::Value> {
        self.send_recv_and("process", "execute", jvalue!({"id": id}), |value| Ok(obj_get(&value, "return")?.clone()))
    }

    fn process_connect_to(&mut self, 
        source_process_id: &crate::Identifier, 
        arg_name: &String, target_process_id: &crate::Identifier,
        manifest: crate::Value) -> crate::JuizResult<crate::Value> {
            
        self.send_recv_and("process", "connect_to", jvalue!({
            "source_process_id": source_process_id,
            "arg_name": arg_name,
            "target_process_id": target_process_id,
            "manifest": manifest
        }), |value| Ok(obj_get(&value, "return")?.clone()))
    }

    fn process_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        self.send_recv_and("process", "profile_full", jvalue!({"id": id}), |value| Ok(obj_get(&value, "return")?.clone()))
    }
}

impl BrokerProxy for LocalBrokerProxy {
    fn is_in_charge_for_process(&self, id: &Identifier) -> JuizResult<bool> {
        self.send_recv_and("process", "is_in_charge_for_process", jvalue!({"id": id}), |value| obj_get_bool(&value, "return"))
    }

    //fn profile_full(&self) -> crate::JuizResult<crate::Value> {
    //    self.send_recv_and("profile_full", jvalue!({}), |value| Ok(obj_get(&value, "return")?.clone()))
    //}

}