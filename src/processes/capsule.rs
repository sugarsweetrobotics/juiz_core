use std::sync::{Arc, Mutex};
use std::{collections::HashMap, mem::swap};

use opencv::core::Mat;

use crate::{jvalue, JuizError, Value};
pub use crate::processes::capsule_map::CapsuleMap;
pub use crate::processes::capsule_ptr::CapsulePtr;

#[derive(Clone, Debug)]
pub enum CapsuleValue {
    Empty(()),
    Value(Value),
    Mat(Mat),
}

impl From<Value> for CapsuleValue {
    fn from(value: Value) -> Self { Self::Value( value ) }
}

impl From<Mat> for CapsuleValue {
    fn from(value: Mat) -> Self { Self::Mat( value ) }
}

impl CapsuleValue {

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Empty(_) => return true, 
            _ => return false
        }
    }

    pub fn is_value(&self) -> bool {
        match self {
            Self::Value(_) => return true, 
            _ => return false
        }
    }

    pub fn as_value(&self) -> Option<&Value> {
        match self {
            Self::Value(v) => Some(v),
            _ => None
        }
    }

    pub fn as_value_mut(&mut self) -> Option<&mut Value> {
        match self {
            Self::Value(v) => Some(v),
            _ => None
        }
    }

    
    pub fn to_value(self) -> Option<Value> {
        match self {
            Self::Value(v) => Some(v),
            _ => None
        }
    }
    

    pub fn is_mat(&self) -> bool {
        match self {
            Self::Mat(_) => return true, 
            _ => return false
        }
    }

    pub fn as_mat(&self) -> Option<&Mat> {
        match self {
            Self::Mat(v) => Some(v),
            _ => None
        }
    }
    /*
    pub fn to_mat(&self) -> Option<Mat> {
        match self {
            Self::Mat(v) => Some(v.clone()),
            _ => None
        }
    }*/
}


#[derive(Clone, Debug)]
pub struct Capsule {
    value: CapsuleValue,
    option: HashMap<String, String>,
}


impl From<Value> for Capsule {
    fn from(mut value: Value) -> Self {
        let mut option_map: HashMap<String, String> = HashMap::new();
        match value.as_object() {
            None => {
                return Self{
                    value: CapsuleValue::from(value),
                    option: option_map,
                };
            },
            Some(obj) => {
                if ! (obj.contains_key("__value__") && obj.contains_key("__option__")) {
                    return Self{
                        value: CapsuleValue::from(value),
                        option: option_map,
                    };
                }
                    
            }
        }
        for (k, v) in value.as_object().unwrap().get("__option__").unwrap().as_object().unwrap().iter() {
            option_map.insert(k.clone(), v.as_str().unwrap().to_owned());
        }
        return Self {
            value: CapsuleValue::from(value.as_object_mut().unwrap().remove("__value__").unwrap()),
            option: option_map
        }
    }
}

impl TryFrom<Capsule> for Value {
    type Error = anyhow::Error;
    
    fn try_from(value: Capsule) -> Result<Self, Self::Error> {
        //match value.as_value() {
        //    Some(v) => {}, //Ok(v),
        //    None => return Err(anyhow::Error::from(JuizError::CapsuleIsNotValueTypeError{}))
        //};
        match value.value {
            CapsuleValue::Value(mut _v) => {
                Ok(jvalue!({
                    "__value__": _v,
                    "__option__": jvalue!(value.option)
                }))
            }
            _ => Err(anyhow::Error::from(JuizError::CapsuleIsNotValueTypeError{}))
        }
    }
}

impl From<Mat> for Capsule {
    fn from(mat_value: Mat) -> Self {
        Self{
            value: CapsuleValue::from(mat_value),
            option: HashMap::new(),
        }
    }
}

/*
impl TryInto<Mat> for Capsule {
    type Error = anyhow::Error;
    
    fn try_into(self) -> Result<Mat, Self::Error> {
        //match self.to_mat() {
        //    Some(v) => Ok(v),
        //    None => Err(anyhow::Error::from(JuizError::CapsuleIsNotValueTypeError{}))
        //}
        todo!()
    }
}*/

impl Capsule {

    pub fn is_empty(&self) -> bool { self.value.is_empty() }

    pub fn is_value(&self) -> bool { self.value.is_value() }

    pub fn as_value(&self) -> Option<&Value> { self.value.as_value() }

    pub fn as_value_mut(&mut self) -> Option<&mut Value> { self.value.as_value_mut() }

    pub fn to_value(self) -> Option<Value> { self.value.to_value() }

    pub fn is_mat(&self) -> bool { self.value.is_mat() }

    pub fn as_mat(&self) -> Option<&opencv::core::Mat> { self.value.as_mat() }

    //pub fn to_mat(&self) -> Option<opencv::core::Mat> { self.value.to_mat() }

    pub fn set_option(&mut self, key: &str, value: &str) -> &mut Self {
        self.option.insert(key.to_owned(), value.to_owned());
        self
    }

    pub fn get_option(&self, key: &str) -> Option<&String> {
        self.option.get(&key.to_owned())
    }

    pub fn get_options(&self) -> &HashMap<String, String> {
        &self.option
    }
    
    pub fn empty() -> Capsule {
        Self {
            value: CapsuleValue::Empty(()),
            option: HashMap::new(),
        }
    }

    pub fn set_function_name(&mut self, name: &str) -> () {
        self.set_option("function_name", name);
        //self
    }

    pub(crate) fn _replace(&mut self, capsule: Capsule) -> () {
        self.value = capsule.value;
        self.option = capsule.option;
    }

    
    pub(crate) fn replace_value(&mut self, value: CapsuleValue) -> () {
        self.value = value;
    }

    pub(crate) fn take_value(&mut self) -> CapsuleValue {
        let mut emp = CapsuleValue::Empty(());
        swap(&mut self.value, &mut emp);
        emp
    }
}



#[no_mangle]
pub unsafe extern "C" fn arc_capsule_is_value(capsule: *mut Arc<Mutex<Capsule>>) -> bool {
    let c = capsule.as_ref().unwrap();
    let is_value = c.lock().and_then(|cap| {Ok(cap.is_value())} );
    is_value.unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn arc_capsule_is_int(capsule: *mut Arc<Mutex<Capsule>>) -> bool {
    if arc_capsule_is_value(capsule) {
        let c = capsule.as_ref().unwrap();    
        return c.lock().and_then(|cap| {
            let v = cap.as_value().unwrap();
            Ok(v.is_i64())
        }).unwrap();
    }
    false    
}




/**
#[no_mangle]
pub unsafe extern "C" fn capsule_get_int(capsule: *mut Capsule, v: *mut i64) -> i64 {
    capsule_ptr.as_ref().unwrap().lock_as_value(|val| {
        val.as_i64().and_then(|iv| { *v = iv; Some(0)}).or(Some(-1)).unwrap()
    } ).or::<i64>(Ok(-2)).unwrap()
}
    */


#[no_mangle]
pub unsafe extern "C" fn arc_capsule_is_float(capsule: *mut Arc<Mutex<Capsule>>) -> bool {
    if arc_capsule_is_value(capsule) {
        let c = capsule.as_ref().unwrap();    
        return c.lock().and_then(|cap| {
            let v = cap.as_value().unwrap();
            Ok(v.is_f64())
        }).unwrap();
    }
    false    
}

#[no_mangle]
pub unsafe extern "C" fn arc_capsule_is_bool(capsule: *mut Arc<Mutex<Capsule>>) -> bool {
    if arc_capsule_is_value(capsule) {
        let c = capsule.as_ref().unwrap();    
        return c.lock().and_then(|cap| {
            let v = cap.as_value().unwrap();
            Ok(v.is_boolean())
        }).unwrap();
    }
    false    
}

#[no_mangle]
pub unsafe extern "C" fn arc_capsule_is_array(capsule: *mut Arc<Mutex<Capsule>>) -> bool {
    if arc_capsule_is_value(capsule) {
        let c = capsule.as_ref().unwrap();    
        return c.lock().and_then(|cap| {
            let v = cap.as_value().unwrap();
            Ok(v.is_array())
        }).unwrap();
    }
    false    
}

#[no_mangle]
pub unsafe extern "C" fn arc_capsule_is_object(capsule: *mut Arc<Mutex<Capsule>>) -> bool {
    if arc_capsule_is_value(capsule) {
        let c = capsule.as_ref().unwrap();    
        return c.lock().and_then(|cap| {
            Ok(cap.as_value().unwrap().is_object())
        }).unwrap();
    }
    false    
}

#[no_mangle]
pub unsafe extern "C" fn arc_capsule_is_string(capsule: *mut Arc<Mutex<Capsule>>) -> bool {
    if arc_capsule_is_value(capsule) {
        let c = capsule.as_ref().unwrap();    
        return c.lock().and_then(|cap| {
            Ok(cap.as_value().unwrap().is_string())
        }).unwrap();
    }
    false    
}

#[no_mangle]
pub unsafe extern "C" fn arc_capsule_is_null(capsule: *mut Arc<Mutex<Capsule>>) -> bool {
    if arc_capsule_is_value(capsule) {
        let c = capsule.as_ref().unwrap();    
        return c.lock().and_then(|cap| {
            Ok(cap.as_value().unwrap().is_null())
        }).unwrap();
    }
    false    
}

#[no_mangle]
pub unsafe extern "C" fn arc_capsule_is_uint(capsule: *mut Arc<Mutex<Capsule>>) -> bool {
    if arc_capsule_is_value(capsule) {
        let c = capsule.as_ref().unwrap();    
        return c.lock().and_then(|cap| {
            Ok(cap.as_value().unwrap().is_u64())
        }).unwrap();
    }
    false    
}

#[no_mangle]
pub unsafe extern "C" fn capsule_is_value(capsule: *mut Capsule) -> bool {
    let c = capsule.as_ref().unwrap();
    c.is_value()
}

#[no_mangle]
pub unsafe extern "C" fn capsule_is_int(capsule: *mut Capsule) -> bool {
    if capsule_is_value(capsule) {
        let c = capsule.as_ref().unwrap();
        c.as_value().unwrap().is_i64()
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn capsule_get_int(capsule: *mut Capsule, v: *mut i64) -> bool {
    if capsule_is_value(capsule) {
        let val = capsule.as_ref().unwrap().as_value().unwrap();
        if !val.is_i64() {
            false
        } else {
            *v = val.as_i64().unwrap();
            true
        }
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn capsule_set_int(capsule: *mut Capsule, v: i64) -> bool {
    let cap = capsule.as_mut().unwrap();
    cap.replace_value(jvalue!(v).into());
    return true;
}