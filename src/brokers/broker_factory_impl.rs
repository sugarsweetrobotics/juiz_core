
use std::sync::{Arc, Mutex};

use crate::core::CoreBrokerPtr;
use crate::prelude::*;
use crate::{object::{JuizObjectCoreHolder, ObjectCore, JuizObjectClass}, value::obj_get_str};

use super::broker_ptr::BrokerPtr;




pub struct BrokerFactoryImpl {
    core: ObjectCore,
    core_broker: CoreBrokerPtr,
    create_function: fn(core_broker: CoreBrokerPtr, Value)->JuizResult<BrokerPtr>,
}

impl BrokerFactoryImpl {
    pub fn new(core_broker: CoreBrokerPtr, manifest: Value, create_function: fn(core_broker: CoreBrokerPtr, Value)->JuizResult<BrokerPtr>) -> JuizResult<Arc<Mutex<BrokerFactoryImpl>>> {
        let class_name = "BrokerFactoryImpl";
        let type_name = obj_get_str(&manifest, "type_name")?;
        
        Ok(Arc::new(Mutex::new(BrokerFactoryImpl{
            core_broker: core_broker.clone(),
            core: ObjectCore::create_factory(JuizObjectClass::BrokerFactory(class_name), type_name),
            create_function: create_function,
        })))
    }
}

impl JuizObjectCoreHolder for BrokerFactoryImpl {
    
    fn core(&self) -> &crate::object::ObjectCore {
        &self.core
    }
}

impl JuizObject for BrokerFactoryImpl {

}

impl BrokerFactory for BrokerFactoryImpl {
    fn create_broker(&self, manifest: Value) -> JuizResult<BrokerPtr> {
        (self.create_function)(self.core_broker.clone(), manifest)
    }
}

pub fn create_broker_factory_impl(core_broker: CoreBrokerPtr, manifest: Value, create_broker_function: fn(core_broker: CoreBrokerPtr, Value)->JuizResult<BrokerPtr>) -> JuizResult<Arc<Mutex<dyn BrokerFactory>>>{
    Ok(BrokerFactoryImpl::new(core_broker, manifest, create_broker_function)?)
}

impl Drop for BrokerFactoryImpl {
    fn drop(&mut self) {
        log::trace!("BrokerFactoryImpl({})::drop() called", self.type_name());
    }
}