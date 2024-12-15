



use crate::prelude::*;

use core::fmt::Debug;
use std::clone::Clone;

use juiz_sdk::connections::{Connection, ConnectionCore, ConnectionManifest};

pub struct SourceConnectionImpl {
    core: ConnectionCore,
    source_process: ProcessPtr,
}

impl SourceConnectionImpl {

    pub fn new_from_manifest(connection_manifest: ConnectionManifest, source_process: ProcessPtr) -> Self {
        log::trace!("SourceConnectionImpl::new_from_manifest({connection_manifest}) called");
        SourceConnectionImpl{
            core: ConnectionCore::new("SourceConnection", connection_manifest),
            source_process}
    }
    
    // pub fn new(owner_identifier: Identifier, source_process: ProcessPtr, manifest: Value, arg_name: String) -> JuizResult<Self> {
    //     let source_process_identifier = source_process.identifier().clone();
    //     log::trace!("SourceConnectionImpl::new(owner={:}, src={:}, manifest={:}, arg_name={:}) called", owner_identifier, source_process_identifier, manifest, arg_name);
    //     Ok(SourceConnectionImpl{
    //         core: ConnectionCore::new("SourceConnection", 
    //             source_process_identifier, 
    //             owner_identifier, 
    //             arg_name, 
    //             &manifest)?,
    //         source_process})
    // }

    fn owner_identifier(&self) -> &Identifier {
        self.core.destination_identifier()
    }
}

impl JuizObjectCoreHolder for SourceConnectionImpl {
    fn core(&self) -> &ObjectCore {
        &self.core.object_core()
    }
}

impl JuizObject for SourceConnectionImpl {

    fn profile_full(&self) -> JuizResult<Value> {
        self.core.profile_full()
    }
}

impl Connection for SourceConnectionImpl {

    fn connection_core(&self) -> &ConnectionCore {
        &self.core
    }
}

impl SourceConnection for SourceConnectionImpl {

    fn is_source_updated(&self) -> JuizResult<bool> {
        self.source_process.lock()?.is_updated()
    }

    fn invoke_source(&mut self) -> JuizResult<CapsulePtr> {
        self.source_process.lock()?.invoke()
    }
 
    fn pull(&self) -> JuizResult<CapsulePtr> {
        log::trace!("SourceConnectionImpl({}).pull() called", self.identifier());
        self.source_process.lock()?.invoke()
    }
}

impl<'a> Debug for SourceConnectionImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SourceConnection").field("source_process", self.source_process.identifier()).field("owner_id", &self.owner_identifier()).finish()
    }
}

impl Clone for SourceConnectionImpl {
    fn clone(&self) -> Self {
        Self { 
            core: self.core.clone(), source_process: self.source_process.clone() }
    }
}

impl Drop for SourceConnectionImpl {
    fn drop(&mut self) {
        // self.source_process.borrow_mut().disconnect_to(self.owner_id);
    }
}//
