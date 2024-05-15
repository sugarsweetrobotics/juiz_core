
use std::sync::{Arc, Mutex};

use crate::{processes::capsule::Capsule, JuizResult};

use super::connection::Connection;

pub trait DestinationConnection : Connection {

    fn execute_destination(&self) -> JuizResult<Arc<Mutex<Capsule>>>;

    fn push(&self, value: Arc<Mutex<Capsule>>) -> JuizResult<Arc<Mutex<Capsule>>>;

}