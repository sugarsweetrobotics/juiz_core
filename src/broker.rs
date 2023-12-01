
use std::sync::Arc;
use std::sync::Mutex;

use crate::identifier::*;
use crate::process::Process;
use crate::value::*;
use crate::error::*;

pub trait Broker {

    fn is_in_charge_for_process(&mut self, _id: &Identifier) -> bool {
        false
    }

    fn call_process(&mut self, _id: &Identifier, _args: Value) -> Result<Value, JuizError> {
        Err(JuizError::NotImplementedError{})
    }

    fn connect_process_to(&mut self, source_process_id: &Identifier, arg_name: &String, target_process_id: &Identifier) -> Result<Value, JuizError>;

    fn create_process(&mut self, manifest: Value) -> Result<Arc<Mutex<dyn Process>>, JuizError>;
}