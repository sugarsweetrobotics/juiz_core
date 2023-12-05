

pub use serde_json::json as jvalue;

use crate::JuizError;

#[repr(transparent)]
//#[repr(C)]
pub struct JValue(pub serde_json::Value, );

pub type Value=serde_json::Value;

pub fn as_obj<'a>(value: &'a Value) -> Result<&'a serde_json::Map<String, Value>, JuizError> {
    match value.as_object() {
        None => return Err(JuizError::ValueAccessValueIsNotObjectError{}),
        Some(hashmap) => Ok(hashmap)
    }
}

pub fn obj_get<'a>(value: &'a Value, key: &str) -> Result<&'a Value, JuizError> {
    match value.as_object() {
        None => return Err(JuizError::ValueAccessValueIsNotObjectError{}),
        Some(hashmap) => {
            match hashmap.get(key) {
                None => return Err(JuizError::ValueAccessKeyNotFoundError{}),
                Some(value_for_key) => Ok(value_for_key)
            }
        }
    }
}

pub fn obj_get_str<'a>(value: &'a Value, key: &str) -> Result<&'a str, JuizError> {
    let obj = obj_get(value, key)?;
    match obj.as_str() {
        None  => Err(JuizError::ValueAccessValueIsNotStrError{}),
        Some(s) => return Ok(s)
    }
}