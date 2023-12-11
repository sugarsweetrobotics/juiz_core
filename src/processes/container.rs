use std::fmt::Display;

use mopa::mopafy;

use crate::{Value, Identifier, Process, JuizResult};

pub trait Container : Display + mopa::Any {
    
    fn identifier(&self) -> &Identifier;

    fn manifest(&self) -> &Value;

    fn profile_full(&self) -> JuizResult<Value>;
}

mopafy!(Container);

pub trait ContainerProcess: Process {
    
}