


use serde_json::Value;

use crate::{process::*, error::JuizError, identifier::Identifier};
use crate::destination_connection::*;
use std::sync::{Mutex, Arc};
use std::{collections::HashMap, cell::RefCell, rc::Rc};
use crate::value::*;
use core::fmt::Debug;
use std::clone::Clone;
use crate::manifest_checker::*;

pub struct DestinationConnectionImpl{
    connection_id: Identifier,
    manifest: Value,
    connection_type: &'static DestinationConnectionType,
    owner_id: Identifier,
    arg_name: String,
    destination_process: Arc<Mutex<dyn Process>>
}

impl DestinationConnectionImpl {

    pub fn new(owner_id: &Identifier, dest_process: Arc<Mutex<dyn Process>>, connection_manifest: Value, arg_name: String) -> Result<Self, JuizError> {
        let manif = check_connection_manifest(connection_manifest)?;
        let mut connection_type = &DestinationConnectionType::Pull;
        match obj_get_str(&manif, "type") {
            Err(_) => {},
            Ok(typ_str) => {
                if typ_str == "pull" {}
                else if typ_str == "push" {
                    connection_type = &DestinationConnectionType::Push;
                } else {
                    return Err(JuizError::DestinationConnectionNewReceivedInvalidManifestTypeError{});
                }
            }
        }

        Ok(DestinationConnectionImpl{
            connection_id: obj_get_str(&manif, "id")?.to_string(),
            owner_id:owner_id.clone(),
            destination_process: dest_process, 
            manifest: manif,
            arg_name,
            connection_type})   
    }

}

impl DestinationConnection for DestinationConnectionImpl {


    fn identifier(&self) -> &Identifier {
        &self.connection_id
    }

    fn arg_name(&self) -> &String {
        &self.arg_name
    }

    fn connection_type(&self) -> &DestinationConnectionType {
        &self.connection_type
    }

    fn execute_destination(&self) -> Result<Value, JuizError> {
        match self.destination_process.try_lock() {
            Err(_err) => return Err(JuizError::DestinationConnectionCanNotBorrowMutableProcessReferenceError{}),
            Ok(proc) => {
                match proc.execute() {
                    Err(e) => return Err(e),
                    Ok(value) => Ok(value)
                }
            }
        }
    }

    fn push(&self, value: &Value) -> Result<Value, JuizError> {
        match self.destination_process.try_lock() {
            Err(_err) => return Err(JuizError::DestinationConnectionCanNotBorrowMutableProcessReferenceError{}),
            Ok(proc) => {
                match proc.push_by(self.arg_name(), value) {
                    Err(e) => return Err(e),
                    Ok(value) => Ok(value)
                }
            }
        }
    }
}

impl<'a> Debug for DestinationConnectionImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SourceConnection").field("source_process", &self.destination_process.try_lock().unwrap().identifier()).field("owner_id", &self.owner_id).finish()
    }
}

impl Clone for DestinationConnectionImpl {
    fn clone(&self) -> Self {
        Self {connection_id: self.identifier().clone(), owner_id: self.owner_id.clone(), destination_process: self.destination_process.clone(), manifest: self.manifest.clone(), arg_name: self.arg_name.clone(), connection_type: self.connection_type }
    }
}


pub struct DestinationConnectionRack {
    connection_map: HashMap<String, Rc<RefCell<dyn DestinationConnection>>>
}

impl<'a> DestinationConnectionRack {

    pub fn new() -> Self {
        DestinationConnectionRack{connection_map: HashMap::new()}
    }

    pub fn append<SC : DestinationConnection + std::clone::Clone + 'static>(&mut self, arg_name: &String,  connection: SC) -> Result<(), JuizError> {
        self.connection_map.insert(arg_name.clone(), Rc::new(RefCell::new(connection.clone())));
        Ok(())
    }

    pub fn remove_connection(&mut self, arg_name: &String) -> &mut Self {
        self.connection_map.remove(arg_name);
        self
    }

    pub fn connection_mut(&mut self, arg_name: &String) -> Option<&mut Rc<RefCell<dyn DestinationConnection>>> {
        self.connection_map.get_mut(arg_name)
    }

    pub fn connection(&self, arg_name: &String) -> Option<&Rc<RefCell<dyn DestinationConnection>>>  {
        self.connection_map.get(arg_name)
    }

    pub fn push(&self, output: Value) -> Result<Value, JuizError> {
        for (_k, v) in self.connection_map.iter() {
            match v.borrow().push(&output) {
                Err(e) => return Err(e),
                Ok(_) => {}
            }
        }
        return Ok(output)
    }
}