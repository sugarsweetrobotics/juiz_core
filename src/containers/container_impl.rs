

use std::{collections::HashMap, fmt::Display, ops::{Deref, DerefMut}};
use crate::{prelude::*, processes::{proc_lock, proc_lock_mut}};
use crate::{object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore}, value::obj_get_str, JuizObject};

use super::container_ptr;

pub struct ContainerImpl<S> {
    core: ObjectCore,
    manifest: Value,
    pub t: Box<S>,
    processes: HashMap<String, ProcessPtr>,
}

fn _identifier_from_manifest(manifest: &Value) -> Identifier {
    match obj_get_str(manifest, "identifier") {
        Err(_) => obj_get_str(manifest, "name").unwrap().to_string(),
        Ok(id) => id.to_string()
    }
}

impl<S: 'static> ContainerImpl<S> {
    pub fn new(manifest: Value, t: Box<S>) -> JuizResult<ContainerPtr> {
        let type_name = obj_get_str(&manifest, "type_name")?;
        let object_name = obj_get_str(&manifest, "name")?;
        Ok(container_ptr(ContainerImpl{
            core: ObjectCore::create(JuizObjectClass::Container("ContainerImpl"), type_name, object_name),
            manifest, 
            t,
            processes: HashMap::new(),
        }))
    }
}

impl<S: 'static> Deref for ContainerImpl<S> {
    type Target = Box<S>;

    fn deref(&self) -> &Self::Target {
        &self.t
    }
}


impl<S: 'static> DerefMut for ContainerImpl<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.t
    }
}


impl<S: 'static> JuizObjectCoreHolder for ContainerImpl<S> {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}

impl<S: 'static> JuizObject for ContainerImpl<S> {}

impl<S: 'static> Container for ContainerImpl<S> {

    fn manifest(&self) -> &Value {
        &self.manifest
    }

    fn process(&self, name_or_id: &String) -> Option<ProcessPtr> {
        for (k, p) in self.processes.iter() {
            if k == name_or_id {
                return Some(p.clone());
            }
            match IdentifierStruct::try_from(name_or_id.clone()) {
                Ok(s) => {
                    if &s.object_name == name_or_id {
                        return Some(p.clone())
                    }
                }
                Err(_) => {},
            }
        }
        None
    }

    fn processes(&self) -> Vec<ProcessPtr> {
        self.processes.iter().map(|(_k, p)|{p.clone()}).collect::<Vec<ProcessPtr>>()
    }

    fn register_process(&mut self, p: ProcessPtr) -> JuizResult<ProcessPtr> {
        let id = proc_lock(&p)?.identifier().clone();
        self.processes.insert(id, p.clone());
        Ok(p)
    }

    fn purge_process(&mut self, name_or_id: &String) -> JuizResult<()> {
        match self.process(name_or_id) {
            Some(p) => {
                let _ = p.write().unwrap().purge()?;
                self.processes.remove(p.read().unwrap().identifier());
                Ok(())
            },
            None => {
                Err(anyhow::Error::from(JuizError::ObjectCanNotFoundByIdError { id: name_or_id.clone() }))
            },
        }
    }

    fn clear(&mut self) -> JuizResult<()> {
        for (k, p) in self.processes.iter() {
            proc_lock_mut(p)?.purge()?;
        }
        self.processes.clear();
        Ok(())
    }
    
    
}

impl<S: 'static> Display for ContainerImpl<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ContainerImpl(identifier={}, manifest={})", self.identifier(), self.manifest())
    }
}

impl<S> Drop for ContainerImpl<S> {
    fn drop(&mut self) {
        log::trace!("ContainerImpl()::drop() called");
        self.processes.clear();
        log::trace!("ContainerImpl()::drop() exit");
    }
}