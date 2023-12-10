use std::sync::{Arc, Mutex};

use crate::{Value, JuizResult, BrokerProxy};




pub trait BrokerProxyFactory {

    fn type_name(&self) -> &str;

    fn create_broker_proxy(&self, manifest: Value) -> JuizResult<Arc<Mutex<dyn BrokerProxy>>>;

    fn profile_full(&self) -> JuizResult<Value>;

}