use std::{sync::{Arc, Mutex, mpsc}, time::Duration, ops::Deref};


use crate::{brokers::create_messenger_broker_factory, jvalue, value::{Capsule, CapsuleMap}, utils::juiz_lock, CapsulePtr, CoreBroker, JuizError, JuizResult, Value};
use crate::brokers::{BrokerFactory, MessengerBroker, MessengerBrokerCore, MessengerBrokerCoreFactory};


pub struct ByteSenderReceiverPair(pub mpsc::Sender<Vec<u8>>, pub mpsc::Receiver<Vec<u8>>);
pub struct BrokerSideSenderReceiverPair(pub mpsc::Sender<CapsulePtr>, pub mpsc::Receiver<CapsuleMap>);
pub struct ProxySideSenderReceiverPair(pub mpsc::Sender<CapsuleMap>, pub mpsc::Receiver<CapsulePtr>);
pub type LocalBroker = MessengerBroker;

#[allow(dead_code)]
pub struct LocalBrokerCore {
    sender_receiver: Arc<Mutex<BrokerSideSenderReceiverPair>>,
    byte_sender_receiver: Arc<Mutex<ByteSenderReceiverPair>>,
}

impl MessengerBrokerCore for LocalBrokerCore {
    fn receive_and_send(&self, timeout: Duration, func: Arc<Mutex<dyn Fn(CapsuleMap)->JuizResult<CapsulePtr> >>) -> JuizResult<Capsule> {
        //log::trace!("LocalBrokerCore::receive_and_send() called");
        let sendr_recvr = juiz_lock(&self.sender_receiver)?;
        let BrokerSideSenderReceiverPair(sendr, recvr) = sendr_recvr.deref();
        match recvr.recv_timeout(timeout) {
            Err(_e) => {
                //log::error!("Timeout Error.");
                //log::trace!("LocalBrokerCore::receive_and_send() exit");
                Ok(Capsule::from(jvalue!({})))
            },
            Ok(value) => {
                log::trace!("LocalBrokerCore::receive_and_send() received some data.");
                let ret_value = match (juiz_lock(&func)?)(value) {
                    Ok(v) => {
                        v
                    },
                    Err(e) => {
                        log::error!("User function call in MessengerBrokerCore::receive_and_send() failed. Error is {}", e.to_string());
                        return Err(e);
                    }
                };
                log::trace!("LocalBrokerCore now sending back data.");
                match sendr.send(ret_value) {
                    Err(e) => {
                        log::error!("Error({e:?}) in LocalBroker::routine()");
                        log::trace!("LocalBrokerCore::receive_and_send() exit");
                        Err(anyhow::Error::from(JuizError::BrokerSendError{}))
                    },
                    Ok(()) => {
                        log::trace!("LocalBrokerCore collectly sent data.");
                        log::trace!("LocalBrokerCore::receive_and_send() exit");
                        Ok(jvalue!({}).into())
                    }
                }
            }
        }
    }
}

impl LocalBrokerCore {

    pub fn new(sender_receiver: Arc<Mutex<BrokerSideSenderReceiverPair>>, byte_sender_receiver: Arc<Mutex<ByteSenderReceiverPair>> ) -> JuizResult<Arc<Mutex<dyn MessengerBrokerCore>>>{
        Ok(Arc::new(Mutex::new(LocalBrokerCore{
            byte_sender_receiver, 
            sender_receiver})))
    }
}


pub struct LocalBrokerCoreFactory {
    sender_receiver: Arc<Mutex<BrokerSideSenderReceiverPair>>,
    byte_sender_receiver: Arc<Mutex<ByteSenderReceiverPair>>,
}

impl LocalBrokerCoreFactory {
    pub fn new(sender_receiver: Arc<Mutex<BrokerSideSenderReceiverPair>>, byte_sender_receiver: Arc<Mutex<ByteSenderReceiverPair>>) -> Box<LocalBrokerCoreFactory> {
        Box::new(LocalBrokerCoreFactory{sender_receiver, byte_sender_receiver})
    }
}


impl MessengerBrokerCoreFactory for LocalBrokerCoreFactory {

    fn create(&self, _manifest: &Value) -> JuizResult<Arc<Mutex<dyn MessengerBrokerCore>>> {
        LocalBrokerCore::new(self.sender_receiver.clone(), self.byte_sender_receiver.clone())
    }
}


pub fn create_local_broker_factory(core_broker: Arc<Mutex<CoreBroker>>, sender_receiver: Arc<Mutex<BrokerSideSenderReceiverPair>>, byte_sender_receiver: Arc<Mutex<ByteSenderReceiverPair>>) -> JuizResult<Arc<Mutex<dyn BrokerFactory>>> {
    log::trace!("create_local_broker_factory called");
    create_messenger_broker_factory("LocalBrokerProxyFactory", "local", core_broker, LocalBrokerCoreFactory::new(sender_receiver, byte_sender_receiver))
}
