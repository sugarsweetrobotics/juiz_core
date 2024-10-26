

use std::fmt::Display;
use std::sync::{Arc, Mutex};

use crate::prelude::*;

#[allow(unused)]
pub struct ContainerProxy {
    core: ObjectCore,
    broker_proxy: Arc<Mutex<dyn BrokerProxy>>,
    identifier: Identifier,
    class_name_str: String,
}

impl ContainerProxy {

    pub fn new(class_name: JuizObjectClass, identifier: &Identifier, broker_proxy: Arc<Mutex<dyn BrokerProxy>>) -> JuizResult<Self> {
        log::trace!("ContainerProxy::new({class_name:?}, {identifier}, broker_proxy) called");
        let id_struct = IdentifierStruct::try_from(identifier.clone())?;
        let class_name_str = "container";
        Ok(ContainerProxy{
            core: ObjectCore::new(identifier.clone(), class_name, id_struct.type_name.as_str(), id_struct.object_name.as_str(), id_struct.broker_name.as_str(), id_struct.broker_type_name.as_str()),
            broker_proxy,
            identifier: identifier.clone(),
            class_name_str: class_name_str.to_owned(),
        })
    }
}

impl JuizObjectCoreHolder for ContainerProxy {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}

impl JuizObject for ContainerProxy {

    fn profile_full(&self) -> JuizResult<Value> {
        let id = self.identifier();
        log::trace!("ContainerProxy({id})::profile_full() called");
        juiz_lock(&self.broker_proxy)?.container_profile_full(self.identifier())
    }
}

impl Display for ContainerProxy {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Container for ContainerProxy {
    fn manifest(&self) -> &ContainerManifest {
        todo!()
    }
    
    fn process(&self, _name_or_id: &String) -> Option<ProcessPtr> {
        todo!()
    }

    fn purge_process(&mut self, _name_or_id: &String) -> JuizResult<()> {
        todo!()
    }

    fn clear(&mut self) -> JuizResult<()> {
        todo!()
    }
    
    fn processes(&self) -> Vec<ProcessPtr> {
        todo!()
    }
    
    fn register_process(&mut self, _p: ProcessPtr) -> JuizResult<ProcessPtr> {
        todo!()
    }
}
