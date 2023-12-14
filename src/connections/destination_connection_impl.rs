


use anyhow::Context;
use serde_json::Value;

use crate::{Process, JuizError, Identifier, utils::{manifest_checker::check_connection_manifest, juiz_lock}, JuizResult, JuizObject, object::{JuizObjectCoreHolder, ObjectCore, JuizObjectClass}};
use std::sync::{Mutex, Arc};
use crate::value::*;
use core::fmt::Debug;
use std::clone::Clone;

use super::{DestinationConnection, connection::{Connection, ConnectionType}};


pub struct DestinationConnectionImpl{
    core: ObjectCore, 
    manifest: Value,
    connection_type: &'static ConnectionType,
    owner_identifier: Identifier,
    arg_name: String,
    destination_process_identifier: Identifier,
    destination_process: Arc<Mutex<dyn Process>>
}

impl DestinationConnectionImpl {

    pub fn new(owner_id: &Identifier, dest_process: Arc<Mutex<dyn Process>>, connection_manifest: Value, arg_name: String) -> JuizResult<Self> {
        log::trace!("# DestinationConnectionImpl::new() called");
        let manif = check_connection_manifest(connection_manifest.clone())?;
        let mut connection_type = &ConnectionType::Pull;
        let destination_process_identifier = juiz_lock(&dest_process)?.identifier().clone();
        match obj_get_str(&manif, "type") {
            Err(_) => {},
            Ok(typ_str) => {
                if typ_str == "pull" {}
                else if typ_str == "push" {
                    connection_type = &ConnectionType::Push;
                } else {
                    return Err(anyhow::Error::from(JuizError::ConnectionTypeError { manifest: connection_manifest }));
                }
            }
        };
        let connection_id = obj_get_str(&manif, "id")?;
        Ok(DestinationConnectionImpl{
            core: ObjectCore::create(JuizObjectClass::Connection("DestinationConnection"), "DestinationConnection", connection_id),
            owner_identifier:owner_id.clone(),
            destination_process: dest_process, 
            destination_process_identifier,
            manifest: manif,
            arg_name,
            connection_type})
    }

}

impl JuizObjectCoreHolder for DestinationConnectionImpl {
    fn core(&self) -> &crate::object::ObjectCore {
        &self.core
    }
}


impl JuizObject for DestinationConnectionImpl {

    
    fn profile_full(&self) -> JuizResult<Value> {
        Ok(jvalue!({
            "identifier": self.identifier(), 
            "connection_type": self.connection_type.to_string(),
            "arg_name": self.arg_name().to_owned(),
            "owner_identifier": self.owner_identifier.to_owned(),
            "destination_process_identifier": self.destination_process_identifier.to_owned(),
        }))
    }

}

impl Connection for DestinationConnectionImpl {
    fn arg_name(&self) -> &String {
        &self.arg_name
    }

    fn connection_type(&self) -> &ConnectionType {
        &self.connection_type
    }

}

impl DestinationConnection for DestinationConnectionImpl {

    fn execute_destination(&self) -> JuizResult<Value> {
        let proc = juiz_lock(&self.destination_process).context("DestinationConnectionImpl.execute_destination()")?;
        proc.execute()
    }

    fn push(&self, value: &Value) -> JuizResult<Value> {
        let proc = juiz_lock(&self.destination_process).context("DestinationConnectionImpl.push()")?;
        proc.push_by(self.arg_name(), value)
    }
}

impl<'a> Debug for DestinationConnectionImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SourceConnection").field("source_process", &self.destination_process.try_lock().unwrap().identifier()).field("owner_id", &self.owner_identifier).finish()
    }
}

impl Clone for DestinationConnectionImpl {
    fn clone(&self) -> Self {
        Self {
            core: self.core.clone(),
            owner_identifier: self.owner_identifier.clone(), 
            destination_process: self.destination_process.clone(), 
            destination_process_identifier: self.destination_process_identifier.clone(), 
            manifest: self.manifest.clone(), 
            arg_name: self.arg_name.clone(), 
            connection_type: self.connection_type }
    }
}
