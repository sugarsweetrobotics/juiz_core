use std::{collections::HashMap, sync::{Mutex, Arc}};

use anyhow::Context;

use crate::{ProcessFactory, JuizError, Identifier, Process, JuizResult, utils::{juiz_lock, manifest_util::get_hashmap_mut}, ContainerFactory, Container, ContainerProcessFactory, ContainerProcess, Value, jvalue};





pub struct CoreStore {
    process_factories: HashMap<String, Arc<Mutex<dyn ProcessFactory>>>,
    processes: HashMap<Identifier, Arc<Mutex<dyn Process>>> ,

    container_factories: HashMap<String, Arc<Mutex<dyn ContainerFactory>>>,
    containers: HashMap<Identifier, Arc<Mutex<dyn Container>>> ,

    container_process_factories: HashMap<String, Arc<Mutex<dyn ContainerProcessFactory>>>,
    container_processes: HashMap<Identifier, Arc<Mutex<dyn ContainerProcess>>> ,
}


impl CoreStore {
    pub fn new() -> CoreStore {
        CoreStore{process_factories: HashMap::new(), 
            processes: HashMap::new(), 
            container_factories: HashMap::new(),
            containers: HashMap::new(), 
            container_process_factories: HashMap::new(),
            container_processes: HashMap::new(),
        }
    }

    pub fn register_process_factory(&mut self, pf: Arc<Mutex<dyn ProcessFactory>>) -> JuizResult<Arc<Mutex<dyn ProcessFactory>>> {
        let type_name = juiz_lock(&pf)?.type_name().to_string();
        log::trace!("CoreStore::register_process_factory(ProcessFactory(type_name={:?})) called", type_name);
        if self.process_factories.contains_key(&type_name) {
            return Err(anyhow::Error::from(JuizError::ProcessFactoryOfSameTypeNameAlreadyExistsError{type_name: type_name}));
        }
        self.process_factories.insert(type_name, Arc::clone(&pf));
        Ok(pf)
    }

    pub fn register_container_factory(&mut self, cf: Arc<Mutex<dyn ContainerFactory>>) -> JuizResult<Arc<Mutex<dyn ContainerFactory>>> {
        let type_name = juiz_lock(&cf)?.type_name().to_string();
        log::trace!("CoreStore::register_container_factory(ContainerFactory(type_name={:?})) called", type_name);
        if self.container_factories.contains_key(&type_name) {
            return Err(anyhow::Error::from(JuizError::ContainerFactoryOfSameTypeNameAlreadyExistsError{type_name: type_name}));
        }
        self.container_factories.insert(type_name, Arc::clone(&cf));
        Ok(cf)
    }

    pub fn register_container_process_factory(&mut self, cf: Arc<Mutex<dyn ContainerProcessFactory>>) -> JuizResult<Arc<Mutex<dyn ContainerProcessFactory>>> {
        let type_name = juiz_lock(&cf)?.type_name().to_string();
        log::trace!("CoreStore::register_container_process_factory(ContainerProcessFactory(type_name={:?})) called", type_name);
        if self.container_process_factories.contains_key(&type_name) {
            return Err(anyhow::Error::from(JuizError::ContainerProcessFactoryOfSameTypeNameAlreadyExistsError{type_name: type_name}));
        }
        self.container_process_factories.insert(type_name, Arc::clone(&cf));
        Ok(cf)
    }

    pub fn process_factory(&self, type_name: &str) -> JuizResult<&Arc<Mutex<dyn ProcessFactory>>> {
        match self.process_factories.get(type_name) {
            None => return Err(anyhow::Error::from(JuizError::ProcessFactoryCanNotFoundError{type_name: type_name.to_string()})),
            Some(pf) => return Ok(pf)
        }
    }

    pub fn container_factory(&self, type_name: &str) -> JuizResult<&Arc<Mutex<dyn ContainerFactory>>> {
        match self.container_factories.get(type_name) {
            None => return Err(anyhow::Error::from(JuizError::ContainerFactoryCanNotFoundError{type_name: type_name.to_string()})),
            Some(pf) => return Ok(pf)
        }
    }

    pub fn container_process_factory(&self, type_name: &str) -> JuizResult<&Arc<Mutex<dyn ContainerProcessFactory>>> {
        match self.container_process_factories.get(type_name) {
            None => return Err(anyhow::Error::from(JuizError::ContainerProcessFactoryCanNotFoundError{type_name: type_name.to_string()})),
            Some(pf) => return Ok(pf)
        }
    }


    pub fn register_process(&mut self, p: Arc<Mutex<dyn Process>>) -> JuizResult<Arc<Mutex<dyn Process>>> {
        let id = juiz_lock(&p)?.identifier().clone();
        log::trace!("CoreStore::register_process(Process(id={:?}, manifest={})) called", id, juiz_lock(&p)?.manifest());
        self.processes.insert(id.clone(), p);
        self.process(&id)
    }

    pub fn register_container(&mut self, p: Arc<Mutex<dyn Container>>) -> JuizResult<Arc<Mutex<dyn Container>>> {
        let id = juiz_lock(&p)?.identifier().clone();
        log::trace!("CoreStore::register_container(Process(id={:?}, manifest={})) called", id, juiz_lock(&p)?.manifest());
        self.containers.insert(id.clone(), p);
        self.container(&id)
    }

    pub fn register_container_process(&mut self, p: Arc<Mutex<dyn ContainerProcess>>) -> JuizResult<Arc<Mutex<dyn ContainerProcess>>> {
        let id = juiz_lock(&p)?.identifier().clone();
        log::trace!("CoreStore::register_container_process(Process(id={:?}, manifest={})) called", id, juiz_lock(&p)?.manifest());
        self.container_processes.insert(id.clone(), p);
        self.container_process(&id).context("register_container_process()")
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
                Err(anyhow::Error::from(JuizError::ProcessCanNotFoundByIdError{id: id.clone()}))
            }
            
        }
    }

    pub fn container(&self, id: &Identifier) -> JuizResult<Arc<Mutex<dyn Container>>> {
        match self.containers.get(id) {
            Some(p) => Ok(Arc::clone(p)),
            None => {
                log::trace!("CoreStore::container(id={:?}) failed.", id);
                log::trace!(" - CoreStore includes containers[");
                for (k, _v) in self.containers.iter() {
                    log::trace!("    - {:?}", k);
                }
                log::trace!("]");
                Err(anyhow::Error::from(JuizError::ContainerCanNotFoundByIdError{id: id.clone()}))
            }
        }
    }


    pub fn container_process(&self, id: &Identifier) -> JuizResult<Arc<Mutex<dyn ContainerProcess>>> {
        match self.container_processes.get(id) {
            Some(p) => Ok(Arc::clone(p)),
            None => {
                log::trace!("CoreStore::container_process(id={:?}) failed.", id);
                log::trace!(" - CoreStore includes container_procesess[");
                for (k, _v) in self.container_processes.iter() {
                    log::trace!("    - {:?}", k);
                }
                log::trace!("]");
                Err(anyhow::Error::from(JuizError::ContainerCanNotFoundByIdError{id: id.clone()}))
            }
        }
    }

    fn processe_factories_profile_full(&self) -> JuizResult<Value> {
        let mut process_factories = jvalue!({});
        let p_hashmap = get_hashmap_mut(&mut process_factories)?;
        self.process_factories.iter().for_each(|(identifier, arc_proc)| {
            match juiz_lock(&arc_proc) {
                Err(e) => {
                    p_hashmap.insert(identifier.clone(), jvalue!(format!("Err({})", e)));
                },
                Ok(p) => {
                    p_hashmap.insert(identifier.clone(), p.profile_full().unwrap());
                }
            }
        });
        Ok(process_factories)
    }

    fn container_factories_profile_full(&self) -> JuizResult<Value> {
        let mut container_factories = jvalue!({});
        let p_hashmap = get_hashmap_mut(&mut container_factories)?;
        self.container_factories.iter().for_each(|(identifier, arc_proc)| {
            match juiz_lock(&arc_proc) {
                Err(e) => {
                    p_hashmap.insert(identifier.clone(), jvalue!(format!("Err({})", e)));
                },
                Ok(p) => {
                    p_hashmap.insert(identifier.clone(), p.profile_full().unwrap());
                }
            }
        });
        Ok(container_factories)
    }
    
    fn container_processe_factories_profile_full(&self) -> JuizResult<Value> {
        let mut container_process_factories = jvalue!({});
        let p_hashmap = get_hashmap_mut(&mut container_process_factories)?;
        self.container_process_factories.iter().for_each(|(identifier, arc_proc)| {
            match juiz_lock(&arc_proc) {
                Err(e) => {
                    p_hashmap.insert(identifier.clone(), jvalue!(format!("Err({})", e)));
                },
                Ok(p) => {
                    p_hashmap.insert(identifier.clone(), p.profile_full().unwrap());
                }
            }
        });
        Ok(container_process_factories)
    }

    fn processes_profile_full(&self) -> JuizResult<Value> {
        let mut processes = jvalue!({});
        let p_hashmap = get_hashmap_mut(&mut processes)?;
        self.processes.iter().for_each(|(identifier, arc_proc)| {
            match juiz_lock(&arc_proc) {
                Err(e) => {
                    p_hashmap.insert(identifier.clone(), jvalue!(format!("Err({})", e)));
                },
                Ok(p) => {
                    p_hashmap.insert(identifier.clone(), p.profile_full().unwrap());
                }
            }
        });
        Ok(processes)
    }

    fn containers_profile_full(&self) -> JuizResult<Value> {
        let mut containers = jvalue!({});
        let c_hashmap = get_hashmap_mut(&mut containers)?;
        self.containers.iter().for_each(|(identifier, arc_container)| {
            match juiz_lock(&arc_container) {
                Err(e) => {
                    c_hashmap.insert(identifier.clone(), jvalue!(format!("Err({})", e)));
                },
                Ok(c) => {
                    c_hashmap.insert(identifier.clone(), c.profile_full().unwrap());
                }
            }
        });
        Ok(containers)
    }

    fn container_processes_profile_full(&self) -> JuizResult<Value> {
        let mut container_processes = jvalue!({});
        let cp_hashmap = get_hashmap_mut(&mut container_processes)?;
        self.container_processes.iter().for_each(|(identifier, arc_con_proc)| {
            match juiz_lock(&arc_con_proc) {
                Err(e) => {
                    cp_hashmap.insert(identifier.clone(), jvalue!(format!("Err({})", e)));
                },
                Ok(cp) => {
                    cp_hashmap.insert(identifier.clone(), cp.profile_full().unwrap());
                }
            }
        });
        Ok(container_processes)
    }


    pub fn profile_full(&self) -> JuizResult<Value> {
        
        Ok(jvalue!({
            "process_factories": self.processe_factories_profile_full()?,
            "container_factories": self.container_factories_profile_full()?,
            "container_process_factories": self.container_processe_factories_profile_full()?,
            "processes": self.processes_profile_full()?,
            "containers": self.containers_profile_full()?,
            "container_processes": self.container_processes_profile_full()?,
        }))
    }
}