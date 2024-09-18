use crate::prelude::*;

pub struct SubSystemRecord {
    manifest: Value
}

fn assert_subsystem_manifest(manifest: Value) -> JuizResult<Value> {
    Ok(manifest)
}

impl SubSystemRecord {

    pub fn new(manifest: Value) -> JuizResult<Self> {
        Ok(SubSystemRecord{ manifest: assert_subsystem_manifest(manifest)? })
    }

    pub fn get_manifest(&self) -> &Value {
        &self.manifest
    }
}