
use crate::{processes::capsule::Capsule, JuizResult};

use super::connection::Connection;

pub trait DestinationConnection : Connection {

    fn execute_destination(&self) -> JuizResult<Capsule>;

    fn push(&self, value: &Capsule) -> JuizResult<Capsule>;

}