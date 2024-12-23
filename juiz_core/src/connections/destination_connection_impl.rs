


use juiz_sdk::connections::{ConnectionManifest, ConnectionType};
use crate::prelude::*;


use core::fmt::Debug;
use std::clone::Clone;

use juiz_sdk::connections::{DestinationConnection, Connection, ConnectionCore};

#[derive(Clone)]
pub struct DestinationConnectionImpl{
    // core: ConnectionCore,
    connection_manifest: ConnectionManifest,
    destination_process: ProcessPtr
}

impl DestinationConnectionImpl {

    pub fn new_from_manifest(connection_manifest: ConnectionManifest,  destination_process: ProcessPtr) -> Self {
        DestinationConnectionImpl{
            connection_manifest,
            // core: ConnectionCore::new("DestinationConnection",  connection_manifest),
            destination_process}
    }

    // pub fn new(owner_identifier: &Identifier, destination_process_id: &Identifier, dest_process: ProcessPtr, connection_manifest: Value, arg_name: String) -> JuizResult<Self> {
    //     let manifest = check_connection_manifest(connection_manifest.clone())?;
    //     let destination_process_identifier = destination_process_id.clone();// juiz_lock(&dest_process).context("DestinationConnection::new()")?.identifier().clone();
    //     log::trace!("DestinationConnectionImpl::new(owner={:}, dest={:}, manifest={:}, arg_name={:}) called", owner_identifier, destination_process_id, manifest, arg_name);
    //     Ok(DestinationConnectionImpl{
    //         core: ConnectionCore::new("DestinationConnection", 
    //             owner_identifier.to_string(), 
    //             destination_process_identifier, 
    //             arg_name, 
    //             &manifest)?,
    //         destination_process: dest_process, })
    // }


    // fn owner_identifier(&self) -> &Identifier {
    //     self.core.source_identifier()
    // }

}

// impl JuizObjectCoreHolder for DestinationConnectionImpl {
//     fn core(&self) -> &ObjectCore {
//         self.core.object_core()
//     }
// }


// impl JuizObject for DestinationConnectionImpl {

//     fn profile_full(&self) -> JuizResult<Value> {
//         self.core.profile_full()
//     }

// }

impl Connection for DestinationConnectionImpl {

    fn identifier(&self) -> ConnectionIdentifier {
        self.connection_manifest.clone().into() //identifier.clone()
    }

    fn connection_type(&self) -> ConnectionType {
        self.connection_manifest.connection_type.clone()
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

