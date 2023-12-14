


use anyhow::Context;

use crate::{jvalue, Value, Process, JuizError, JuizResult, Identifier, utils::juiz_lock, connections::source_connection::SourceConnectionType, value::obj_get_str, JuizObject, object::{JuizObjectCoreHolder, ObjectCore, JuizObjectClass}};
use std::sync::{Mutex, Arc};
use core::fmt::Debug;
use std::clone::Clone;

use super::{SourceConnection, connection::Connection};



pub fn source_connection_type_str(typ: &'static SourceConnectionType) -> String {
    match typ {
        SourceConnectionType::Pull => "Pull".to_string(),
        _ => "Push".to_string()
    }
}
pub struct SourceConnectionImpl {
    core: ObjectCore, 
    arg_name: String,
    connection_type: &'static SourceConnectionType,
    manifest: Value,
    owner_identifier: Identifier,
    // source_id: Identifier,
    source_process: Arc<Mutex<dyn Process>>,
    source_process_identifier: Identifier,
}

impl SourceConnectionImpl {

    pub fn new(owner_identifier: Identifier, source_process: Arc<Mutex<dyn Process>>, manifest: Value, arg_name: String) -> JuizResult<Self> {
        log::trace!("# SourceConnectionImpl::new() called");
        let connection_id = obj_get_str(&manifest, "id")?.to_string();
        let source_process_identifier = juiz_lock(&source_process)?.identifier().clone();
        let mut connection_type = &SourceConnectionType::Pull;
        match obj_get_str(&manifest, "type") {
            Err(_) => {},
            Ok(typ_str) => {
                if typ_str == "pull" {}
                else if typ_str == "push" {
                    connection_type = &SourceConnectionType::Push;
                } else {
                    return Err(anyhow::Error::from(JuizError::ConnectionTypeError{manifest}));
                }
            }
        };
        Ok(SourceConnectionImpl{
            core: ObjectCore::create(JuizObjectClass::Connection("SourceConnection"), "SourceConnection", connection_id),
            owner_identifier, 
            source_process_identifier,
            // source_id, 
            source_process, manifest,
            arg_name,
            connection_type})
    }

}

impl JuizObjectCoreHolder for SourceConnectionImpl {
    fn core(&self) -> &crate::object::ObjectCore {
        &self.core
    }
}

impl JuizObject for SourceConnectionImpl {

    fn profile_full(&self) -> JuizResult<Value> {
        Ok(jvalue!({
            "identifier": self.identifier(),
            "connection_type": source_connection_type_str(self.connection_type),
            "arg_name": self.arg_name().to_owned(),
            "owner_identifier": self.owner_identifier.to_owned(),
            "source_process_identifier": self.source_process_identifier.to_owned(),
        }))
    }
}

impl Connection for SourceConnectionImpl {

}

impl SourceConnection for SourceConnectionImpl {

    fn arg_name(&self) -> &String {
        &self.arg_name
    }

    fn connection_type(&self) -> &SourceConnectionType {
        &self.connection_type
    }

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
        f.debug_struct("SourceConnection").field("source_process", &self.source_process.try_lock().unwrap().identifier()).field("owner_id", &self.owner_identifier).finish()
    }
}

impl Clone for SourceConnectionImpl {
    fn clone(&self) -> Self {
        Self { 
            core: self.core.clone(),
            owner_identifier: self.owner_identifier.clone(), source_process_identifier: self.source_process_identifier.clone(), source_process: self.source_process.clone(), manifest: self.manifest.clone(), arg_name: self.arg_name.clone(), connection_type: self.connection_type }
    }
}

impl Drop for SourceConnectionImpl {
    fn drop(&mut self) {
        // self.source_process.borrow_mut().disconnect_to(self.owner_id);
    }
}//
