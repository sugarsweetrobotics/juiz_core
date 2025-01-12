

use std::sync::{Arc, Mutex};
use juiz_sdk::{anyhow, connection_identifier::ConnectionIdentifier, connections::{ConnectionManifest, ConnectionProfile}, manifests::ProcessProfile, process_identifier::ProcessIdentifier};
use crate::prelude::*;
use juiz_sdk::prelude::*;
use crate::brokers::BrokerProxy;

#[allow(unused)]
pub struct ProcessProxy {
    // core: ObjectCore,
    broker_proxy: Arc<Mutex<dyn BrokerProxy>>,
    identifier: ProcessIdentifier,
    //profile: ProcessProfile,
    class_name_str: String,
}

impl ProcessProxy {

    pub fn new(class_name: JuizObjectClass, identifier: ProcessIdentifier, broker_proxy: Arc<Mutex<dyn BrokerProxy>>) -> JuizResult<ProcessPtr> {
        log::trace!("ProcessProxy::new({class_name:?}, {identifier}, broker_proxy) called");
        // let id_struct = IdentifierStruct::try_from(identifier.clone())?;
        let class_name_str = match class_name {
            JuizObjectClass::Process(_) => Ok("process"),
            JuizObjectClass::ContainerProcess(_) => Ok("container_process"),
            _ => {Err(anyhow::Error::from(JuizError::ProcessProxyCanNotAcceptClassError{class_name: class_name.as_str().to_string()}))}
        }?;
        Ok(ProcessPtr::new(ProcessProxy{
            broker_proxy,
            identifier,
            class_name_str: class_name_str.to_string(),
        }))
    }
}

// impl JuizObjectCoreHolder for ProcessProxy {
//     fn core(&self) -> &ObjectCore {
//         &self.core
//     }
// }
// impl JuizObject for ProcessProxy {

//     fn profile_full(&self) -> JuizResult<Value> {
//         let id = self.identifier();
//         log::trace!("ProcessProxy({id})::profile_full() called");
//         juiz_lock(&self.broker_proxy)?.any_process_profile_full(self.identifier())
//     }
// }

impl std::fmt::Debug for ProcessProxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProcessProxy").field("identifier", &self.identifier).field("class_name_str", &self.class_name_str).finish()
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

    fn profile(&self) -> JuizResult<ProcessProfile> {
        juiz_lock(&self.broker_proxy)?.any_process_profile_full(&self.identifier())
    }

    fn invoke<'b>(&self) -> JuizResult<CapsulePtr> {
        todo!()
    }

    fn execute(&self) -> JuizResult<CapsulePtr> {
        todo!()
    }

    fn push_by(&self, arg_name: &str, value: CapsulePtr) -> JuizResult<CapsulePtr> {
        juiz_lock(&self.broker_proxy)?.process_push_by(&self.identifier(), arg_name.to_owned(), value)
    }

    fn get_output(&self) -> CapsulePtr {
        todo!()
    }

    fn notify_connected_from<'b>(&'b mut self, source: ProcessPtr, manifest: &ConnectionManifest) -> JuizResult<ConnectionProfile> {
        log::trace!("ProcessProxy::notify_connected_from() called");
        juiz_lock(&self.broker_proxy)?.process_notify_connected_from(&manifest)
    }

    fn try_connect_to(&mut self, destination: ProcessPtr, manifest: &ConnectionManifest) -> JuizResult<ConnectionManifest> {
        log::trace!("ProcessProxy::try_connect_to() called");
        juiz_lock(&self.broker_proxy)?.process_try_connect_to(&manifest)
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
    
    fn identifier(&self) -> ProcessIdentifier {
        todo!()
    }
    
}
