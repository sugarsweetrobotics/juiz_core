


use crate::{process::*, error::JuizError, identifier::Identifier, source_connection::*};
use std::sync::{Mutex, Arc};
use crate::value::*;
use core::fmt::Debug;
use std::clone::Clone;


pub struct SourceConnectionImpl {
    connection_id: Identifier,
    arg_name: String,
    connection_type: &'static SourceConnectionType,
    manifest: Value,
    owner_id: Identifier,
    source_id: Identifier,
    source_process: Arc<Mutex<dyn Process>>
}

impl SourceConnectionImpl {

    pub fn new(owner_id: Identifier, source_process: Arc<Mutex<dyn Process>>, manifest: Value, arg_name: String) -> Result<Self, JuizError> {
        let connection_id = obj_get_str(&manifest, "id")?.to_string();
        let source_id = source_process.lock().unwrap().identifier().clone();
        let mut connection_type = &SourceConnectionType::Pull;
        match obj_get_str(&manifest, "type") {
            Err(_) => {},
            Ok(typ_str) => {
                if typ_str == "pull" {}
                else if typ_str == "push" {
                    connection_type = &SourceConnectionType::Push;
                } else {
                    return Err(JuizError::SourceConnectionNewReceivedInvalidManifestTypeError{});
                }
            }
        }
        Ok(SourceConnectionImpl{
            connection_id,
            owner_id, 
            source_id, 
            source_process, manifest,
            arg_name,
            connection_type})
    }

}

impl SourceConnection for SourceConnectionImpl {


    fn identifier(&self) -> &Identifier {
        &self.connection_id
    }

    fn arg_name(&self) -> &String {
        &self.arg_name
    }

    fn connection_type(&self) -> &SourceConnectionType {
        &self.connection_type
    }

    fn is_source_updated(&self) -> Result<bool, JuizError> {
        match self.source_process.try_lock() {
            Err(_err) => return Err(JuizError::SourceConnectionCanNotBorrowMutableProcessReferenceError{}),
            Ok(proc) => {
                match proc.is_updated() {
                    Err(e) => return Err(e),
                    Ok(value) => Ok(value)
                }
            }
        }
    }

    fn invoke_source(&mut self) -> Result<Value, JuizError> {
        match self.source_process.try_lock() {
            Err(_err) => return Err(JuizError::SourceConnectionCanNotBorrowMutableProcessReferenceError{}),
            Ok(proc) => {
                match proc.invoke() {
                    Err(e) => return Err(e),
                    Ok(value) => Ok(value)
                }
            }
        }
    }

    fn source_process_id(&self) -> &Identifier {
        &self.source_id
    }

 
    fn pull(&self) -> Result<Value, JuizError> {
        match self.source_process.try_lock() {
        Err(_err) => return Err(JuizError::SourceConnectionCanNotBorrowMutableProcessReferenceError{}),
        Ok(proc) => {
            match proc.invoke() {
                Err(e) => return Err(e),
                Ok(value) => Ok(value)
            }
        }
    }
    }
}

impl<'a> Debug for SourceConnectionImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SourceConnection").field("source_process", &self.source_process.try_lock().unwrap().identifier()).field("owner_id", &self.owner_id).finish()
    }
}

impl Clone for SourceConnectionImpl {
    fn clone(&self) -> Self {
        Self { connection_id: self.connection_id.clone(),
            owner_id: self.owner_id.clone(), source_id: self.source_id.clone(), source_process: self.source_process.clone(), manifest: self.manifest.clone(), arg_name: self.arg_name.clone(), connection_type: self.connection_type }
    }
}

impl Drop for SourceConnectionImpl {
    fn drop(&mut self) {
        // self.source_process.borrow_mut().disconnect_to(self.owner_id);
    }
}//
