

use serde_json::Map;
pub use serde_json::json as jvalue;

use crate::{JuizError, JuizResult};

#[repr(transparent)]
//#[repr(C)]
pub struct JValue(pub serde_json::Value, );

pub type Value=serde_json::Value;

pub fn as_obj<'a>(value: &'a Value) -> JuizResult<&'a serde_json::Map<String, Value>> {
    match value.as_object() {
        None => return Err(anyhow::Error::from(JuizError::ValueIsNotObjectError{value: value.clone()})),
        Some(hashmap) => Ok(hashmap)
    }
}

pub fn obj_get<'a>(value: &'a Value, key: &str) -> JuizResult<&'a Value> {
    match value.as_object() {
        None => return Err(anyhow::Error::from(JuizError::ValueWithKeyIsNotObjectError{value: value.clone(), key: key.to_string()})),
        Some(hashmap) => {
            match hashmap.get(key) {
                None => return Err(anyhow::Error::from(JuizError::ValueWithKeyNotFoundError{value: value.clone(), key: key.to_string()})),
                Some(value_for_key) => Ok(value_for_key)
            }
        }
    }
}

pub fn obj_get_mut<'a>(value: &'a mut Value, key: &str) -> JuizResult<&'a mut Value> {
    {
        if ! value.is_object() {
            return Err(anyhow::Error::from(JuizError::ValueWithKeyIsNotObjectError{value: value.clone(), key: key.to_string()}));
        }
        
        {
            let hashmap = value.as_object().unwrap();
            if hashmap.contains_key(key) {
                return Ok(value.as_object_mut().unwrap().get_mut(key).unwrap());
            }
        }
    }
    Err(anyhow::Error::from(JuizError::ValueWithKeyNotFoundError{value: value.clone(), key: key.to_string()}))
}


pub fn obj_get_str<'a>(value: &'a Value, key: &str) -> JuizResult<&'a str> {
    let obj = obj_get(value, key)?;
    match obj.as_str() {
        None  => Err(anyhow::Error::from(JuizError::ValueWithKeyIsNotStringError{value: value.clone(), key: key.to_string()})),
        Some(s) => return Ok(s)
    }
}

pub fn obj_get_obj<'a>(value: &'a Value, key: &str) -> JuizResult<&'a Map<String, Value>> {
    let obj = obj_get(value, key)?;
    match obj.as_object() {
        None  => Err(anyhow::Error::from(JuizError::ValueWithKeyIsNotObjectError{value: value.clone(), key: key.to_string()})),
        Some(s) => return Ok(s)
    }
}