
use crate::{Value, Identifier, JuizResult};

use super::connection::Connection;

pub enum SourceConnectionType {
    Pull,
    Push
}

pub trait SourceConnection : Connection {

    fn arg_name(&self) -> &String;

    fn connection_type(&self) -> &SourceConnectionType;

    fn is_source_updated(&self) -> JuizResult<bool>;

    fn invoke_source(&mut self) -> JuizResult<Value>;

    // fn source_process_id(&self) -> &Identifier;

    fn pull(&self) -> JuizResult<Value>;
}

