use std::{sync::{Arc, Mutex, mpsc}, time::Duration, ops::Deref};


use crate::{jvalue, JuizResult, JuizError, Value, utils::juiz_lock, CoreBroker, brokers::create_messenger_broker_factory};
use crate::brokers::{BrokerFactory, MessengerBroker, MessengerBrokerCore, MessengerBrokerCoreFactory};


pub type LocalBroker = MessengerBroker;

#[allow(dead_code)]
pub struct LocalBrokerCore {
    sender_receiver: Arc<Mutex<SenderReceiverPair>>,
}

impl MessengerBrokerCore for LocalBrokerCore {
    fn receive_and_send(&self, timeout: Duration, func: Arc<Mutex<dyn Fn(Value)->JuizResult<Value>>>) -> JuizResult<Value> {
        
        //log::trace!("LocalBrokerCore::receive_and_send() called");
        let sendr_recvr = juiz_lock(&self.sender_receiver)?;
        let SenderReceiverPair(sendr, recvr) = sendr_recvr.deref();
        match recvr.recv_timeout(timeout) {
            Err(_e) => {
                Ok(jvalue!({}))
            },
            Ok(value) => {
                log::trace!("LocalBrokerCore::receive_and_send() received some data.");
                
                let ret_value = match (juiz_lock(&func)?)(value) {
                    Ok(v) => v,
                    Err(e) => {
                        log::error!("User function call in MessengerBrokerCore::receive_and_send() failed. Error is {}", e.to_string());
                        return Err(e);
                    }
                };
                log::trace!("LocalBrokerCore now sending back data.");
                match sendr.send(ret_value) {
                    Err(e) => {
                        log::error!("Error({e:?}) in LocalBroker::routine()");
                        Err(anyhow::Error::from(JuizError::BrokerSendError{error: e}))
                    },
                    Ok(()) => Ok(jvalue!({}))
                }
            }
        }
    }
}
pub struct SenderReceiverPair(pub mpsc::Sender<Value>, pub mpsc::Receiver<Value>);

impl LocalBrokerCore {

    pub fn new(sender_receiver: Arc<Mutex<SenderReceiverPair>> ) -> JuizResult<Arc<Mutex<dyn MessengerBrokerCore>>>{
        Ok(Arc::new(Mutex::new(LocalBrokerCore{
                sender_receiver})))
    }
}


pub struct LocalBrokerCoreFactory {
    sender_receiver: Arc<Mutex<SenderReceiverPair>>,
}

impl LocalBrokerCoreFactory {
    pub fn new(sender_receiver: Arc<Mutex<SenderReceiverPair>>) -> Box<LocalBrokerCoreFactory> {
        Box::new(LocalBrokerCoreFactory{sender_receiver})
    }
}


impl MessengerBrokerCoreFactory for LocalBrokerCoreFactory {

    fn create(&self) -> JuizResult<Arc<Mutex<dyn MessengerBrokerCore>>> {
        LocalBrokerCore::new(self.sender_receiver.clone())
    }
}


pub fn create_local_broker_factory(core_broker: Arc<Mutex<CoreBroker>>, sender_receiver: Arc<Mutex<SenderReceiverPair>>) -> JuizResult<Arc<Mutex<dyn BrokerFactory>>> {
    log::trace!("create_local_broker_factory called");
    create_messenger_broker_factory("LocalBrokerProxyFactory", "local", core_broker, LocalBrokerCoreFactory::new(sender_receiver))
}
