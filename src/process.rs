


use std::cell::RefCell;
use std::rc::Rc;

use crate::error::JuizError;
use crate::identifier::Identifier;
use crate::value::*;


pub type ProcessFunction=fn(serde_json::Value) -> Result<serde_json::Value, JuizError>;


pub trait Process {

    fn identifier(&self) -> Identifier;

    fn call(&self, _args: Value) -> Result<Value, JuizError>;

    fn is_updated(& self) -> Result<bool, JuizError>;

    fn invoke<'b>(&mut self) -> Result<Value, JuizError>;
}


pub trait Connectable<'a> {

    fn connected_from<'b>(&'b mut self, source: &'a Rc<RefCell<&'a mut dyn Process>>, connecting_arg: &String) -> ();

    fn is_connected_from(&self) -> bool;

    //fn connection_to(&mut self, target: &mut dyn Process, connect_arg_to: &String) -> &mut Self;
}