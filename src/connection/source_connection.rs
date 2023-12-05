
use crate::{Value, JuizError, Identifier};

pub enum SourceConnectionType {
    Pull,
    Push
}

pub trait SourceConnection {

    fn identifier(&self) -> &Identifier;

    fn arg_name(&self) -> &String;

    fn connection_type(&self) -> &SourceConnectionType;

    fn is_source_updated(&self) -> Result<bool, JuizError>;

    fn invoke_source(&mut self) -> Result<Value, JuizError>;

    // fn source_process_id(&self) -> &Identifier;

    fn pull(&self) -> Result<Value, JuizError>;
}

