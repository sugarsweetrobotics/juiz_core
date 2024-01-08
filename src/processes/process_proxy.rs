

use std::sync::Arc;
use std::sync::Mutex;

use crate::JuizError;
use crate::object::JuizObjectClass;
use crate::utils::juiz_lock;
use crate::{JuizObject, JuizResult, Process, Value};
use crate::brokers::BrokerProxy;
use crate::object::JuizObjectCoreHolder;
use crate::object::ObjectCore;
use crate::identifier::*;

use super::Output;

#[allow(unused)]
pub struct ProcessProxy {
    core: ObjectCore,
    broker_proxy: Arc<Mutex<dyn BrokerProxy>>,
    identifier: Identifier,
    class_name_str: String,
}

impl ProcessProxy {

    pub fn new(class_name: JuizObjectClass, identifier: &Identifier, broker_proxy: Arc<Mutex<dyn BrokerProxy>>) -> JuizResult<Arc<Mutex<ProcessProxy>>> {
        let id_struct = IdentifierStruct::from(identifier.clone());
        let class_name_str = match class_name {
            JuizObjectClass::Process(_) => Ok("process"),
            JuizObjectClass::ContainerProcess(_) => Ok("container_process"),
            _ => {Err(anyhow::Error::from(JuizError::ProcessProxyCanNotAcceptClassError{class_name: class_name.as_str().to_string()}))}
        }?;
        Ok(Arc::new(Mutex::new(ProcessProxy{
            core: ObjectCore::create(class_name, id_struct.type_name, id_struct.object_name),
            broker_proxy,
            identifier: identifier.clone(),
            class_name_str: class_name_str.to_string(),
        })))
    }
}

impl JuizObjectCoreHolder for ProcessProxy {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}

impl JuizObject for ProcessProxy {

    fn profile_full(&self) -> JuizResult<Value> {
        juiz_lock(&self.broker_proxy)?.any_process_profile_full(self.identifier())
        /*
        match self.class_name_str.as_str() {
            "process" => juiz_lock(&self.broker_proxy)?.process_profile_full(self.identifier()),
            "container_process" => juiz_lock(&self.broker_proxy)?.container_process_profile_full(self.identifier()),
            _ => { Err(anyhow::Error::from(JuizError::ProcessProxyCanNotAcceptClassError{class_name: self.class_name_str.clone()}))}
        }
        */
    }
}

impl Process for ProcessProxy {
    
    fn call(&self, _args: crate::Value) -> JuizResult<Output> {
        //juiz_lock(&self.broker_proxy)?.any_process_call(args)
        todo!()
    }

    fn is_updated(& self) -> JuizResult<bool> {
        todo!()
    }

    fn is_updated_exclude(& self, _caller_id: &Identifier) -> JuizResult<bool> {
        todo!()
    }

    fn manifest(&self) -> &Value {
        todo!()
    }

    fn invoke<'b>(&self) -> JuizResult<Output> {
        todo!()
    }

    fn invoke_exclude<'b>(&self, _arg_name: &String, _value: Output) -> JuizResult<Output> {
        todo!()
    }

    fn execute(&self) -> JuizResult<Output> {
        todo!()
    }

    fn push_by(&self, _arg_name: &String, _value: &Output) -> JuizResult<Output> {
        todo!()
    }

    fn get_output(&self) -> Option<Output> {
        todo!()
    }

    fn notify_connected_from<'b>(&'b mut self, source: Arc<Mutex<dyn Process>>, arg_name: &String, manifest: Value) -> JuizResult<Value> {
        log::trace!("ProcessProxy::notify_connected_from() called");
        let source_process_id = juiz_lock(&source)?.identifier().clone();
        let destination_process_id = self.identifier();
        juiz_lock(&self.broker_proxy)?.process_notify_connected_from(&source_process_id, arg_name, destination_process_id, manifest)
    }

    fn try_connect_to(&mut self, destination: Arc<Mutex<dyn Process>>, arg_name: &String,manifest: Value) -> JuizResult<Value> {
        log::trace!("ProcessProxy::try_connect_to() called");
        let source_process_id = self.identifier();
        let destination_process_id = juiz_lock(&destination)?.identifier().clone();
        juiz_lock(&self.broker_proxy)?.process_try_connect_to(source_process_id, arg_name, &destination_process_id, manifest)
    }

    fn source_connections(&self) -> JuizResult<Vec<&Box<dyn crate::connections::SourceConnection>>> {
        todo!()
    }

    fn destination_connections(&self) -> JuizResult<Vec<&Box<dyn crate::connections::DestinationConnection>>> {
        todo!()
    }


    
}
