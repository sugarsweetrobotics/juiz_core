use std::sync::{Arc, Mutex};

use crate::{Value, JuizResult, Broker, JuizObject};




pub trait BrokerFactory : JuizObject {

    fn create_broker(&self, manifest: Value) -> JuizResult<Arc<Mutex<dyn Broker>>>;

}