use crate::{Value, JuizResult, value::obj_get_str};

pub type Identifier = String;



pub fn identifier_new(broker_type_name: &str, broker_name: &str, class_name: &str, type_name: &str, object_name: &str) -> Identifier {
    // "core://core/process/inc0:increment_function"
    // "http://localhost:3000/procss/inc0:increment_function"
    // "{broker_type_name}://{broker_name}/{class_name}/{object_name}:{type_name}
    broker_type_name.to_string() + "://" + broker_name + "/" + class_name + "/" + object_name + "::" + type_name
}

pub fn identifier_from_manifest(broker_type_name: &str, broker_name: &str, class_name: &str, manifest: &Value) -> JuizResult<Identifier> {
    match obj_get_str(manifest, "identifier") {
        Ok(id) => Ok(id.to_string()), 
        Err(_) => {
            let object_name = obj_get_str(manifest, "name")?;
            let type_name = obj_get_str(manifest, "type_name")?;
            Ok(identifier_new(broker_type_name, broker_name, class_name, type_name, object_name))
        }
    }
}

pub(crate) fn _create_identifier(class_name: &str, type_name: &str, object_name: &str) -> Identifier {
    identifier_new("core", "core", class_name, type_name, object_name)
}

pub(crate) fn _create_factory_identifier(class_name: &str, type_name: &str) -> Identifier {
    identifier_new("core", "core", class_name, type_name, type_name)
}

pub(crate) fn create_identifier_from_manifest(class_name: &str, manifest: &Value) -> JuizResult<Identifier> {
    identifier_from_manifest("core", "core", class_name, manifest)
}
/*
pub(crate) fn create_factory_identifier_from_manifest(class_name: &str, manifest: &Value) -> JuizResult<Identifier> {
    let type_name = obj_get_str(manifest, "type_name")?;
    Ok(identifier_new("core", "core", class_name, type_name, type_name))
}
*/