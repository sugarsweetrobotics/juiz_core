
use crate::{Value, JuizResult, processes::Output};

use super::connection::Connection;

pub trait DestinationConnection : Connection {

    fn execute_destination(&self) -> JuizResult<Output>;

    fn push(&self, value: &Output) -> JuizResult<Output>;

}