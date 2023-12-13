
use std::sync::{Mutex, Arc};
use crate::object::{ObjectCore, JuizObjectClass};
use crate::JuizResult;

use super::broker_proxy_factory::BrokerProxyFactory;
use super::local_broker::SenderReceiverPair;
use super::local_broker_proxy::LocalBrokerProxyCoreFactory;
use super::messenger_broker_proxy_factory::MessengerBrokerProxyFactory;

pub type LocalBrokerProxyFactory = MessengerBrokerProxyFactory;

pub fn create_local_broker_proxy_factory(sender_receiver: Arc<Mutex<SenderReceiverPair>>) -> JuizResult<Arc<Mutex<dyn BrokerProxyFactory>>> {
    log::trace!("create_local_broker_factory called");
    let type_name = "local";
    MessengerBrokerProxyFactory::new(
        ObjectCore::create_factory(JuizObjectClass::BrokerProxyFactory("LocalBrokerProxyFactory"), type_name),
        LocalBrokerProxyCoreFactory::new(sender_receiver)?,
    )
}

