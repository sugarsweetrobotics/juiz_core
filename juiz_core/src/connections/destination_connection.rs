


use crate::prelude::*;
use super::connection::Connection;

pub trait DestinationConnection : Connection {

    fn execute_destination(&self) -> JuizResult<CapsulePtr>;

    fn push(&self, value: CapsulePtr) -> JuizResult<CapsulePtr>;

}