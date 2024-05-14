


use anyhow::Context;
use serde_json::Value;

use crate::{object::JuizObjectCoreHolder, processes::{capsule::Capsule, proc_lock}, utils::manifest_checker::check_connection_manifest, Identifier, JuizObject, JuizResult,  ProcessPtr};


use core::fmt::Debug;
use std::clone::Clone;

use super::{DestinationConnection, connection::{Connection, ConnectionCore}};


pub struct DestinationConnectionImpl{
    core: ConnectionCore,
    destination_process: ProcessPtr
}

impl DestinationConnectionImpl {

    pub fn new(owner_identifier: &Identifier, destination_process_id: &Identifier, dest_process: ProcessPtr, connection_manifest: Value, arg_name: String) -> JuizResult<Self> {
        log::trace!("# DestinationConnectionImpl::new() called");
        let manifest = check_connection_manifest(connection_manifest.clone())?;
        let destination_process_identifier = destination_process_id.clone();// juiz_lock(&dest_process).context("DestinationConnection::new()")?.identifier().clone();
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

    fn profile_full(&self) -> JuizResult<Capsule> {
        Ok(self.core.profile_full()?.into())
    }

}

impl Connection for DestinationConnectionImpl {
    fn connection_core(&self) -> &ConnectionCore {
        &self.core
    }
}

impl DestinationConnection for DestinationConnectionImpl {

    fn execute_destination(&self) -> JuizResult<Capsule> {
        let proc = proc_lock(&self.destination_process).context("DestinationConnectionImpl.execute_destination()")?;
        proc.execute()
    }

    fn push(&self, value: &Capsule) -> JuizResult<Capsule> {
        let proc = proc_lock(&self.destination_process).context("DestinationConnectionImpl.push()")?;
        proc.push_by(self.arg_name(), value)
    }
}

impl<'a> Debug for DestinationConnectionImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SourceConnection").field("source_process", &self.destination_process.read().unwrap().identifier()).field("owner_id", &self.owner_identifier()).finish()
    }
}

impl Clone for DestinationConnectionImpl {
    fn clone(&self) -> Self {
        Self { core: self.core.clone(), destination_process: self.destination_process.clone() }
    }
}
