use std::{sync::{Arc, Mutex, mpsc}, time::Duration, ops::Deref};

use crate::{jvalue, BrokerProxy, JuizResult, Identifier, Value, JuizError, value::{obj_get_str, obj_get_bool, obj_get}};

use super::local_broker::SenderReceiverPair;




pub struct LocalBrokerProxy {
    //sender: Arc<Mutex<mpsc::Sender<Value>>>,
    //receiver: Arc<Mutex<mpsc::Receiver<Value>>>,
    sender_receiver: Arc<Mutex<SenderReceiverPair>>,
    
}

impl LocalBrokerProxy {

    pub fn new(sender_receiver: Arc<Mutex<SenderReceiverPair>> ) -> JuizResult<Arc<Mutex<dyn BrokerProxy>>>{
        Ok(Arc::new(Mutex::new(LocalBrokerProxy{
        sender_receiver})))
    }

    pub fn send_recv_and<F: Fn(Value)->JuizResult<T>, T>(&self, function_name: &str, arguments: Value, func: F) -> JuizResult<T> {
        let sndr_recvr = self.sender_receiver.lock().map_err(|e| return anyhow::Error::from(JuizError::BrokerSendCanNotLockSenderError{}))?;
        let SenderReceiverPair(sndr, recvr) = sndr_recvr.deref();
        let _ = sndr.send(jvalue!({"function_name": function_name, "arguments": arguments})).map_err(|e| return anyhow::Error::from(JuizError::LocalBrokerProxySendError{send_error: e}))?;
        
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

impl BrokerProxy for LocalBrokerProxy {
    fn is_in_charge_for_process(&self, id: &Identifier) -> JuizResult<bool> {
        self.send_recv_and("is_in_charge_for_process", jvalue!({"id": id}), |value| obj_get_bool(&value, "return"))
    }

    fn call_process(&self, id: &Identifier, args: crate::Value) -> crate::JuizResult<crate::Value> {
        self.send_recv_and("call_process", jvalue!({"id": id, "args": args}), |value| Ok(obj_get(&value, "return")?.clone()))
    }

    fn execute_process(&self, id: &crate::Identifier) -> crate::JuizResult<crate::Value> {
        self.send_recv_and("execute_process", jvalue!({"id": id}), |value| Ok(obj_get(&value, "return")?.clone()))
    }

    fn connect_process_to(&mut self, 
        source_process_id: &crate::Identifier, 
        arg_name: &String, target_process_id: &crate::Identifier,
        manifest: crate::Value) -> crate::JuizResult<crate::Value> {
            
        self.send_recv_and("connect_process_to", jvalue!({
            "source_process_id": source_process_id,
            "arg_name": arg_name,
            "target_process_id": target_process_id,
            "manifest": manifest
        }), |value| Ok(obj_get(&value, "return")?.clone()))
    }

    fn profile_full(&self) -> crate::JuizResult<crate::Value> {
        self.send_recv_and("profile_full", jvalue!({}), |value| Ok(obj_get(&value, "return")?.clone()))
    }
}