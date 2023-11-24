
use crate::identifier::*;
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
}