
use std::sync::{Mutex, Arc};
use crate::{jvalue, Value, JuizResult, CoreBroker};

use super::broker_factory::BrokerFactory;
use super::local_broker::LocalBroker;


pub struct LocalBrokerFactory {
    core_broker: Arc<Mutex<CoreBroker>>
}

impl LocalBrokerFactory {

    pub fn new(core_broker: Arc<Mutex<CoreBroker>>) -> JuizResult<Arc<Mutex<dyn BrokerFactory>>> {
        Ok(Arc::new(Mutex::new(
            LocalBrokerFactory{
                core_broker
            }
        )))
    }
}

impl BrokerFactory for LocalBrokerFactory {


    fn type_name(&self) -> &str {
        "local"
    }


    fn profile_full(&self) -> JuizResult<Value> {
        Ok(jvalue!({
            "type_name": self.type_name()
        }))
    }

    fn create_broker(&self, _manifest: Value) -> JuizResult<Arc<Mutex<dyn crate::Broker>>> {
        Ok(LocalBroker::new(Arc::clone(&self.core_broker))?)
    }
    
}
