
use std::sync::{Mutex, Arc};
use anyhow::Context;

use crate::core::core_broker::CoreBroker;
use crate::object::{ObjectCore, JuizObjectClass, JuizObjectCoreHolder};

use crate::value::obj_get_str;

use crate::prelude::*;

use crate::brokers::{MessengerBroker, Broker, BrokerFactory, MessengerBrokerCoreFactory};


pub struct MessengerBrokerFactory {
    core: ObjectCore, 
    core_broker: Arc<Mutex<CoreBroker>>,
    broker_impl_class_name: &'static str,
    core_factory: Box<dyn MessengerBrokerCoreFactory>,
}

impl MessengerBrokerFactory {

    pub fn new(broker_impl_class_name: &'static str, type_name: &str, core_broker: Arc<Mutex<CoreBroker>>, core_factory: Box<dyn MessengerBrokerCoreFactory>) -> JuizResult<Arc<Mutex<dyn BrokerFactory>>> {
        let lbf = Arc::new(Mutex::new(MessengerBrokerFactory{
            core: ObjectCore::create_factory(JuizObjectClass::BrokerFactory(broker_impl_class_name), type_name), 
            core_broker:core_broker.clone(), 
            broker_impl_class_name: broker_impl_class_name,
            core_factory,
        }));
        Ok(lbf)
    }
}

impl JuizObjectCoreHolder for MessengerBrokerFactory {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}

impl JuizObject for MessengerBrokerFactory {}

impl BrokerFactory for MessengerBrokerFactory {

    fn create_broker(&self, manifest: Value) -> JuizResult<Arc<Mutex<dyn Broker>>> {
        log::trace!("MessengerBrokerFactory::create_broker(manifest={manifest}) called");
        let object_name = obj_get_str(&manifest, "name").context("Failed when getting 'name' property from manifest in MessngerBrokerFactory::create_broker()")?;
        Ok(MessengerBroker::new(
                   self.broker_impl_class_name,
                    self.type_name(),
                    object_name,
                    self.core_broker.clone(),
                    self.core_factory.create(&manifest).context("MessengerCoreFactory::create() failed in MessngerBrokerFactory::create_broker()")?)?,
        )
    }
    
}



pub fn create_messenger_broker_factory(impl_class_name: &'static str, type_name: &str, core_broker: Arc<Mutex<CoreBroker>>, core_factory: Box<dyn MessengerBrokerCoreFactory>) -> JuizResult<Arc<Mutex<dyn BrokerFactory>>> {
    log::trace!("create_messenger_broker_factory called");
    MessengerBrokerFactory::new(
    impl_class_name,
        type_name, 
        core_broker,
        core_factory,
    )
}