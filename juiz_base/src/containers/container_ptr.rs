use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::prelude::*;
use anyhow::anyhow;



#[derive(Clone)]
pub struct ContainerPtr {
    identifier: Identifier,
    type_name: String,
    ptr: Arc<RwLock<dyn Container>>,
}

impl ContainerPtr {

    pub fn new(container: impl Container) -> Self {
        ContainerPtr{
            identifier: container.identifier().clone(),
            type_name: container.type_name().to_owned(),
            ptr: Arc::new(RwLock::new(container))
        }
    }

    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }

    pub fn type_name(&self) -> &String {
        &self.type_name
    }
    
    pub fn lock(&self) -> JuizResult<RwLockReadGuard<dyn Container>> {
        self.ptr.read().or_else(|_|{ Err(anyhow!(JuizError::ObjectLockError{target:"ContainerPtr".to_owned()})) })
    }

    pub fn lock_mut(&self) -> JuizResult<RwLockWriteGuard<dyn Container>> {
        self.ptr.write().or_else(|_|{ Err(anyhow!(JuizError::ObjectLockError{target:"ContainerPtr".to_owned()})) })
    }

    
    // pub fn downcast_and_then<T: 'static, R, F>(&self, func: F) -> JuizResult<R> where F: FnOnce(&ContainerImpl<T>)->R {
    //     match self.lock()?.downcast_ref::<ContainerImpl<T>>() {
    //         None => Err(anyhow::Error::from(JuizError::ContainerDowncastingError{identifier: self.identifier.clone()})),
    //         Some(container_impl) => { 
    //             Ok(func(container_impl))
    //         }
    //     }
    // }

    // pub fn downcast_mut_and_then<T: 'static, R, F>(&self, func: F) -> JuizResult<R> where F: FnOnce(&mut ContainerImpl<T>)->R {
    //     match self.lock_mut()?.downcast_mut::<ContainerImpl<T>>() {
    //         None => Err(anyhow::Error::from(JuizError::ContainerDowncastingError{identifier: self.identifier.clone()})),
    //         Some(container_impl) => { 
    //             Ok(func(container_impl))
    //         }
    //     }
    // }
}
