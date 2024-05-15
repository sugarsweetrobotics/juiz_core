
use std::sync::{Arc, Mutex};

use crate::{processes::capsule::Capsule, JuizResult};

use super::connection::Connection;


pub trait SourceConnection : Connection {

    fn is_source_updated(&self) -> JuizResult<bool>;

    fn invoke_source(&mut self) -> JuizResult<Arc<Mutex<Capsule>>>;

    // fn source_process_id(&self) -> &Identifier;

    fn pull(&self) -> JuizResult<Arc<Mutex<Capsule>>>;
}

