

use std::sync::{Arc, Mutex};
use juiz_sdk::anyhow;
use crate::prelude::*;
use juiz_sdk::prelude::*;
use crate::brokers::BrokerProxy;

#[allow(unused)]
pub struct ProcessProxy {
    core: ObjectCore,
    broker_proxy: Arc<Mutex<dyn BrokerProxy>>,
    identifier: Identifier,
    class_name_str: String,
}

impl ProcessProxy {

    pub fn new(class_name: JuizObjectClass, identifier: &Identifier, broker_proxy: Arc<Mutex<dyn BrokerProxy>>) -> JuizResult<ProcessPtr> {
        log::trace!("ProcessProxy::new({class_name:?}, {identifier}, broker_proxy) called");
        let id_struct = IdentifierStruct::try_from(identifier.clone())?;
        let class_name_str = match class_name {
            JuizObjectClass::Process(_) => Ok("process"),
            JuizObjectClass::ContainerProcess(_) => Ok("container_process"),
            _ => {Err(anyhow::Error::from(JuizError::ProcessProxyCanNotAcceptClassError{class_name: class_name.as_str().to_string()}))}
        }?;
        log::trace!("id_struct: {id_struct:?}");
        Ok(ProcessPtr::new(ProcessProxy{
            core: ObjectCore::new(identifier.clone(), class_name, id_struct.type_name.as_str(), id_struct.object_name.as_str(), id_struct.broker_name.as_str(), id_struct.broker_type_name.as_str()),
            broker_proxy,
            identifier: identifier.clone(),
            class_name_str: class_name_str.to_string(),
        }))
    }
}

impl JuizObjectCoreHolder for ProcessProxy {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}

impl JuizObject for ProcessProxy {

    fn profile_full(&self) -> JuizResult<Value> {
        let id = self.identifier();
        log::trace!("ProcessProxy({id})::profile_full() called");
        juiz_lock(&self.broker_proxy)?.any_process_profile_full(self.identifier())
    }
}

impl Process for ProcessProxy {
    
    fn call(&self, args: CapsuleMap) -> JuizResult<CapsulePtr> {
        let id = self.identifier();
        log::trace!("ProcessProxy({id})::call() called");
        let result = juiz_lock(&self.broker_proxy)?.any_process_call(&self.identifier(), args);
        log::trace!(" - return: {result:?}");
        return result;
    }

    fn is_updated(& self) -> JuizResult<bool> {
        todo!()
    }

    fn manifest(&self) -> &ProcessManifest {
        todo!()
    }

    fn invoke<'b>(&self) -> JuizResult<CapsulePtr> {
        todo!()
    }

    fn execute(&self) -> JuizResult<CapsulePtr> {
        todo!()
    }

    fn push_by(&self, _arg_name: &str, _value: CapsulePtr) -> JuizResult<CapsulePtr> {
        todo!()
    }

    fn get_output(&self) -> CapsulePtr {
        todo!()
    }

    fn notify_connected_from<'b>(&'b mut self, source: ProcessPtr, arg_name: &str, manifest: Value) -> JuizResult<Value> {
        log::trace!("ProcessProxy::notify_connected_from() called");
        juiz_lock(&self.broker_proxy)?.process_notify_connected_from(source.identifier(), arg_name, &self.identifier(), manifest)
    }

    fn try_connect_to(&mut self, destination: ProcessPtr, arg_name: &str,manifest: Value) -> JuizResult<Value> {
        log::trace!("ProcessProxy::try_connect_to() called");
        juiz_lock(&self.broker_proxy)?.process_try_connect_to(&self.identifier(), arg_name, destination.identifier(), manifest)
    }

    fn source_connections(&self) -> JuizResult<Vec<&Box<dyn SourceConnection>>> {
        todo!()
    }

    fn destination_connections(&self) -> JuizResult<Vec<&Box<dyn DestinationConnection>>> {
        todo!()
    }


    fn p_apply(&mut self, arg_name: &str, value: CapsulePtr) -> JuizResult<CapsulePtr> {
        juiz_lock(&self.broker_proxy)?.process_p_apply(&self.identifier(), arg_name, value)
    }
    
    fn purge(&mut self) -> JuizResult<()> {

        log::trace!("ProcessProxy({})::purge() called", self.identifier());
        todo!()
    }
    
}
