

/// 
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use anyhow::anyhow;
use crate::prelude::*;

#[derive(Clone)]
pub struct ProcessPtr {
    identifier: Identifier,
    ptr: Arc<RwLock<dyn Process>>,
}

impl ProcessPtr {

    pub fn new(proc: impl Process) -> Self {
        let identifier = proc.identifier().clone();
        ProcessPtr{
            identifier,
            ptr: Arc::new(RwLock::new(proc))
        }
    }

    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }
    
    pub fn lock(&self) -> JuizResult<RwLockReadGuard<dyn Process>> {
        self.ptr.read().or_else(|_|{ Err(anyhow!(JuizError::ObjectLockError{target:"ProcessPtr".to_owned()})) })
    }

    pub fn lock_mut(&self) -> JuizResult<RwLockWriteGuard<dyn Process>> {
        self.ptr.write().or_else(|_|{ Err(anyhow!(JuizError::ObjectLockError{target:"ProcessPtr".to_owned()})) })
    }
}
