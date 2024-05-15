


use anyhow::Context;

use crate::{object::{JuizObjectCoreHolder, ObjectCore}, processes::proc_lock, CapsulePtr, Identifier, JuizObject, JuizResult, ProcessPtr, Value};

use core::fmt::Debug;
use std::clone::Clone;

use super::{SourceConnection, connection::{Connection, ConnectionCore}};

pub struct SourceConnectionImpl {
    core: ConnectionCore,
    source_process: ProcessPtr,
}

impl SourceConnectionImpl {
    pub fn new(owner_identifier: Identifier, source_process: ProcessPtr, manifest: Value, arg_name: String) -> JuizResult<Self> {
        log::trace!("SourceConnectionImpl::new() called");
        let source_process_identifier = proc_lock(&source_process)?.identifier().clone();
        Ok(SourceConnectionImpl{
            core: ConnectionCore::new("SourceConnection", 
                source_process_identifier, 
                owner_identifier, 
                arg_name, 
                &manifest)?,
            source_process})
    }

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
        let proc = proc_lock(&self.source_process).context("in SourceConnectionImpl.is_source_updated()")?;
        proc.is_updated()
    }

    fn invoke_source(&mut self) -> JuizResult<CapsulePtr> {
        let proc = proc_lock(&self.source_process).context("in SourceConnectionImpl.invoke_source()")?;
        proc.invoke()
    }
 
    fn pull(&self) -> JuizResult<CapsulePtr> {
        log::trace!("SourceConnectionImpl({:?}).pull() called", self.identifier());
        proc_lock(&self.source_process).context("SourceConnectionImpl.pull()")?.invoke()
    }
}

impl<'a> Debug for SourceConnectionImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let p = proc_lock(&self.source_process);
        f.debug_struct("SourceConnection").field("source_process", p.unwrap().identifier()).field("owner_id", &self.owner_identifier()).finish()
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
