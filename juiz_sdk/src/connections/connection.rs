
use crate::connection_identifier::ConnectionIdentifier;
use crate::object::JuizObject;
use super::connection_type::ConnectionType;
use super::connection_core::ConnectionCore;



pub trait Connection {

    fn identifier(&self) -> ConnectionIdentifier;

    fn connection_type(&self) -> ConnectionType;
}
