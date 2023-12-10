
use crate::{Identifier, Value, JuizResult};

pub trait BrokerProxy {

    fn is_in_charge_for_process(&self, _id: &Identifier) -> JuizResult<bool>;

    fn call_process(&self, _id: &Identifier, _args: Value) -> JuizResult<Value>;

    fn execute_process(&self, _id: &Identifier) -> JuizResult<Value>;

    fn connect_process_to(&mut self, source_process_id: &Identifier, arg_name: &String, target_process_id: &Identifier, manifest: Value) -> JuizResult<Value>;

    fn profile_full(&self) -> JuizResult<Value>;

}