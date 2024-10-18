

use std::{collections::HashMap, fmt::Display, ops::{Deref, DerefMut}, sync::{Arc, RwLock}};
use crate::{prelude::*, value::obj_merge};

use crate::{object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore}, value::obj_get_str};



pub struct ContainerImpl<S: 'static> {
    core: ObjectCore,
    manifest: Value,
    pub t: Box<S>,
    processes: HashMap<String, ProcessPtr>,
    parent_container: Option<ContainerPtr>,
}

fn _identifier_from_manifest(manifest: &Value) -> Identifier {
    match obj_get_str(manifest, "identifier") {
        Err(_) => obj_get_str(manifest, "name").unwrap().to_string(),
        Ok(id) => id.to_string()
    }
}

impl<S: 'static> ContainerImpl<S> {
    pub fn new(manifest: Value, t: Box<S>) -> JuizResult<Self> {
        let type_name = obj_get_str(&manifest, "type_name")?;
        let object_name = obj_get_str(&manifest, "name")?;
        Ok(ContainerImpl{
            core: ObjectCore::create(JuizObjectClass::Container("ContainerImpl"), type_name, object_name),
            manifest, 
            t,
            processes: HashMap::new(),
            parent_container: None,
        })
    }

    pub fn new_with_parent(manifest: Value, t: Box<S>, parent_container: ContainerPtr) -> JuizResult<Self> {
        let type_name = obj_get_str(&manifest, "type_name")?;
        let object_name = obj_get_str(&manifest, "name")?;
        Ok(ContainerImpl{
            core: ObjectCore::create(JuizObjectClass::Container("ContainerImpl"), type_name, object_name),
            manifest, 
            t,
            processes: HashMap::new(),
            parent_container: Some(parent_container),
        })
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

impl<S: 'static> JuizObject for ContainerImpl<S> {
    fn profile_full(&self) -> JuizResult<Value> {
        log::trace!("ContainerImpl({})::profile_full() called", self.identifier());
        let ids = self.processes().iter().map(|p| -> JuizResult<Identifier> { Ok(p.identifier().clone()) }).collect::<JuizResult<Vec<Identifier>>>()?;
        obj_merge(self.core.profile_full()?, &jvalue!({
            "processes": ids}))
    }
}

impl<S: 'static> Container for ContainerImpl<S> {

    fn manifest(&self) -> &Value {
        &self.manifest
    }

    fn process(&self, name_or_id: &String) -> Option<ProcessPtr> {
        log::trace!("ContainerImpl({}):process({}) called", self.identifier(), name_or_id);
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
        let id = p.identifier().clone();
        self.processes.insert(id, p.clone());
        Ok(p)
    }

    fn purge_process(&mut self, name_or_id: &String) -> JuizResult<()> {
        log::trace!("ContainerImpl({})::purge_process({}) called", self.identifier(), name_or_id);

        match self.process(name_or_id) {
            Some(p) => {
                //let _ = p.write().unwrap().purge()?;
                let _res = self.processes.remove(p.identifier());
                //log::trace!("ContainerImpl::purge_process({}) result: {:?}", name_or_id, res.is_some());
                Ok(())
            },
            None => {
                log::error!("purge_process({}) failed. Process is None.", name_or_id);
                Err(anyhow::Error::from(JuizError::ObjectCanNotFoundByIdError { id: name_or_id.clone() }))
            },
        }
    }

    fn clear(&mut self) -> JuizResult<()> {
        log::trace!("ContainerImpl({})::clear() called", self.identifier());
        for (_k, p) in self.processes.iter() {
            p.lock_mut()?.purge()?;
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

impl<S: 'static> Drop for ContainerImpl<S> {
    fn drop(&mut self) {
        let id = self.type_name().to_owned();
        log::info!("ContainerImpl({})::drop() called", id);
        self.processes.clear();
        log::trace!("ContainerImpl({})::drop() exit", id);
    }
}