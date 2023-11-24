


use crate::{process::*, error::JuizError, identifier::Identifier};
use serde_json::map::Map;
use std::{collections::HashMap, cell::RefCell, rc::Rc};
use crate::value::*;
use core::fmt::Debug;


pub struct SourceConnection<'a> {
    owner_id: Identifier,
    source_process: &'a Rc<RefCell<&'a mut dyn Process>>
}

impl<'a> SourceConnection<'a> {

    pub fn new(owner_id: &Identifier, source_process: &'a Rc<RefCell<&'a mut dyn Process>>) -> Self {
        SourceConnection{owner_id:owner_id.clone(), source_process: source_process}
    }

    pub fn is_source_updated(&self) -> bool {
        return true;
    }

    pub fn invoke_source(&mut self) -> Result<Value, JuizError> {
        match self.source_process.try_borrow_mut() {
            Err(err) => return Err(JuizError::SourceConnectionCanNotBorrowMutableProcessReferenceError{}),
            Ok(mut proc) => {
                match proc.invoke() {
                    Err(e) => return Err(e),
                    Ok(value) => Ok(value)
                }
            }
        }
    }

    
}

impl<'a> Debug for SourceConnection<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SourceConnection").field("source_process", &self.source_process.borrow().identifier()).field("owner_id", &self.owner_id).finish()
    }
}


#[derive(Debug)]
pub struct SourceConnectionRack<'a> {
    connection_map: HashMap<String, SourceConnection<'a>>
}

impl<'a> SourceConnectionRack<'a> {

    pub fn new() -> Self {
        SourceConnectionRack{connection_map: HashMap::new()}
    }

    pub fn append(&mut self, arg_name: &String, connection: SourceConnection<'a>) -> &mut Self {
        self.connection_map.insert(arg_name.clone(), connection);
        self
    }

    pub fn remove_connection(&mut self, arg_name: &String) -> &mut Self {
        self.connection_map.remove(arg_name);
        self
    }

    pub fn connection_mut(&'a mut self, arg_name: &String) -> Option<&'a mut SourceConnection> {
        self.connection_map.get_mut(arg_name)
    }

    pub fn connection(&'a self, arg_name: &String) -> Option<&'a SourceConnection> {
        self.connection_map.get(arg_name)
    }

    pub fn collect_values(&mut self) -> Result<Value, JuizError> {
        let mut args = Map::new();
        /// for_eachを使った実装に直したい
        for (k, v) in self.connection_map.iter_mut() {
            match v.invoke_source() {
                Err(e) => return Err(e),
                Ok(value) => {
                    args.insert(k.clone(), value);
                }
            }
        }
        Ok(jvalue!(args))
    }

    pub fn is_updated(&self) -> bool {
        (&self.connection_map).into_iter().any(| (_k, v) | v.is_source_updated())
    }
}