
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::prelude::*;
use juiz_sdk::anyhow::anyhow;
/// ContainerProcessを生成するためのFactoryクラスのtrait
/// 
pub trait ContainerProcessFactory : JuizObject + 'static {

    /// ContainerProcessを生成
    /// 
    fn create_container_process(&self, container: ContainerPtr, manifest: ProcessManifest) -> JuizResult<ProcessPtr>;

    fn destroy_container_process(&mut self, p: ProcessPtr) -> JuizResult<Value>;
}

// pub type ContainerProcessFactoryPtr = Arc<Mutex<dyn ContainerProcessFactory>>;



#[derive(Clone)]
pub struct ContainerProcessFactoryPtr {
    identifier: Identifier,
    type_name: String,
    ptr: Arc<RwLock<dyn ContainerProcessFactory>>,
}

impl ContainerProcessFactoryPtr {

    pub fn new(proc: impl ContainerProcessFactory) -> Self {
        let identifier = proc.identifier().clone();
        ContainerProcessFactoryPtr{
            identifier,
            type_name: proc.type_name().to_owned(),
            ptr: Arc::new(RwLock::new(proc))
        }
    }

    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }

    pub fn type_name(&self) -> &str {
        self.type_name.as_str()
    }
    
    pub fn lock(&self) -> JuizResult<RwLockReadGuard<dyn ContainerProcessFactory>> {
        self.ptr.read().or_else(|_|{ Err(anyhow!(JuizError::ObjectLockError{target:"ProcessPtr".to_owned()})) })
    }

    pub fn lock_mut(&self) -> JuizResult<RwLockWriteGuard<dyn ContainerProcessFactory>> {
        self.ptr.write().or_else(|_|{ Err(anyhow!(JuizError::ObjectLockError{target:"ProcessPtr".to_owned()})) })
    }

}