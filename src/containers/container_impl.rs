

use std::{sync::{Arc, Mutex}, fmt::Display};
use crate::{Value, Identifier, value::obj_get_str, JuizResult, jvalue, JuizObject, identifier::create_identifier_from_manifest, object::{ObjectCore, JuizObjectCoreHolder, JuizObjectClass}};

use super::container::Container;

pub struct ContainerImpl<S> {
    core: ObjectCore,
    manifest: Value,
    pub t: Box<S>
}

fn _identifier_from_manifest(manifest: &Value) -> Identifier {
    match obj_get_str(manifest, "identifier") {
        Err(_) => obj_get_str(manifest, "name").unwrap().to_string(),
        Ok(id) => id.to_string()
    }
}

impl<S: 'static> ContainerImpl<S> {
    pub fn new(manifest: Value, t: Box<S>) -> JuizResult<Arc<Mutex<dyn Container>>> {
        let type_name = obj_get_str(&manifest, "type_name")?;
        let object_name = obj_get_str(&manifest, "name")?;
        Ok(Arc::new(Mutex::new(ContainerImpl{
            core: ObjectCore::create(JuizObjectClass::Container("ContainerImpl"), type_name, object_name),
            manifest, t
        })))
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
}

impl<S: 'static> Display for ContainerImpl<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ContainerImpl(identifier={}, manifest={})", self.identifier(), self.manifest())
    }
}