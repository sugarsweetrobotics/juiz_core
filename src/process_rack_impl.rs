
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::error::JuizError;
use crate::identifier::*;
use crate::process_rack::ProcessRack;
use crate::process::*;

pub struct ProcessRackImpl {
    processes: HashMap<String, Arc<Mutex<dyn Process>>>
}

impl ProcessRackImpl {
    
    pub fn new() -> Self {
        ProcessRackImpl{processes: HashMap::new()}
    }

    pub fn push(&mut self, proc: Arc<Mutex<dyn Process>>) -> Result<(), JuizError> {
        let id = match proc.try_lock() {
            Err(_e) => Err(JuizError::ProcessRackCanNotBorrowInsertedProcessError{}),
            Ok(p) => Ok(p.identifier().clone())
        }?;
        self.processes.insert(id, Arc::clone(&proc));
        Ok(())
    }
}

impl ProcessRack for ProcessRackImpl {

    fn process(&mut self, id: &Identifier) -> Option<&Arc<Mutex<dyn Process>>> {
        for (k, p) in self.processes.iter_mut() {
            if k == id {
                return Some(p);
            }
        }
        None
    }

}