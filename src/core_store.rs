use std::{collections::HashMap, sync::{Mutex, Arc}};

use crate::{ProcessFactory, JuizError};





pub struct CoreStore {
    process_factories: HashMap<String, Arc<Mutex<dyn ProcessFactory>>>,
}


impl CoreStore {
    pub fn new() -> CoreStore {
        CoreStore{process_factories: HashMap::new()}
    }

    pub fn register_process_factory(&mut self, pf: Arc<Mutex<dyn ProcessFactory>>) -> Result<&Arc<Mutex<dyn ProcessFactory>>, JuizError> {
        let typ = match pf.try_lock() {
            Err(_) => Err(JuizError::CoreStoreCanNotLockProcessFactoryError{}),
            Ok(pfv) => Ok(pfv.type_name().to_string())
            
        }?;
        if self.process_factories.contains_key(typ.as_str()) {
            return Err(JuizError::ProcessFactoryOfSameTypeNameAlreadyExistsError{});
        }
        self.process_factories.insert(typ.clone(), pf);
        Ok(self.process_factories.get(&typ).unwrap())
    }

    pub fn process_factory(&self, type_name: &str) -> Result<&Arc<Mutex<dyn ProcessFactory>>, JuizError> {
        match self.process_factories.get(type_name) {
            None => return Err(JuizError::ProcessFactoryCanNotFoundByTypeNameError{}),
            Some(pf) => return Ok(pf)
        }
    }
}