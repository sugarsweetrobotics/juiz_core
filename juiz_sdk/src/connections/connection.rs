
use crate::prelude::*;
use crate::object::{JuizObject, JuizObjectClass, ObjectCore};
use super::connection_type::ConnectionType;
use super::connection_core::ConnectionCore;



pub trait Connection : JuizObject {

    fn connection_core(&self) -> &ConnectionCore;

    fn arg_name(&self) -> &String {
        self.connection_core().arg_name()
    }

    fn connection_type(&self) -> ConnectionType {
        self.connection_core().connection_type()
    }
}
