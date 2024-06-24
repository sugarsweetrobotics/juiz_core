
//pub type CapsulePtr = Arc<Mutex<Capsule>>;

use std::sync::{Arc, Mutex};

use opencv::core::Mat;
use serde_json::Map;

use crate::{Capsule, JuizError, JuizResult, Value};

#[derive(Debug)]
pub struct CapsulePtr {
    value: Arc<Mutex<Capsule>>,
}

impl CapsulePtr {

    pub fn new() -> Self {
        Self{value: Arc::new(Mutex::new(Capsule::empty()))}
    }

    pub fn replace(&self, capsule: CapsulePtr) -> () {
        match self.value.lock() {
            Ok(mut c) => {
                c.replace_value(capsule.value.lock().unwrap().take_value());
            },
            Err(_) => todo!(),
        }
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

    pub fn lock_modify_as_value<F>(&self, func: F) -> JuizResult<()> where F: FnOnce(&mut Value) -> () {
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

impl Clone for CapsulePtr {
    fn clone(&self) -> Self {
        Self { value: self.value.clone() }
    }
}

pub fn capsule_to_value(capsule: CapsulePtr) -> JuizResult<Value> {
    match capsule.value.lock() {
        Ok(v) => {
            match v.to_value() {
                Some(vv) => Ok(vv),
                None => Err(anyhow::Error::from(JuizError::CapsuleIsNotValueTypeError {  }))
            }
        }, 
        Err(_e) => Err(anyhow::Error::from(JuizError::MutexLockFailedError { error: "capsule_to_value()".to_owned() }))
    }
}

pub fn value_to_capsule(value: Value) -> CapsulePtr {
    value.into()
}