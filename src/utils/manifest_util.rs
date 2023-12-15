use anyhow::Context;
use serde_json::Map;
use crate::{Value, JuizError, JuizResult, value::obj_get_str, Identifier, identifier::identifier_new};

pub fn construct_id(class_name: &str, type_name: &str, name: &str, broker_type: &str, broker_name: &str) -> Identifier {
    //broker_type.to_string() + "://" + broker_name + "/" + class_name + "/" + name + ":" + type_name
    identifier_new(broker_type, broker_name, class_name, type_name, name)
}

pub fn id_from_manifest(manifest: &serde_json::Value) -> JuizResult<Identifier> {
    let id_result = obj_get_str(manifest, "id");
    if id_result.is_ok() {
        return Ok(id_result.ok().unwrap().to_string());
    }
    let identifier_result = obj_get_str(manifest, "identifier");
    if identifier_result.is_ok() {
        return Ok(identifier_result.ok().unwrap().to_string());
    }

    let type_name = obj_get_str(manifest, "type_name").context("id_from_manifest() failed. 'id' or 'identifier' can't be found. Now the manifest must have class_name, type_name, and name. But type_name is not found.")?;
    let name = obj_get_str(manifest, "name").context("id_from_manifest() failed. 'id' or 'identifier' can't be found. Now the manifest must have class_name, type_name, and name. But name is not found.")?;
    let class_name = obj_get_str(manifest, "class_name").context("id_from_manifest() failed. 'id' or 'identifier' can't be found. Now the manifest must have class_name, type_name, and name. But class_name is not found.")?;

    match obj_get_str(manifest, "broker_type") {
        Err(_) => Ok(construct_id(class_name, type_name, name, "core", "core")),
        Ok(broker_type) => {
            let broker_name = obj_get_str(manifest, "broker_name").context("id_from_manifest() failed. 'id' or 'identifier' can't be found, but broker_type is found. Now the manifest must have type_name, name, and broker_name. But broker_name is not found.")?;
            Ok(construct_id(class_name, type_name, name, broker_type, broker_name))
        }     
    }
}

pub fn when_contains_do<T, F:Fn(&Value)->JuizResult<T>>(manifest: &Value, key: &str, func: F) -> JuizResult<Option<T>> {
    match manifest.as_object() {
        None => Err(anyhow::Error::from(JuizError::ValueIsNotObjectError { value: manifest.clone() })),
        Some(obj_v) => {
            match obj_v.get(key) {
                None => Ok(None), //Err(anyhow::Error::from(JuizError::ValueWithKeyNotFoundError { value: manifest.clone(), key: key.to_string() })),
                Some(v) => Ok(Some(func(v)?))
            }
        }
    }
}

pub fn when_contains_do_mut<T, F:FnMut(&Value)->JuizResult<T>>(manifest: &Value, key: &str, mut func: F) -> JuizResult<Option<T>> {
    match manifest.as_object() {
        None => Err(anyhow::Error::from(JuizError::ValueIsNotObjectError { value: manifest.clone() })),
        Some(obj_v) => {
            match obj_v.get(key) {
                None => Ok(None), //Err(anyhow::Error::from(JuizError::ValueWithKeyNotFoundError { value: manifest.clone(), key: key.to_string() })),
                Some(v) => Ok(Some(func(v)?))
            }
        }
    }
}

pub fn get_array<'a>(manifest: &'a Value) -> JuizResult<&'a Vec<Value>> {
    match manifest.as_array() {
        None => Err(anyhow::Error::from(JuizError::ValueIsNotArrayError{value: manifest.clone()})),
        Some(arr) => Ok(arr)
    }
}

pub fn get_hashmap<'a>(manifest: &'a Value) -> JuizResult<&'a Map<String, Value>> {
    match manifest.as_object() {
        None => Err(anyhow::Error::from(JuizError::ValueIsNotObjectError{value: manifest.clone()})),
        Some(arr) => Ok(arr)
    }
}

pub fn get_hashmap_mut<'a>(manifest: &'a mut Value) -> JuizResult<&'a mut Map<String, Value>> {
    if !manifest.is_object() {
        return Err(anyhow::Error::from(JuizError::ValueIsNotObjectError{value: manifest.clone()}));
    }
    return Ok(manifest.as_object_mut().unwrap());
}


pub fn get_value<'a>(manifest: &'a Value, key: &str) -> JuizResult<&'a Value> {
    let obj = get_hashmap(manifest)?;
    match obj.get(key) {
        None => {
            Err(anyhow::Error::from(JuizError::ValueWithKeyNotFoundError { value: manifest.clone(), key: key.to_string() }))
        },
        Some(v) => Ok(v)
    }

}

pub fn get_str<'a>(manifest: &'a Value, key: &str) -> JuizResult<&'a str> {
    let v = get_value(manifest, key)?;
    match v.as_str() {
        None => return Err(anyhow::Error::from(JuizError::ValueWithKeyIsNotStringError { value: manifest.clone(), key: key.to_string() })),
        Some(s) => return Ok(s)
    }           
}

pub fn type_name(manifest: &Value) -> JuizResult<&str> {
    get_str(manifest, "type_name")
}