


use anyhow::Context;

use crate::{Value, Process, JuizResult, Identifier, utils::juiz_lock, JuizObject, object::{JuizObjectCoreHolder, ObjectCore}};
use std::sync::{Mutex, Arc};
use core::fmt::Debug;
use std::clone::Clone;

use super::{SourceConnection, connection::{Connection, ConnectionCore}};

pub struct SourceConnectionImpl {
    core: ConnectionCore,
    source_process: Arc<Mutex<dyn Process>>,
}

impl SourceConnectionImpl {

    pub fn new(owner_identifier: Identifier, source_process: Arc<Mutex<dyn Process>>, manifest: Value, arg_name: String) -> JuizResult<Self> {
        log::trace!("# SourceConnectionImpl::new() called");
        let source_process_identifier = juiz_lock(&source_process)?.identifier().clone();
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
        let proc = juiz_lock(&self.source_process).context("in SourceConnectionImpl.is_source_updated()")?;
        proc.is_updated()
    }

    fn invoke_source(&mut self) -> JuizResult<Value> {
        let proc = juiz_lock(&self.source_process).context("in SourceConnectionImpl.invoke_source()")?;
        proc.invoke()
    }
 
    fn pull(&self) -> JuizResult<Value> {
        log::trace!("SourceConnectionImpl({:?}).pull() called", self.identifier());
        juiz_lock(&self.source_process).context("SourceConnectionImpl.pull()")?.invoke()
    }
}

impl<'a> Debug for SourceConnectionImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SourceConnection").field("source_process", &self.source_process.try_lock().unwrap().identifier()).field("owner_id", &self.owner_identifier()).finish()
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
