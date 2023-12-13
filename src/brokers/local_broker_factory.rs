
use std::sync::{Mutex, Arc};
use crate::object::{ObjectCore, JuizObjectClass, JuizObjectCoreHolder};
use crate::utils::juiz_lock;
use crate::{Value, JuizResult, JuizObject, CoreBroker};

use super::broker_factory::BrokerFactory;
use super::local_broker::{LocalBroker, SenderReceiverPair};
use super::messenger_broker::MessengerBrokerCoreFactory;



impl LocalBrokerFactory {

    pub fn new(core_broker: Arc<Mutex<CoreBroker>>, sender_receiver: Arc<Mutex<SenderReceiverPair>>) -> JuizResult<Arc<Mutex<dyn BrokerFactory>>> {
        let type_name = "local";
        let lbf = Arc::new(Mutex::new(LocalBrokerFactory{
            core: ObjectCore::create_factory(JuizObjectClass::BrokerFactory("LocalBrokerFacotry"), type_name), 
            core_broker:core_broker.clone(), 
            sender_receiver
        }));
        Ok(lbf)
    }
}

impl JuizObjectCoreHolder for LocalBrokerFactory {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}

impl JuizObject for LocalBrokerFactory {}

impl BrokerFactory for LocalBrokerFactory {

    fn create_broker(&self, _manifest: Value) -> JuizResult<Arc<Mutex<dyn crate::Broker>>> {
        log::trace!("LocalBrokerFactory::create_broker(manifest={_manifest}) called");
        Ok(LocalBroker::new(
                    self.core_broker.clone(),
                    Arc::clone(&self.sender_receiver),)?,
        )
    }
    
}
