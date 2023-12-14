
use crate::{Value, Identifier, JuizResult};

use super::connection::Connection;


pub trait SourceConnection : Connection {

    fn is_source_updated(&self) -> JuizResult<bool>;

    fn invoke_source(&mut self) -> JuizResult<Value>;

    // fn source_process_id(&self) -> &Identifier;

    fn pull(&self) -> JuizResult<Value>;
}

