use std::{sync::{Arc, Mutex, atomic::AtomicBool, mpsc, MutexGuard}, thread::Builder, time::Duration, ops::Deref};
use crate::{jvalue, Broker, JuizResult, JuizError, Value, value::{obj_get_str, obj_get, obj_merge}, utils::juiz_lock, BrokerProxy, JuizObject, object::{ObjectCore, JuizObjectClass, JuizObjectCoreHolder}, CoreBroker, Process, BrokerFactory, brokers::messenger_broker_factory::create_messenger_broker_factory};

use std::sync::atomic::Ordering::SeqCst;

use super::messenger_broker::{MessengerBrokerCore, MessengerBrokerCoreFactory};

#[allow(dead_code)]
pub struct LocalBrokerCore {
    sender_receiver: Arc<Mutex<SenderReceiverPair>>,
}

impl MessengerBrokerCore for LocalBrokerCore {
    fn receive_and_send(&self, timeout: Duration, func: Arc<Mutex<dyn Fn(Value)->JuizResult<Value>>>) -> JuizResult<Value> {
        let sendr_recvr = juiz_lock(&self.sender_receiver)?;
        let SenderReceiverPair(sendr, recvr) = sendr_recvr.deref();
        match recvr.recv_timeout(timeout) {
            Err(_e) => {
                Ok(jvalue!({}))
            },
            Ok(value) => {
                let ret_value = (juiz_lock(&func)?)(value)?;
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

    fn create(&self) -> JuizResult<Arc<Mutex<dyn super::messenger_broker::MessengerBrokerCore>>> {
        LocalBrokerCore::new(self.sender_receiver.clone())
    }
}


pub fn create_local_broker_factory(core_broker: Arc<Mutex<CoreBroker>>, sender_receiver: Arc<Mutex<SenderReceiverPair>>) -> JuizResult<Arc<Mutex<dyn BrokerFactory>>> {
    log::trace!("create_local_broker_factory called");
    create_messenger_broker_factory("LocalBrokerProxyFactory", "local", core_broker, LocalBrokerCoreFactory::new(sender_receiver))
}
