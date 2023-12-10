
use std::sync::{Mutex, Arc};
use crate::brokers::LocalBrokerProxy;
use crate::{jvalue, Value, JuizResult, CoreBroker, BrokerProxy};

use super::broker_proxy_factory::BrokerProxyFactory;


pub struct LocalBrokerProxyFactory {
    core_broker: Arc<Mutex<CoreBroker>>
}

pub fn create_local_broker_factory(core_broker: Arc<Mutex<CoreBroker>>) -> JuizResult<Arc<Mutex<dyn BrokerProxyFactory>>> {
    log::trace!("create_local_broker_factory called");
    LocalBrokerProxyFactory::new(core_broker)
}

impl LocalBrokerProxyFactory {

    pub fn new(core_broker: Arc<Mutex<CoreBroker>>) -> JuizResult<Arc<Mutex<dyn BrokerProxyFactory>>> {
        Ok(Arc::new(Mutex::new(
            LocalBrokerProxyFactory{
                core_broker
            }
        )))
    }
    /*
    fn apply_default_manifest(&self, manifest: Value) -> Result<Value, JuizError> {
        let mut new_manifest = self.manifest.clone();
        for (k, v) in manifest.as_object().unwrap().iter() {
            new_manifest.as_object_mut().unwrap().insert(k.to_owned(), v.clone());
        }
        return Ok(new_manifest);
    }
    */
}

impl BrokerProxyFactory for LocalBrokerProxyFactory {


    fn type_name(&self) -> &str {
        "local"
    }

    fn create_broker_proxy(&self, manifest: Value) -> JuizResult<Arc<Mutex<dyn BrokerProxy>>>{
        log::trace!("LocalBrokerFactory::create_broker_proxy(manifest={}) called", manifest);
        Ok(
            LocalBrokerProxy::new(
                Arc::clone(&self.core_broker))?
        )
    }


    fn profile_full(&self) -> JuizResult<Value> {
        Ok(jvalue!({
            "type_name": self.type_name()
        }))
    }
    
}
