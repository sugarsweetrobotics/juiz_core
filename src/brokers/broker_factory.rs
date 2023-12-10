use std::sync::{Arc, Mutex};

use crate::{Value, JuizResult, Broker};




pub trait BrokerFactory {

    fn type_name(&self) -> &str;

    fn create_broker(&self, manifest: Value) -> JuizResult<Arc<Mutex<dyn Broker>>>;

    fn profile_full(&self) -> JuizResult<Value>;

}