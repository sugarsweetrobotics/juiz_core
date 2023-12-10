use std::sync::{Arc, Mutex};

use crate::{BrokerProxy, CoreBroker, JuizResult, utils::juiz_lock, Identifier};




pub struct LocalBrokerProxy {
    core_broker: Arc<Mutex<CoreBroker>>
}

impl LocalBrokerProxy {

    pub fn new(core_broker: Arc<Mutex<CoreBroker>>) -> JuizResult<Arc<Mutex<dyn BrokerProxy>>>{
        Ok(Arc::new(Mutex::new(LocalBrokerProxy{core_broker})))
    }
}

impl BrokerProxy for LocalBrokerProxy {
    fn is_in_charge_for_process(&self, id: &Identifier) -> JuizResult<bool> {
        juiz_lock(&self.core_broker)?.is_in_charge_for_process(id)
    }

    fn call_process(&self, id: &Identifier, args: crate::Value) -> crate::JuizResult<crate::Value> {
        juiz_lock(&self.core_broker)?.call_process(id, args)
    }

    fn execute_process(&self, id: &crate::Identifier) -> crate::JuizResult<crate::Value> {
        juiz_lock(&self.core_broker)?.execute_process(id)
    }

    fn connect_process_to(&mut self, source_process_id: &crate::Identifier, arg_name: &String, target_process_id: &crate::Identifier, manifest: crate::Value) -> crate::JuizResult<crate::Value> {
        juiz_lock(&self.core_broker)?.connect_process_to(source_process_id, arg_name, target_process_id, manifest)
    }

    fn profile_full(&self) -> crate::JuizResult<crate::Value> {
        juiz_lock(&self.core_broker)?.profile_full()
    }
}