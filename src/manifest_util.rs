use serde_json::Map;

use crate::{Value, JuizError, JuizResult, value::obj_get_str};



pub fn id_from_manifest(manifest: &serde_json::Value) -> JuizResult<&str> {
    obj_get_str(manifest, "name")
}


pub fn when_contains_do<T, F:Fn(&Value)->JuizResult<T>>(manifest: &Value, key: &str, func: F) -> JuizResult<T> {
    match manifest.as_object() {
        None => Err(JuizError::ManifestIsNotObjectError {  }),
        Some(obj_v) => {
            match obj_v.get(key) {
                None => Err(JuizError::ManifestDoesNotContainsKeyError{}),
                Some(v) => func(v)
            }
        }
    }
}


pub fn get_array<'a>(manifest: &'a Value) -> Result<&'a Vec<Value>, JuizError> {
    match manifest.as_array() {
        None => Err(JuizError::ManifestIsNotArrayError{}),
        Some(arr) => Ok(arr)
    }
}

pub fn get_hashmap<'a>(manifest: &'a Value) -> Result<&'a Map<String, Value>, JuizError> {
    match manifest.as_object() {
        None => Err(JuizError::ManifestIsNotObjectError{}),
        Some(arr) => Ok(arr)
    }
}


pub fn get_value<'a>(manifest: &'a Value, key: &str) -> Result<&'a Value, JuizError> {
    match manifest.as_object() {
        None => return Err(JuizError::ManifestIsNotObjectError{}),
        Some(obj) => {
            match obj.get(key) {
                None => {
                    log::error!("get_value(manifest={:?}, key={:?}) failed.", manifest, key);
                    Err(JuizError::ManifestDoesNotIncludeKeyError{})
                },
                Some(v) => Ok(v)
            }
        }
    }
}

pub fn get_str<'a>(manifest: &'a Value, key: &str) -> Result<&'a str, JuizError> {
    let v = get_value(manifest, key)?;
    match v.as_str() {
        None => return Err(JuizError::ManifestTypeNameIsNotStringError{}),
        Some(s) => return Ok(s)
    }           
}

pub fn type_name(manifest: &Value) -> Result<&str, JuizError> {
    get_str(manifest, "type_name")
}