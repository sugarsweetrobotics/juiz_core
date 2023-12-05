


use serde_json::Value;

use crate::{Process, JuizError, identifier::Identifier};
use std::sync::{Mutex, Arc};
use crate::value::*;
use core::fmt::Debug;
use std::clone::Clone;
use crate::manifest_checker::*;
use crate::connection::destination_connection::DestinationConnectionType;

use super::DestinationConnection;

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
        log::trace!("# DestinationConnectionImpl::new() called");
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
