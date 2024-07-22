use std::{sync::{Arc, Mutex}, time::Duration};
use anyhow::Context;
use crate::{brokers::broker_proxy::{BrokerBrokerProxy, ConnectionBrokerProxy, ContainerBrokerProxy, ContainerProcessBrokerProxy, ExecutionContextBrokerProxy}, jvalue, object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore}, value::{CapsuleMap, capsule_to_value}, CapsulePtr, Identifier, JuizError, JuizObject, JuizResult, Value};
use super::super::broker_proxy::{BrokerProxy, SystemBrokerProxy, ProcessBrokerProxy};




pub struct MessengerBrokerProxy {
    core: ObjectCore, 
    messenger: Box<dyn MessengerBrokerProxyCore>,
}

pub type SenderType = dyn Fn(CapsuleMap) -> JuizResult<()>;
pub type ReceiverType = dyn Fn(Duration) -> JuizResult<CapsulePtr>;

pub struct SendReceivePair(pub Box<SenderType>, pub Box<ReceiverType>);
pub trait MessengerBrokerProxyCore : Send {
    fn send_and_receive(&self, v: CapsuleMap, timeout: Duration) -> JuizResult<CapsulePtr>;
    fn send_and_receive_output(&self, v: CapsuleMap, timeout: Duration) -> JuizResult<CapsulePtr>;
}

pub trait MessengerBrokerProxyCoreFactory { 
    fn create_core(&self, object_name: &str) -> JuizResult<Box<dyn MessengerBrokerProxyCore>>;
}

/*
fn to_map(params: &[(String, String)]) -> Map<String, Value> {
    let mut map : Map<String, Value> = Map::new();
    for (k, v) in params {
        map.insert(k.clone(), jvalue!(v));
    }
    map
}
*/

impl MessengerBrokerProxy {

    pub fn new(class_name: &'static str, type_name: &str, object_name: &str, messenger: Box<dyn MessengerBrokerProxyCore>) -> JuizResult<Arc<Mutex<dyn BrokerProxy>>>{
        Ok(Arc::new(Mutex::new(MessengerBrokerProxy{
            core: ObjectCore::create(JuizObjectClass::BrokerProxy(class_name), type_name, object_name),
            messenger})))
    }

    fn construct_capsule_map(&self, method_name: &str, class_name: &str, function_name: &str, mut arguments: CapsuleMap, params: &[(String, String)]) -> CapsuleMap {
        arguments
            .set_param("method_name", method_name)
            .set_param("class_name", class_name)
            .set_param("function_name", function_name);
        for (k, v) in params {
            arguments.set_param(k.as_str(), v);
        }
        arguments
    }

    fn extract_function_param(&self, value: &CapsulePtr) -> JuizResult<String> {
        let _err = |name: &str | anyhow::Error::from(JuizError::CapsuleDoesNotIncludeParamError{ name: name.to_owned() });
        Ok(value.get_option("function_name")?.clone())
    }

    pub fn send_recv_and<F: Fn(CapsulePtr)->JuizResult<T>, T>(&self, method_name: &str, class_name: &str, function_name: &str, arguments: CapsuleMap, params: &[(String, String)], func: F) -> JuizResult<T> {
        log::trace!("MessengerBrokerProxy::send_recv_and({class_name}, {function_name}, arguments) called");
        //let SendReceivePair(sndr, recvr) = self.messenger.send_receive()?;
    
        let value = self.messenger.send_and_receive(self.construct_capsule_map(
            method_name, 
            class_name,
            function_name,
            arguments,
            params
        ), Duration::new(3, 0)).context("MessengerBrokerProxyCore.send_and_receive() failed in MessengerBrokerProxy.send_recv_and()")?;
        //let value = (recvr)(timeout)?;
        log::trace!("MessengerBrokerProxy::send_recv_and() received value {value:?}");
        let response_function_name_result = self.extract_function_param(&value);//obj_get_str(&value, "function_name")?;
        log::trace!("reponse_function_name is {response_function_name_result:?}");
        let response_function_name = response_function_name_result?;
        let result = match response_function_name.as_str() {
            "RequestFunctionNameNotSupported" => {
                log::error!("MessengerBrokerProxy::send_recv_and() error. Requested function name {function_name} is not supported.");
                return Err(anyhow::Error::from(JuizError::BrokerProxyRequestFunctionNameNotSupportedError{request_function_name: function_name.to_string()}));
            },
            _ => {
                if response_function_name != function_name {
                    log::error!("MessengerBrokerProxy::send_recv_and() error. Function name {function_name} does not match. Response function name is {response_function_name}.");
                    return Err(anyhow::Error::from(JuizError::BrokerProxyFunctionNameInResponseDoesNotMatchError{function_name: function_name.to_string(), response_function_name: response_function_name.to_string()}));
                }
                log::trace!("MessengerBrokerProxy::send_recv_and() success. Calling post function callback");
                func(value)
            }
        };
        log::trace!("MessengerBrokerProxy::send_recv_and() exit");
        result
    }

    fn _construct_argument(_method_name: &str, _class_name: &str, _function_name: &str, _arguments: CapsuleMap, _params: &[(String, String)]) -> JuizResult<CapsuleMap> {

        todo!("ここにmethod_nameなどをArgumentMapに埋め込む作業を書く")
        /* 
        jvalue!({
            "method_name": method_name,
            "class_name": class_name,
            "function_name": function_name, 
            "arguments": arguments,
            "params": to_map(params),
        }
        */
    }

    pub fn send_recv_output_and<F: Fn(CapsulePtr)->JuizResult<T>, T>(&self, method_name: &str, class_name: &str, function_name: &str, arguments: CapsuleMap, params: &[(String, String)], func: F) -> JuizResult<T> {
        //log::trace!("MessengerBrokerProxy::send_recv_output_and({class_name}, {function_name}, {arguments}) called");
        log::trace!("MessengerBrokerProxy::send_recv_output_and({class_name}, {function_name}, arguments) called");
        //let SendReceivePair(sndr, recvr) = self.messenger.send_receive()?;
        let value = self.messenger.send_and_receive_output(
            Self::_construct_argument(method_name, class_name, function_name, arguments, params)?, Duration::new(3, 0)).context("MessengerBrokerProxyCore.send_and_receive() failed in MessengerBrokerProxy.send_recv_and()")?;
        //let value = (recvr)(timeout)?;
        //let response_function_name = obj_get_str(juiz_lock(&value)?.as_value().unwrap(), "function_name")?.to_owned();
        let response_function_name = value.get_function_name()?;
        
        match response_function_name.as_str() {
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

    pub fn read(&self, class_name: &str, function_name: &str) -> JuizResult<CapsulePtr> {
        self.send_recv_and(
            "READ", 
            class_name, 
            function_name,
            CapsuleMap::new(), 
            &[],
            //|value| Ok(obj_get(&value, "return")?.clone()))
            |value| Ok(value))
    }

    pub fn read_by_id(&self, class_name: &str, function_name: &str, id: &Identifier) -> JuizResult<CapsulePtr> {
        self.send_recv_and(
            "READ", 
            class_name, 
            function_name,
            CapsuleMap::new(), 
            &[("id".to_owned(), id.clone())],
            //|value| Ok(obj_get(&value, "return")?.clone()))
            |value| Ok(value))
    }

    pub fn read_with_param(&self, class_name: &str, function_name: &str, param: &[(String, String)]) -> JuizResult<CapsulePtr> {
        self.send_recv_and(
            "READ", 
            class_name, 
            function_name,
            CapsuleMap::new(), 
            param,
            //|value| Ok(obj_get(&value, "return")?.clone()))
            |value| Ok(value))
    }

    pub fn update_output_by_id(&self, class_name: &str, function_name: &str, args: CapsuleMap, id: &Identifier) -> JuizResult<CapsulePtr>  {
        self.send_recv_output_and(
            "UPDATE",
            class_name, 
            function_name, 
            args, 
            &[("id".to_owned(), id.clone())],
            |value| Ok(value))
    }

    pub fn update_by_id(&self, class_name: &str, function_name: &str, args: CapsuleMap, id: &Identifier) -> JuizResult<CapsulePtr>  {
        self.send_recv_and(
            "UPDATE",
            class_name,  
            function_name, 
            args, 
            &[("id".to_owned(), id.clone())],
            |value| Ok(value))
    }

    pub fn create(&self, class_name: &str, function_name: &str, args: CapsuleMap) -> JuizResult<CapsulePtr>  {
        self.send_recv_and(
            "CREATE",
            class_name,  
            function_name, 
            args, 
            &[],
            |value| Ok(value))
            //|value| Ok(obj_get(&value, "return")?.clone()))
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
        capsule_to_value(self.read("system", "profile_full")?)
    }
    
    fn system_filesystem_list(&self, path_buf: std::path::PathBuf) -> JuizResult<Value> {
        capsule_to_value(self.read_with_param("system", "filesystem_list", &[("path".to_owned(), path_buf.to_str().unwrap().to_owned())])?)
    }
}

impl ProcessBrokerProxy for MessengerBrokerProxy {

    fn process_call(&self, id: &Identifier, args: CapsuleMap) -> JuizResult<CapsulePtr> {
        self.update_output_by_id("process", "call", args, id)
    }

    fn process_execute(&self, id: &crate::Identifier) -> JuizResult<CapsulePtr> {
        self.update_output_by_id("process", "execute", CapsuleMap::new(), id)
    }

    fn process_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        capsule_to_value(self.read_by_id("process", "profile_full", id)?)
    }

    fn process_list(&self) -> JuizResult<Value> {
        self.read("process", "list")?.extract_value()
        //todo!("ここで__value__, __option___を使ってた弊害出てるぞ");
        //capsule_to_value(self.read("process", "list")?)
    }

    fn process_try_connect_to(&mut self, source_process_id: &Identifier, arg_name: &str, destination_process_id: &Identifier, manifest: Value) -> JuizResult<Value> {
        let capsule = self.send_recv_and(
            "UPDATE", 
            "process", 
            "try_connect_to", 
            jvalue!({
                "source_process_id": source_process_id,
                "arg_name": arg_name,
                "destination_process_id": destination_process_id,
                "manifest": manifest
            }).try_into()?, 
            &[], 
            |value| Ok(value))?;
        capsule_to_value(capsule)
    }

    fn process_notify_connected_from(&mut self, source_process_id: &Identifier, arg_name: &str, destination_process_id: &Identifier, manifest: Value) -> JuizResult<Value> {
        let value = self.send_recv_and(
            "UPDATE", 
            "process", 
            "notify_connected_from", 
            jvalue!({
                "source_process_id": source_process_id,
                "arg_name": arg_name,
                "destination_process_id": destination_process_id,
                "manifest": manifest
            }).try_into()?, 
            &[], 
            |value| Ok(value))?;
        capsule_to_value(value)
    }
    
    fn process_bind(&mut self, id: &Identifier, arg_name: &str, value: CapsulePtr) -> JuizResult<CapsulePtr> {
        let arg = vec!(("arg_name", jvalue!(arg_name)), ("value", capsule_to_value(value)?));
        self.update_by_id("process", "bind", arg.into(), id)
    }
}


impl ContainerBrokerProxy for MessengerBrokerProxy {
    fn container_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        let capsule = self.read_by_id("container", "profile_full", id)?;
        capsule_to_value(capsule)
    }

    fn container_list(&self) -> JuizResult<Value> {
        capsule_to_value(self.read("container", "list")?)
    }
}

impl ContainerProcessBrokerProxy for MessengerBrokerProxy {
    fn container_process_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        capsule_to_value(self.read_by_id("container_process", "profile_full", id)?)
    }

    fn container_process_list(&self) -> JuizResult<Value> {
        capsule_to_value(self.read("container_process", "list")?)
    }
    
    fn container_process_call(&self, id: &Identifier, args: CapsuleMap) -> JuizResult<CapsulePtr> {       
        self.update_output_by_id("container_process", "call", args, id)
    }
    
    fn container_process_execute(&self, id: &Identifier) -> JuizResult<CapsulePtr> {
        self.update_output_by_id("container_process", "execute", CapsuleMap::new(), id)
    }
}

impl ExecutionContextBrokerProxy for MessengerBrokerProxy {

    fn ec_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        capsule_to_value(self.read_by_id("execution_context", "profile_full", id)?)
    }

    fn ec_list(&self) -> JuizResult<Value> {
        capsule_to_value(self.read("execution_context", "list")?)
    }

    fn ec_get_state(&self, id: &Identifier) -> JuizResult<Value> {
        capsule_to_value(self.read_by_id("execution_context", "get_state", id)?)
    }

    fn ec_start(&mut self, id: &Identifier) -> JuizResult<Value> {
        capsule_to_value(self.update_by_id("execution_context", "start",  CapsuleMap::new(), id)?)
    }

    fn ec_stop(&mut self, id: &Identifier) -> JuizResult<Value> {
        capsule_to_value(self.update_by_id("execution_context", "stop", CapsuleMap::new(), id)?)
    }
}

impl BrokerBrokerProxy for MessengerBrokerProxy {
    fn broker_list(&self) -> JuizResult<Value> {
        capsule_to_value(self.read("broker", "list")?)
    }

    fn broker_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        capsule_to_value(self.read_by_id("broker", "profile_full", id)?)
    }
}

impl ConnectionBrokerProxy for MessengerBrokerProxy {
    fn connection_list(&self) -> JuizResult<Value> {
        capsule_to_value(self.read("connection", "list")?)
    }

    fn connection_profile_full(&self, id: &Identifier) -> JuizResult<Value> {
        capsule_to_value(self.read_by_id("connection", "profile_full", id)?)
    }

    fn connection_create(&mut self, manifest: Value) -> JuizResult<Value> {
        capsule_to_value(self.create("connection", "create", manifest.try_into()?)?)
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