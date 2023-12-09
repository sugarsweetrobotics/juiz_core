use serde_json::Map;
use crate::{Value, JuizError, JuizResult, value::obj_get_str};

pub fn id_from_manifest(manifest: &serde_json::Value) -> JuizResult<String> {
    match obj_get_str(manifest, "type_name") {
        Ok(type_name) => {
            let name = obj_get_str(manifest, "name")?;
            match obj_get_str(manifest, "broker_type") {
                Err(_) => Ok("core://".to_string() + name + ":" + type_name),
                Ok(broker_type) => Ok(broker_type.to_string() + "://" + name + ":" + type_name)
            }
        },
        Err(_) => Ok(obj_get_str(manifest, "identifier")?.to_string())
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