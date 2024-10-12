use std::{fmt::Display, sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard}};

use mopa::mopafy;
use crate::prelude::*;

pub trait Container : Display + mopa::Any + JuizObject{
    
    fn manifest(&self) -> &Value;

    fn process(&self, name_or_id: &String) -> Option<ProcessPtr>;

    fn processes(&self) -> Vec<ProcessPtr>;

    fn register_process(&mut self, p: ProcessPtr) -> JuizResult<ProcessPtr>;

    fn purge_process(&mut self, name_or_id: &String) -> JuizResult<()>;

    fn clear(&mut self) -> JuizResult<()>;
}

mopafy!(Container);

pub type ContainerPtr = Arc<RwLock<dyn Container>>;

pub fn container_lock<'a>(obj: &'a ContainerPtr) -> JuizResult<RwLockReadGuard<'a, dyn Container>> {
    match obj.read() {
        Err(e) => {
            log::error!("juiz_lock() failed. Error is {:?}", e);
            Err(anyhow::Error::from(JuizError::MutexLockFailedError{error: e.to_string()}))
        },
        Ok(v) => Ok(v)
    }
}

pub fn container_lock_mut<'b, T: Container + ?Sized>(obj: &'b Arc<RwLock<T>>) -> JuizResult<RwLockWriteGuard<'b, T>>{
    match obj.write() {
        Err(e) => {
            log::error!("juiz_lock() failed. Error is {:?}", e);
            Err(anyhow::Error::from(JuizError::MutexLockFailedError{error: e.to_string()}))
        },
        Ok(v) => Ok(v)
    }
}
