use std::sync::{Arc, Mutex, MutexGuard};

use crate::{JuizError, JuizResult};






pub fn juiz_lock<'b, T: ?Sized>(obj: &'b Arc<Mutex<T>>) -> JuizResult<MutexGuard<'b, T>> {
    // log::trace!("juiz_lock() called");
    match obj.try_lock() {
        Err(e) => {
            log::error!("juiz_lock() failed. Error is {:?}", e);
            Err(anyhow::Error::from(JuizError::MutexLockFailedError{}))
        },
        Ok(v) => Ok(v)
    }
}