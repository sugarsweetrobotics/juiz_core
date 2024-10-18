use std::sync::{Arc, Mutex};

use crate::prelude::*;
use crate::brokers::Broker;

use super::broker_ptr::BrokerPtr;





pub trait BrokerFactory : JuizObject {

    fn create_broker(&self, manifest: Value) -> JuizResult<BrokerPtr>;
}
