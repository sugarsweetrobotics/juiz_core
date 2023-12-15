use std::sync::{Arc, Mutex};

use crate::{Value, JuizResult, brokers::Broker, JuizObject, object::{JuizObjectCoreHolder, ObjectCore, JuizObjectClass}, value::obj_get_str};

use super::BrokerProxy;




pub trait BrokerFactory : JuizObject {

    fn create_broker(&self, manifest: Value) -> JuizResult<Arc<Mutex<dyn Broker>>>;
}

pub struct BrokerFactoryImpl {
    core: ObjectCore,
    core_broker: Arc<Mutex<dyn BrokerProxy>>,
    create_function: fn(core_broker: Arc<Mutex<dyn BrokerProxy>>, Value)->JuizResult<Arc<Mutex<dyn Broker>>>,
}

impl BrokerFactoryImpl {
    pub fn new(core_broker: Arc<Mutex<dyn BrokerProxy>>, manifest: Value, create_function: fn(core_broker: Arc<Mutex<dyn BrokerProxy>>, Value)->JuizResult<Arc<Mutex<dyn Broker>>>) -> JuizResult<Arc<Mutex<BrokerFactoryImpl>>> {
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
    fn create_broker(&self, manifest: Value) -> JuizResult<Arc<Mutex<dyn Broker>>> {
        (self.create_function)(self.core_broker.clone(), manifest)
    }
}

pub fn create_broker_factory_impl(core_broker: Arc<Mutex<dyn BrokerProxy>>, manifest: Value, create_broker_function: fn(core_broker: Arc<Mutex<dyn BrokerProxy>>, Value)->JuizResult<Arc<Mutex<dyn Broker>>>) -> JuizResult<Arc<Mutex<dyn BrokerFactory>>>{
    Ok(BrokerFactoryImpl::new(core_broker, manifest, create_broker_function)?)
}