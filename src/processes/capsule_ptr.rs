
//pub type CapsulePtr = Arc<Mutex<Capsule>>;

use std::{collections::HashMap, ffi::CStr, sync::{Arc, Mutex}};

use opencv::core::Mat;
use serde_json::Map;

use crate::{jvalue, Capsule, JuizError, JuizResult, Value};

#[repr(C)]
#[derive(Debug)]
pub struct CapsulePtr {
    value: Arc<Mutex<Capsule>>,
}

impl CapsulePtr {

    pub fn new() -> Self {
        Self{value: Arc::new(Mutex::new(Capsule::empty()))}
    }

    pub(crate) fn value_mut(&mut self) -> &mut Arc<Mutex<Capsule>> {
        &mut self.value
    }

    pub fn replace(&self, capsule: CapsulePtr) -> () {
        match self.value.lock() {
            Ok(mut c) => {
                c.replace_value(capsule.value.lock().unwrap().take_value());
            },
            Err(_) => todo!(),
        }
    }

    pub fn replace_with_value(&mut self, value: Value) -> () {
        self.value = Arc::new(Mutex::new(value.into()));
    }

    pub fn is_empty(&self) -> JuizResult<bool> {
        match self.value.lock() {
            Ok(c) => {
                Ok(c.is_empty())
            },
            Err(_e) => Err(anyhow::Error::from(JuizError::MutexLockFailedError { error: "CapsulePtr.is_empty() lock error.".to_owned() })),
        }
    }

    pub fn is_mat(&self) -> JuizResult<bool> {
        match self.value.lock() {
            Ok(c) => {
                Ok(c.is_mat())
            },
            Err(_e) => Err(anyhow::Error::from(JuizError::MutexLockFailedError { error: "CapsulePtr.is_empty() lock error.".to_owned() })),
        }
    }

    pub fn is_value(&self) -> JuizResult<bool> {
        match self.value.lock() {
            Ok(c) => {
                Ok(c.is_value())
            },
            Err(_e) => Err(anyhow::Error::from(JuizError::MutexLockFailedError { error: "CapsulePtr.is_empty() lock error.".to_owned() })),
        }
    }

    pub fn set_option(&mut self, key: &str, value: &str) -> JuizResult<&mut Self> {
        match self.value.lock() {
            Ok(mut c) => {
                c.set_option(key, value);
            },
            Err(_e) => {
                return Err(anyhow::Error::from(JuizError::MutexLockFailedError { error: "CapsulePtr.set_option() lock error.".to_owned() }));
            }
        }
        Ok(self)
    }

    pub fn get_option(&self, key: &str) -> JuizResult<String> {
        match self.value.lock() {
            Ok(c) => {
                match c.get_option(key) {
                    Some(s) => Ok(s.clone()),
                    None => Err(anyhow::Error::from(JuizError::CapsuleDoesNotIncludeParamError { name: key.to_owned() })),
                }
            },
            Err(_e) => Err(anyhow::Error::from(JuizError::MutexLockFailedError { error: "CapsulePtr.get_potion() lock error.".to_owned() })),
        }
    }
    
    pub fn lock_as_value_and_opt<T, F>(&self, func: F) -> JuizResult<T> where F: FnOnce(&Value, &HashMap<String, String>) -> T{
        match self.value.lock() {
            Ok(c) => {
                match c.as_value() {
                    Some(v) => Ok(func(v, c.get_options())),
                    None => todo!(),
                }
            }
            Err(_e) => Err(anyhow::Error::from(JuizError::MutexLockFailedError { error: "CapsulePtr.lock_as_value() lock error.".to_owned() })),
        }
    }

    pub fn extract_value(self) -> JuizResult<Value> {
        match Arc::try_unwrap(self.value) {
            Ok(v) => {
                match v.into_inner() {
                    Ok(vv) => {
                        return Ok(vv.to_value().unwrap())
                    },
                    Err(v) => {
                        return Ok(v.get_ref().as_value().unwrap().clone())
                    }
                }
            },
            Err(e) => {
                e.lock().and_then(|v| { Ok(v.as_value().unwrap().clone()) }).or_else(|_e| { Err(anyhow::Error::from(JuizError::MutexLockFailedError { error: "".to_owned() })) })
            }
        }
    }

    pub fn lock_as_value<T, F>(&self, func: F) -> JuizResult<T> where F: FnOnce(&Value) -> T{
        match self.value.lock() {
            Ok(c) => {
                match c.as_value() {
                    Some(v) => Ok(func(v)),
                    None => todo!(),
                }
            }
            Err(_e) => Err(anyhow::Error::from(JuizError::MutexLockFailedError { error: "CapsulePtr.lock_as_value() lock error.".to_owned() })),
        }
    }

    pub fn lock_as_str<T, F>(&self, func: F) -> JuizResult<T> where F: FnOnce(&str) -> T {
        match self.value.lock() {
            Ok(c) => {
                match c.as_value() {
                    Some(v) => {
                        match v.as_str() {
                            Some(s) => Ok(func(s)),
                            None => Err(anyhow::Error::from(JuizError::ValueTypeError {message: "CapsulePtr.lock_as_str() value is not str type.".to_owned() })),
                        }
                    },
                    None => Err(anyhow::Error::from(JuizError::ValueTypeError { message: "CapsulePtr.lock_as_value() data is not Value type.".to_owned() })),
                }
            }
            Err(_e) => Err(anyhow::Error::from(JuizError::MutexLockFailedError { error: "CapsulePtr.lock_as_str() lock error.".to_owned() })),
        }
    }

    pub fn lock_as_map<T, F>(&self, func: F) -> JuizResult<T> where F: FnOnce(&Map<String, Value>) -> T {
        match self.value.lock() {
            Ok(c) => {
                match c.as_value() {
                    Some(v) => {
                        match v.as_object() {
                            Some(s) => Ok(func(s)),
                            None => Err(anyhow::Error::from(JuizError::ValueTypeError {message: "CapsulePtr.lock_as_map() value is not Map type.".to_owned() })),
                        }
                    },
                    None => Err(anyhow::Error::from(JuizError::ValueTypeError { message: "CapsulePtr.lock_as_map() data is not Value type.".to_owned() })),
                }
            }
            Err(_e) => Err(anyhow::Error::from(JuizError::MutexLockFailedError { error: "CapsulePtr.lock_as_map() lock error.".to_owned() })),
        }
    }

    pub fn lock_modify_as_value<F, T>(&self, func: F) -> JuizResult<T> where F: FnOnce(&mut Value) -> T {
        match self.value.lock() {
            Ok(mut c) => {
                match c.as_value_mut() {
                    Some(v) => Ok(func(v)),
                    None => todo!(),
                }
            }
            Err(_e) => Err(anyhow::Error::from(JuizError::MutexLockFailedError { error: "CapsulePtr.lock_as_value() lock error.".to_owned() })),
        }
    }

    pub fn lock_as_mat<T, F>(&self, func: F) -> JuizResult<T> where F: FnOnce(&Mat) -> T{
        match self.value.lock() {
            Ok(c) => {
                match c.as_mat() {
                    Some(v) => Ok(func(v)),
                    None => todo!(),
                }
            }
            Err(_e) => Err(anyhow::Error::from(JuizError::MutexLockFailedError { error: "CapsulePtr.lock_as_value() lock error.".to_owned() })),
        }
    }
    
    pub(crate) fn set_function_name(&mut self, name: &str) -> JuizResult<()> {
        self.set_option("function_name", name)?;
        Ok(())
    }

    pub(crate) fn get_function_name(&self) -> JuizResult<String> {
        self.get_option("function_name")
    }

    pub(crate) fn set_class_name(&mut self, name: &str) -> JuizResult<()> {
        self.set_option("class_name", name)?;
        Ok(())
    }

    #[allow(unused)]
    pub(crate) fn get_class_name(&self) -> JuizResult<String> {
        self.get_option("class_name")
    }

}

impl From<Value> for CapsulePtr {
    fn from(value: Value) -> Self {
        Self{value: Arc::new(Mutex::new(value.into()))}
    }
}

impl From<Mat> for CapsulePtr {
    fn from(value: Mat) -> Self {
        Self{value: Arc::new(Mutex::new(value.into()))}
    }
}

impl From<Capsule> for CapsulePtr {
    fn from(value: Capsule) -> Self {
        Self{value: Arc::new(Mutex::new(value))}
    }
}

impl TryInto<Value> for CapsulePtr {
    fn try_into(self) -> Result<Value, Self::Error> {
        self.lock_as_value_and_opt(|v, opt| {
            jvalue!({
                "__value__": v,
                "__option__": jvalue!(opt)
            })
        } )
    }
    type Error = anyhow::Error;
}

impl Clone for CapsulePtr {
    fn clone(&self) -> Self {
        Self { value: self.value.clone() }
    }
}


pub fn capsule_to_value(capsule: CapsulePtr) -> JuizResult<Value> {
    log::trace!("capsule_to_value(capsule: {capsule:?}) called");
    capsule.lock_as_value_and_opt(|v, opt| {
        jvalue!({
            "__value__": v,
            "__option__": jvalue!(opt)
        })
    } )
}

pub fn value_to_capsule(value: Value) -> CapsulePtr {
    value.into()
}


#[no_mangle]
pub unsafe extern "C" fn capsule_ptr_get_value(capsule_ptr: *mut CapsulePtr) -> *mut Arc<Mutex<Capsule>> {
    capsule_ptr.as_mut().unwrap().value_mut()
}


#[no_mangle]
pub unsafe extern "C" fn capsule_ptr_get_int(capsule_ptr: *mut CapsulePtr, v: *mut i64) -> i64 {
    capsule_ptr.as_ref().unwrap().lock_as_value(|val| {
        val.as_i64().and_then(|iv| { *v = iv; Some(0)}).or(Some(-1)).unwrap()
    } ).or::<i64>(Ok(-2)).unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn capsule_ptr_get_uint(capsule_ptr: *mut CapsulePtr, v: *mut u64) -> i64 {
    capsule_ptr.as_ref().unwrap().lock_as_value(|val| {
        val.as_u64().and_then(|iv| { *v = iv; Some(0)}).or(Some(-1)).unwrap()
    } ).or::<i64>(Ok(-2)).unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn capsule_ptr_get_float(capsule_ptr: *mut CapsulePtr, v: *mut f64) -> i64 {
    capsule_ptr.as_ref().unwrap().lock_as_value(|val| {
        val.as_f64().and_then(|iv| { *v = iv; Some(0)}).or(Some(-1)).unwrap()
    } ).or::<i64>(Ok(-2)).unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn capsule_ptr_get_bool(capsule_ptr: *mut CapsulePtr, v: *mut i64) -> i64 {
    capsule_ptr.as_ref().unwrap().lock_as_value(|val| {
        val.as_bool().and_then(|iv| { if iv { *v = 1 } else { *v = 0 }; Some(0)}).or(Some(-1)).unwrap()
    } ).or::<i64>(Ok(-2)).unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn capsule_ptr_get_string(capsule_ptr: *mut CapsulePtr, v: *mut *const u8) -> i64 {
    capsule_ptr.as_ref().unwrap().lock_as_value(|val| {
        val.as_str().and_then(|iv| { *v = iv.as_ptr(); Some(0)}).or(Some(-1)).unwrap()
    } ).or::<i64>(Ok(-2)).unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn capsule_ptr_lock_as_value(capsule_ptr: *mut CapsulePtr, callback: extern fn(*mut Value) -> ()) -> () {
    let _ = capsule_ptr.as_mut().unwrap().lock_modify_as_value(|v|->() {
        callback(v)
    } );
}

#[no_mangle]
pub unsafe extern "C" fn capsule_ptr_lock_as_value_with_arg(capsule_ptr: *mut CapsulePtr, callback: extern fn(*mut std::ffi::c_void, *mut Value) -> i64, arg: *mut std::ffi::c_void) -> i64 {
    capsule_ptr.as_mut().unwrap().lock_modify_as_value(|v|->i64 {
        callback(arg, v)
    } ).or::<i64>(Ok(-1)).unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn capsule_ptr_set_int(capsule_ptr: *mut CapsulePtr, v: i64) -> i64 {
    let c = capsule_ptr.as_mut().unwrap();
    log::trace!("capsule_ptr_set_int({c:?}, {v:?}) called");
    if !c.is_value().unwrap() {
        c.replace_with_value(jvalue!(v));
        return 0;
    }
    return -1;
}

#[no_mangle]
pub unsafe extern "C" fn capsule_ptr_set_uint(capsule_ptr: *mut CapsulePtr, v: u64) -> i64 {
    let c = capsule_ptr.as_mut().unwrap();
    if !c.is_value().unwrap() {
        c.replace_with_value(jvalue!(v));
        return 0;
    }
    return -1;
}

#[no_mangle]
pub unsafe extern "C" fn capsule_ptr_set_float(capsule_ptr: *mut CapsulePtr, v: f64) -> i64 {
    let c = capsule_ptr.as_mut().unwrap();
    if !c.is_value().unwrap() {
        c.replace_with_value(jvalue!(v));
        return 0;
    }
    return -1;
}

#[no_mangle]
pub unsafe extern "C" fn capsule_ptr_set_bool(capsule_ptr: *mut CapsulePtr, v: i64) -> i64 {
    let c = capsule_ptr.as_mut().unwrap();
    if !c.is_value().unwrap() {
        c.replace_with_value(jvalue!(v != 0));
        return 0;
    }
    return -1;
}

#[no_mangle]
pub unsafe extern "C" fn capsule_ptr_set_string(capsule_ptr: *mut CapsulePtr, v: *const i8) -> i64 {
    let c = capsule_ptr.as_mut().unwrap();
    if !c.is_value().unwrap() {
        c.replace_with_value(jvalue!(CStr::from_ptr(v).to_str().unwrap()));
        return 0;
    }
    return -1;
}

#[no_mangle]
pub unsafe extern "C" fn capsule_ptr_set_empty_object(capsule_ptr: *mut CapsulePtr) -> i64 {
    let c = capsule_ptr.as_mut().unwrap();
    c.replace_with_value(jvalue!({}));
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn capsule_ptr_set_empty_array(capsule_ptr: *mut CapsulePtr) -> i64 {
    let c = capsule_ptr.as_mut().unwrap();
    c.replace_with_value(jvalue!([]));
    return 0;
}