


use serde_json::Value;

use crate::connections::connection::ConnectionType;
use crate::prelude::*;
use crate::{object::JuizObjectCoreHolder, utils::manifest_checker::check_connection_manifest};


use core::fmt::Debug;
use std::clone::Clone;

use super::{DestinationConnection, connection::{Connection, ConnectionCore}};


pub struct DestinationConnectionImpl{
    core: ConnectionCore,
    destination_process: ProcessPtr
}

impl DestinationConnectionImpl {

    pub fn new(owner_identifier: &Identifier, destination_process_id: &Identifier, dest_process: ProcessPtr, connection_manifest: Value, arg_name: String) -> JuizResult<Self> {
        let manifest = check_connection_manifest(connection_manifest.clone())?;
        let destination_process_identifier = destination_process_id.clone();// juiz_lock(&dest_process).context("DestinationConnection::new()")?.identifier().clone();
        log::trace!("DestinationConnectionImpl::new(owner={:}, dest={:}, manifest={:}, arg_name={:}) called", owner_identifier, destination_process_id, manifest, arg_name);
        Ok(DestinationConnectionImpl{
            core: ConnectionCore::new("DestinationConnection", 
                owner_identifier.to_string(), 
                destination_process_identifier, 
                arg_name, 
                &manifest)?,
            destination_process: dest_process, })
    }


    fn owner_identifier(&self) -> &Identifier {
        self.core.source_identifier()
    }

}

impl JuizObjectCoreHolder for DestinationConnectionImpl {
    fn core(&self) -> &crate::object::ObjectCore {
        self.core.object_core()
    }
}


impl JuizObject for DestinationConnectionImpl {

    fn profile_full(&self) -> JuizResult<Value> {
        self.core.profile_full()
    }

}

impl Connection for DestinationConnectionImpl {
    fn connection_core(&self) -> &ConnectionCore {
        &self.core
    }
}

impl DestinationConnection for DestinationConnectionImpl {

    fn execute_destination(&self) -> JuizResult<CapsulePtr> {
        log::trace!("DestinationConnectionImpl::execute_destination() called");
        self.destination_process.lock()?.execute()
    }

    fn push(&self, value: CapsulePtr) -> JuizResult<CapsulePtr> {
        log::trace!("DestinationConnectionImpl::push() called");
        let proc = self.destination_process.lock()?;
        if self.connection_type() == ConnectionType::Push {
            proc.push_by(self.arg_name(), value)
        } else {
            Ok(value)
        }
    }
}

impl<'a> Debug for DestinationConnectionImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SourceConnection").field("source_process", &self.destination_process.identifier()).field("owner_id", &self.owner_identifier()).finish()
    }
}

impl Clone for DestinationConnectionImpl {
    fn clone(&self) -> Self {
        Self { core: self.core.clone(), destination_process: self.destination_process.clone() }
    }
}
