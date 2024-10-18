use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::prelude::*;
use anyhow::anyhow;




#[derive(Clone)]
pub struct ContainerPtr {
    identifier: Identifier,
    ptr: Arc<RwLock<dyn Container>>,
}

impl ContainerPtr {

    pub fn new(container: impl Container) -> Self {
        let identifier = container.identifier().clone();
        ContainerPtr{
            identifier,
            ptr: Arc::new(RwLock::new(container))
        }
    }

    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }
    
    pub fn lock(&self) -> JuizResult<RwLockReadGuard<dyn Container>> {
        self.ptr.read().or_else(|_|{ Err(anyhow!(JuizError::ObjectLockError{target:"ContainerPtr".to_owned()})) })
    }

    pub fn lock_mut(&self) -> JuizResult<RwLockWriteGuard<dyn Container>> {
        self.ptr.write().or_else(|_|{ Err(anyhow!(JuizError::ObjectLockError{target:"ContainerPtr".to_owned()})) })
    }
}
