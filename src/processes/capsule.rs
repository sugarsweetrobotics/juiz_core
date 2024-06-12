use std::{collections::HashMap, mem::swap, option::Iter, sync::{Arc, Mutex}};

use opencv::core::Mat;

use crate::{jvalue, utils::get_hashmap, JuizError, JuizResult, Value};

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

    pub fn to_value(&self) -> Option<Value> {
        match self {
            Self::Value(v) => Some(v.clone()),
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
    fn from(value: Value) -> Self {
        Self{
            value: CapsuleValue::from(value),
            option: HashMap::new(),
        }
    }
}


impl TryFrom<Capsule> for Value {
    type Error = anyhow::Error;
    
    fn try_from(value: Capsule) -> Result<Self, Self::Error> {
        match value.to_value() {
            Some(v) => Ok(v),
            None => Err(anyhow::Error::from(JuizError::CapsuleIsNotValueTypeError{}))
        }
    }
}

impl From<Mat> for Capsule {
    fn from(value: Mat) -> Self {
        Self{
            value: CapsuleValue::from(value),
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

    pub fn to_value(&self) -> Option<Value> { self.value.to_value() }

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

    
    fn replace_value(&mut self, value: CapsuleValue) -> () {
        self.value = value;
    }

    fn take_value(&mut self) -> CapsuleValue {
        let mut emp = CapsuleValue::Empty(());
        swap(&mut self.value, &mut emp);
        emp
    }
}

#[derive(Debug)]
pub struct CapsuleMap {
    map: HashMap<String, CapsulePtr>,
    param: HashMap<String, String>,
}

impl CapsuleMap {

    pub fn new() -> Self {
        CapsuleMap { map: HashMap::new(), param: HashMap::new() }
    }

    pub fn set_param(&mut self, key: &str, value: &str) -> &mut Self {
        self.param.insert(key.to_owned(), value.to_owned());
        self
    }

    pub fn get_param(&self, key: &str) -> Option<&String> {
        self.param.get(key)
    }

    pub fn get(&self, key: &str) -> JuizResult<CapsulePtr> {
        match self.map.get(key) {
            Some(v) => Ok(v.clone()),
            None => Err(anyhow::Error::from(JuizError::CapsuleMapDoesNotContainValueError{key: key.to_owned()}))
        }
    }

    pub fn get_params<'a>(&'a self) -> &'a HashMap<String, String> {
        &self.param
    }
    
    pub fn insert(&mut self, key: String, value: CapsulePtr) -> &Self {
        self.map.insert(key, value);
        self
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<String, CapsulePtr> {
        self.map.iter()
    }
}

impl TryFrom<Value> for CapsuleMap {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let mut c = CapsuleMap::new();
        for (k, v) in get_hashmap(&value)?.into_iter() {
            c.insert(k.to_owned(), (*v).clone().into());
        }
        Ok(c)
    }
}

impl From<Vec<(std::string::String, CapsulePtr)>> for CapsuleMap {
    fn from(value: Vec<(std::string::String, CapsulePtr)>) -> Self {
        let mut map = CapsuleMap::new();
        for v in value {
            map.insert(v.0, v.1);
        }
        return map
    }
}

impl From<CapsuleMap> for Value {
    fn from(value: CapsuleMap) -> Self {
        let mut v = jvalue!({});
        let map = v.as_object_mut().unwrap();
        for (k, v) in value.map.into_iter() {
            let _ = v.lock_as_value(|vv| -> () { map.insert(k.clone(), vv.clone()); });
            //map.insert(k, Value::try_from(v.lock().unwrap().clone()).unwrap());
        }
        return v;
    }
}

impl From<Vec<(&str, Value)>> for CapsuleMap {
    fn from(value: Vec<(&str, Value)>) -> Self {
        let mut c = CapsuleMap::new();
        for (k, v) in value {
            c.insert(k.to_owned(), v.into());
        }
        c
    }
}


impl From<&[(&str, Value)]> for CapsuleMap {
    fn from(value: &[(&str, Value)]) -> Self {
        let mut c = CapsuleMap::new();
        for (k, v) in value {
            c.insert((*k).to_owned(), (*v).clone().into());
        }
        c
    }
}


//pub type CapsulePtr = Arc<Mutex<Capsule>>;

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

/*
pub fn unwrap_arc_capsule(arc: CapsulePtr) -> JuizResult<Capsule> {
    let lock = Arc::try_unwrap(arc.value).or_else(|e| {Err(anyhow::Error::from(JuizError::MutexLockFailedError { error: "Arc unwrapping error when try_unwrap(). Error is ".to_owned() +  }))})?;
    lock.into_inner().or_else(|e| {Err(anyhow::Error::from(JuizError::MutexLockFailedError { error: "Arc unwrapping error{e.to_string()}".to_owned()}))})
}
*/
        