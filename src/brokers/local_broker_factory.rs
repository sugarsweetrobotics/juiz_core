
use std::sync::{Mutex, Arc};
use crate::{jvalue, Value, JuizResult, BrokerProxy};

use super::broker_factory::BrokerFactory;
use super::local_broker::{LocalBroker, SenderReceiverPair};


pub struct LocalBrokerFactory {
    core_broker: Arc<Mutex<dyn BrokerProxy>>,
    sender_receiver: Arc<Mutex<SenderReceiverPair>>, 
    //sender: Arc<Mutex<mpsc::Sender<Value>>>, 
    //receiver: Arc<Mutex<mpsc::Receiver<Value>>>
}

impl LocalBrokerFactory {

    pub fn new(core_broker: Arc<Mutex<dyn BrokerProxy>>, sender_receiver: Arc<Mutex<SenderReceiverPair>>) -> JuizResult<Arc<Mutex<dyn BrokerFactory>>> {
        Ok(Arc::new(Mutex::new(LocalBrokerFactory{core_broker, sender_receiver})))
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
        //Ok(Arc::clone(&self.broker))
        Ok(LocalBroker::new(
                    Arc::clone(&self.core_broker),
                    Arc::clone(&self.sender_receiver),)?,
                
        )
    }
    
}
