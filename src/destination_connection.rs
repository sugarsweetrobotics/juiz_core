
use crate::error::*;
use crate::identifier::Identifier;
use crate::value::*;

pub enum DestinationConnectionType {
    Pull,
    Push
}

pub trait DestinationConnection {

    fn identifier(&self) -> &Identifier;

    fn arg_name(&self) -> &String;

    fn connection_type(&self) -> &DestinationConnectionType;

    fn execute_destination(&self) -> Result<Value, JuizError>;

    fn push(&self, value: &Value) -> Result<Value, JuizError>;
}