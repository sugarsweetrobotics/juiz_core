
use crate::{Value, Identifier, JuizResult, JuizObject};

use super::connection::Connection;

pub enum DestinationConnectionType {
    Pull,
    Push
}

pub trait DestinationConnection : Connection {


    fn arg_name(&self) -> &String;

    fn connection_type(&self) -> &DestinationConnectionType;

    fn execute_destination(&self) -> JuizResult<Value>;

    fn push(&self, value: &Value) -> JuizResult<Value>;

}