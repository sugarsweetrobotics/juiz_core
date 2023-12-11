
use std::sync::{Mutex, Arc};
use crate::brokers::LocalBrokerProxy;
use crate::{jvalue, Value, JuizResult, BrokerProxy};

use super::broker_proxy_factory::BrokerProxyFactory;
use super::local_broker::SenderReceiverPair;


pub struct LocalBrokerProxyFactory {
    broker_proxy: Arc<Mutex<dyn BrokerProxy>>,
}

pub fn create_local_broker_factory(sender_receiver: Arc<Mutex<SenderReceiverPair>>) -> JuizResult<Arc<Mutex<dyn BrokerProxyFactory>>> {
    log::trace!("create_local_broker_factory called");
    LocalBrokerProxyFactory::new(sender_receiver)
}

impl LocalBrokerProxyFactory {

    pub fn new(sender_receiver: Arc<Mutex<SenderReceiverPair>> ) -> JuizResult<Arc<Mutex<dyn BrokerProxyFactory>>> {
        Ok(Arc::new(Mutex::new(
            LocalBrokerProxyFactory{
                broker_proxy: LocalBrokerProxy::new(sender_receiver)?,
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
        Ok(Arc::clone(&self.broker_proxy))
    }

    fn profile_full(&self) -> JuizResult<Value> {
        Ok(jvalue!({
            "type_name": self.type_name()
        }))
    }
    
}
