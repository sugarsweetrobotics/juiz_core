use std::sync::{Arc, Mutex};

use crate::{Value, JuizResult, BrokerProxy, JuizObject};




pub trait BrokerProxyFactory : JuizObject {

    fn create_broker_proxy(&self, manifest: Value) -> JuizResult<Arc<Mutex<dyn BrokerProxy>>>;

    fn profile_full(&self) -> JuizResult<Value>;

}