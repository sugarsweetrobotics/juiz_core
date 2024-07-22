

use std::{collections::HashMap, ffi::{CStr, CString}};

use serde_json::Map;
pub use serde_json::json as jvalue;

use crate::{utils::{get_array, get_hashmap, manifest_util::{get_array_mut, get_hashmap_mut}}, JuizError, JuizResult};

#[repr(transparent)]
//#[repr(C)]
pub struct JValue(pub serde_json::Value, );

pub type Value=serde_json::Value;

pub fn load_str(json_str: &str) -> JuizResult<Value> {
    serde_json::from_str(json_str).or_else(|e| { Err(anyhow::Error::from(e)) })
}

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

pub fn obj_insert(value: &mut Value, key: &str, data: Value) -> JuizResult<()> {
    get_hashmap_mut(value)?.insert(key.to_string(), data);
    Ok(())
}

pub fn obj_merge_mut(value: &mut Value, data: &Value) -> JuizResult<()> {
    let mut_map = get_hashmap_mut(value)?;
    let data_map = get_hashmap(data)?;
    for (k, v) in data_map.iter() {
        mut_map.insert(k.clone(), v.clone());
    }
    Ok(())
}

pub fn value_merge(value: Value, data: &Value) -> JuizResult<Value> {
    if value.is_object() && data.is_object() {
        obj_merge(value, data)
    } else if value.is_array() && data.is_array() {
        array_merge(value, data)
    } else {
        Err(anyhow::Error::from(JuizError::ValueMergeError{message:"Two values have different data type.".to_owned()}))
    }
}

pub fn obj_merge(mut value: Value, data: &Value) -> JuizResult<Value> {
    let mut_map = get_hashmap_mut(&mut value)?;
    let data_map = get_hashmap(data)?;
    for (k, v) in data_map.iter() {
        if mut_map.contains_key(k) {
            let vv = mut_map.remove(k).unwrap();
            mut_map.insert(k.clone(), value_merge(vv, v)?);
        } else {
            mut_map.insert(k.clone(), v.clone());
        }
    }
    Ok(value)
}

pub fn array_merge(mut value: Value, data: &Value) -> JuizResult<Value> {
    let mut_vec = get_array_mut(&mut value)?;
    let data_vec = get_array(data)?;
    for v in data_vec.iter() {
        mut_vec.push(v.clone());
    }
    Ok(value)
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

pub fn obj_get_i64<'a>(value: &'a Value, key: &str) -> JuizResult<i64> {
    let obj = obj_get(value, key)?;
    match obj.as_i64() {
        None  => Err(anyhow::Error::from(JuizError::ValueWithKeyIsNotNumError{value: value.clone(), key: key.to_string()})),
        Some(s) => return Ok(s)
    }
}

pub fn obj_get_f64<'a>(value: &'a Value, key: &str) -> JuizResult<f64> {
    let obj = obj_get(value, key)?;
    match obj.as_f64() {
        None  => {
            match obj.as_i64() {
                None => Err(anyhow::Error::from(JuizError::ValueWithKeyIsNotNumError{value: value.clone(), key: key.to_string()})),    
                Some(i) => return Ok(i as f64)
            }
        },
        Some(s) => return Ok(s)
    }
}

pub fn obj_get_bool(value: &Value, key: &str) -> JuizResult<bool> {
    let obj = obj_get(value, key)?;
    match obj.as_bool() {
        None  => Err(anyhow::Error::from(JuizError::ValueWithKeyIsNotBoolError{value: value.clone(), key: key.to_string()})),
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

pub fn obj_get_hashmap<'a>(value: &'a Value, key: &str) -> JuizResult<HashMap<String, Value>> {
    let map = obj_get_obj(&value, key)?;
    let mut ret_map: HashMap<String, Value> = HashMap::with_capacity(map.len());
    for (k, v) in map.iter() {
        ret_map.insert(k.clone(), v.clone());
    }
    Ok(ret_map)

}



#[no_mangle]
pub unsafe extern "C" fn value_is_int(value: *mut Value) -> bool {
    value.as_ref().unwrap().is_i64()
}

#[no_mangle]
pub unsafe extern "C" fn value_get_int(value: *mut Value, output: *mut i64) -> i64 {
    match value.as_ref().unwrap().as_i64() {
        Some(v) => {
            *output = v;
            0
        },
        None => {
            -1
        }
    }
}


#[no_mangle]
pub unsafe extern "C" fn value_is_uint(value: *mut Value) -> bool {
    value.as_ref().unwrap().is_u64()
}

#[no_mangle]
pub unsafe extern "C" fn value_get_uint(value: *mut Value, output: *mut u64) -> i64 {
    match value.as_ref().unwrap().as_u64() {
        Some(v) => {
            *output = v;
            0
        },
        None => {
            -1
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn value_is_float(value: *mut Value) -> bool {
    value.as_ref().unwrap().is_f64()
}

#[no_mangle]
pub unsafe extern "C" fn value_get_float(value: *mut Value, output: *mut f64) -> i64 {
    match value.as_ref().unwrap().as_f64() {
        Some(v) => {
            *output = v;
            0
        },
        None => {
            -1
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn value_is_boolean(value: *mut Value) -> bool {
    value.as_ref().unwrap().is_boolean()
}

#[no_mangle]
pub unsafe extern "C" fn value_get_bool(value: *mut Value, output: *mut bool) -> i64 {
    match value.as_ref().unwrap().as_bool() {
        Some(v) => {
            *output = v;
            0
        },
        None => {
            -1
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn value_is_string(value: *mut Value) -> bool {
    value.as_ref().unwrap().is_string()
}

#[no_mangle]
pub unsafe extern "C" fn value_get_string(value: *mut Value, output: *mut *const u8) -> i64 {
    value.as_ref().unwrap().as_str().and_then(|v| { *output = v.as_ptr(); Some(0)}).or(Some(-1)).unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn value_is_object(value: *mut Value) -> bool {
    value.as_ref().unwrap().is_object()
}

#[no_mangle]
pub unsafe extern "C" fn value_object_get_value(value: *mut Value, key: *const i8) -> *mut Value {
    let obj = value.as_mut().unwrap().as_object_mut().unwrap();
    match obj.get_mut(CStr::from_ptr(key).to_str().unwrap()) {
        Some(v) => { 
            return v;
        },
        None => {
            return std::ptr::null_mut();
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn value_object_foreach(value: *mut Value, callback: extern fn(*mut i8, *mut Value) -> ()) -> () {
    let obj = value.as_mut().unwrap().as_object_mut().unwrap();
    for (k, v) in obj.iter_mut() {
        let key = CString::new(k.as_str()).unwrap();
        callback(key.into_raw(), v);
    }
}

#[no_mangle]
pub unsafe extern "C" fn value_object_set_int(value: *mut Value, key: *const i8, v: i64) -> *mut Value {
    let obj = value.as_mut().unwrap().as_object_mut().unwrap();
    let keystr = CStr::from_ptr(key).to_str().unwrap().to_owned();
    obj.insert(keystr.clone(), jvalue!(v));
    obj.get_mut(&keystr).unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn value_object_set_uint(value: *mut Value, key: *const i8, v: u64) -> *mut Value {
    let obj = value.as_mut().unwrap().as_object_mut().unwrap();
    let keystr = CStr::from_ptr(key).to_str().unwrap().to_owned();
    obj.insert(keystr.clone(), jvalue!(v));
    obj.get_mut(&keystr).unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn value_object_set_float(value: *mut Value, key: *const i8, v: f64) -> *mut Value {
    let obj = value.as_mut().unwrap().as_object_mut().unwrap();
    let keystr = CStr::from_ptr(key).to_str().unwrap().to_owned();
    obj.insert(keystr.clone(), jvalue!(v));
    obj.get_mut(&keystr).unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn value_object_set_string(value: *mut Value, key: *const i8, v: *const i8) -> *mut Value {
    let obj = value.as_mut().unwrap().as_object_mut().unwrap();
    let keystr = CStr::from_ptr(key).to_str().unwrap().to_owned();
    obj.insert(keystr.clone(), jvalue!(CStr::from_ptr(v).to_str().unwrap()));
    obj.get_mut(&keystr).unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn value_object_set_bool(value: *mut Value, key: *const i8, v: i64) -> *mut Value {
    let obj = value.as_mut().unwrap().as_object_mut().unwrap();
    let keystr = CStr::from_ptr(key).to_str().unwrap().to_owned();
    obj.insert(keystr.clone(), jvalue!(v != 0));
    obj.get_mut(&keystr).unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn value_is_array(value: *mut Value) -> bool {
    value.as_ref().unwrap().is_array()
}

#[no_mangle]
pub unsafe extern "C" fn value_array_foreach(value: *mut Value, callback: extern fn(*mut Value) -> ()) -> () {
    let obj = value.as_mut().unwrap().as_array_mut().unwrap();
    for v in obj.iter_mut() {
        callback(v);
    }
}

#[no_mangle]
pub unsafe extern "C" fn value_object_set_empty_array(value: *mut Value, key: *const i8) -> *mut Value {
    let obj = value.as_mut().unwrap().as_object_mut().unwrap();
    let keystr = CStr::from_ptr(key).to_str().unwrap().to_owned();
    obj.insert(keystr.clone(), jvalue!([]));
    obj.get_mut(&keystr).unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn value_object_set_empty_object(value: *mut Value, key: *const i8) -> *mut Value {
    let obj = value.as_mut().unwrap().as_object_mut().unwrap();
    let keystr = CStr::from_ptr(key).to_str().unwrap().to_owned();
    obj.insert(keystr.clone(), jvalue!({}));
    obj.get_mut(&keystr).unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn value_array_push_int(value: *mut Value, v: i64) -> *mut Value {
    let obj = value.as_mut().unwrap().as_array_mut().unwrap();
    obj.push(jvalue!(v));
    obj.last_mut().unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn value_array_push_uint(value: *mut Value, v: u64) -> *mut Value {
    let obj = value.as_mut().unwrap().as_array_mut().unwrap();
    obj.push(jvalue!(v));
    obj.last_mut().unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn value_array_push_float(value: *mut Value, v: f64) -> *mut Value {
    let obj = value.as_mut().unwrap().as_array_mut().unwrap();
    obj.push(jvalue!(v));
    obj.last_mut().unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn value_array_push_bool(value: *mut Value, v: i64) -> *mut Value {
    let obj = value.as_mut().unwrap().as_array_mut().unwrap();
    obj.push(jvalue!(v != 0));
    obj.last_mut().unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn value_array_push_string(value: *mut Value, v: *const i8) -> *mut Value {
    let obj = value.as_mut().unwrap().as_array_mut().unwrap();
    obj.push(jvalue!(CStr::from_ptr(v).to_str().unwrap()));
    obj.last_mut().unwrap()
}


#[no_mangle]
pub unsafe extern "C" fn value_array_push_empty_object(value: *mut Value) -> *mut Value {
    let obj = value.as_mut().unwrap().as_array_mut().unwrap();
    obj.push(jvalue!({}));
    obj.last_mut().unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn value_array_push_empty_array(value: *mut Value) -> *mut Value {
    let obj = value.as_mut().unwrap().as_array_mut().unwrap();
    obj.push(jvalue!([]));
    obj.last_mut().unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn value_is_null(value: *mut Value) -> bool {
    value.as_ref().unwrap().is_null()
}
