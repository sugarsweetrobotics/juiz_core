

use crate::identifier::Identifier;
use crate::{value::*, JuizResult};

pub enum DestinationConnectionType {
    Pull,
    Push
}

pub trait DestinationConnection {

    fn identifier(&self) -> &Identifier;

    fn arg_name(&self) -> &String;

    fn connection_type(&self) -> &DestinationConnectionType;

    fn execute_destination(&self) -> JuizResult<Value>;

    fn push(&self, value: &Value) -> JuizResult<Value>;
}