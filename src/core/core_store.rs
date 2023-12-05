use std::{collections::HashMap, sync::{Mutex, Arc}};

use crate::{ProcessFactory, JuizError, Identifier, Process, JuizResult, utils::juiz_lock};





pub struct CoreStore {
    process_factories: HashMap<String, Arc<Mutex<dyn ProcessFactory>>>,
    processes: HashMap<Identifier, Arc<Mutex<dyn Process>>> ,
}


impl CoreStore {
    pub fn new() -> CoreStore {
        CoreStore{process_factories: HashMap::new(), processes: HashMap::new(), 
        }
    }

    pub fn register_process_factory(&mut self, pf: Arc<Mutex<dyn ProcessFactory>>) -> JuizResult<Arc<Mutex<dyn ProcessFactory>>> {
        let type_name = juiz_lock(&pf)?.type_name().to_string();
        log::trace!("CoreStore::register_process_factory(ProcessFactory(type_name={:?})) called", type_name);
        if self.process_factories.contains_key(&type_name) {
            return Err(JuizError::ProcessFactoryOfSameTypeNameAlreadyExistsError{});
        }
        self.process_factories.insert(type_name, Arc::clone(&pf));
        Ok(pf)
    }

    pub fn process_factory(&self, type_name: &str) -> Result<&Arc<Mutex<dyn ProcessFactory>>, JuizError> {
        match self.process_factories.get(type_name) {
            None => return Err(JuizError::ProcessFactoryCanNotFoundByTypeNameError{}),
            Some(pf) => return Ok(pf)
        }
    }

    pub fn register_process(&mut self, p: Arc<Mutex<dyn Process>>) -> Result<Arc<Mutex<dyn Process>>, JuizError> {
        let id = juiz_lock(&p)?.identifier().clone();
        log::trace!("CoreStore::register_process(Process(id={:?}, manifest={})) called", id, juiz_lock(&p)?.manifest());
        self.processes.insert(id.clone(), p);
        self.process(&id)
    }

    pub fn process(&self, id: &Identifier) -> JuizResult<Arc<Mutex<dyn Process>>> {
        match self.processes.get(id) {
            Some(p) => Ok(Arc::clone(p)),
            None => {
                log::trace!("CoreStore::process(id={:?}) failed.", id);
                log::trace!(" - CoreStore includes processes[");
                for (k, _v) in self.processes.iter() {
                    log::trace!("    - {:?}", k);
                }
                log::trace!("]");
                Err(JuizError::ProcessCanNotFoundError{})
            }
            
        }
    }
}