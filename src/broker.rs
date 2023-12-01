
use std::sync::{Arc, Mutex};
use crate::{Identifier, Value, Process, JuizError};

pub trait Broker {

    fn is_in_charge_for_process(&self, _id: &Identifier) -> bool {
        false
    }

    fn process(&self, id: &Identifier) -> Result<Arc<Mutex<dyn Process>>, JuizError>;

    fn call_process(&self, _id: &Identifier, _args: Value) -> Result<Value, JuizError>;

    fn execute_process(&self, _id: &Identifier) -> Result<Value, JuizError>;

    fn connect_process_to(&mut self, source_process_id: &Identifier, arg_name: &String, target_process_id: &Identifier, manifest: Value) -> Result<Value, JuizError>;

    fn create_process(&mut self, manifest: Value) -> Result<Arc<Mutex<dyn Process>>, JuizError>;
}