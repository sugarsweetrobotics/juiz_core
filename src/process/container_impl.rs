

use std::{sync::{Arc, Mutex}, fmt::Display};
use crate::{Value, Identifier, value::obj_get_str, JuizResult, jvalue};

use super::container::Container;

pub struct ContainerImpl<S> {
    identifier: Identifier,
    manifest: Value,
    pub t: Box<S>
}

fn identifier_from_manifest(manifest: &Value) -> Identifier {
    match obj_get_str(manifest, "identifier") {
        Err(_) => obj_get_str(manifest, "name").unwrap().to_string(),
        Ok(id) => id.to_string()
    }
}

impl<S: 'static> ContainerImpl<S> {
    pub fn new(manifest: Value, t: Box<S>) -> Arc<Mutex<dyn Container>> {
        Arc::new(Mutex::new(ContainerImpl{
            identifier: identifier_from_manifest(&manifest),
            manifest, t
        }))
    }
}

impl<S: 'static> Container for ContainerImpl<S> {
    fn identifier(&self) -> &crate::Identifier {
        &self.identifier
    }

    fn manifest(&self) -> &Value {
        &self.manifest
    }


    fn profile_full(&self) -> JuizResult<Value> {
        Ok(jvalue!({
            "name": self.manifest.get("name").unwrap(),
            "identifier": self.manifest.get("identifier").unwrap()
        }))
    }
}

impl<S: 'static> Display for ContainerImpl<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ContainerImpl(identifier={}, manifest={})", self.identifier(), self.manifest())
    }
}