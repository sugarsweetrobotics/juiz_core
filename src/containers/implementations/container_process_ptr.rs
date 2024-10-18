

use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use anyhow::anyhow;

use crate::prelude::*;
use super::container_process_impl::ContainerProcessImpl;


pub struct ContainerProcessPtr {
    ptr: Arc<RwLock<ContainerProcessImpl>>,
}

impl ContainerProcessPtr { 
    pub fn new(proc: ContainerProcessImpl) -> Self {
        Self{
            ptr: Arc::new(RwLock::new(proc))
        }
    }
    
    pub fn lock(&self) -> JuizResult<RwLockReadGuard<ContainerProcessImpl>> {
        self.ptr.read().or_else(|_|{ Err(anyhow!(JuizError::ObjectLockError{target:"ProcessPtr".to_owned()})) })
    }

    pub fn lock_mut(&self) -> JuizResult<RwLockWriteGuard<ContainerProcessImpl>> {
        self.ptr.write().or_else(|_|{ Err(anyhow!(JuizError::ObjectLockError{target:"ProcessPtr".to_owned()})) })
    }
}