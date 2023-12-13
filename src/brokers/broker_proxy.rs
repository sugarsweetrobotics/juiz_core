
use crate::{Identifier, Value, JuizResult, JuizObject, Process};


pub trait SystemBrokerProxy {
    fn system_profile_full(&self) -> JuizResult<Value>;
}

pub trait ProcessBrokerProxy {

    fn process_profile_full(&self, id: &Identifier) -> JuizResult<Value>;

    fn process_call(&self, id: &Identifier, _args: Value) -> JuizResult<Value>;

    fn process_execute(&self, id: &Identifier) -> JuizResult<Value>;

    fn process_connect_to(&mut self, source_process_id: &Identifier, arg_name: &String, target_process_id: &Identifier, manifest: Value) -> JuizResult<Value>;

}


pub trait BrokerProxy : Send + JuizObject + SystemBrokerProxy + ProcessBrokerProxy {

    fn is_in_charge_for_process(&self, _id: &Identifier) -> JuizResult<bool>;
}