use std::{collections::HashMap, sync::{Arc, Mutex}};

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

    pub fn to_mat(&self) -> Option<Mat> {
        match self {
            Self::Mat(v) => Some(v.clone()),
            _ => None
        }
    }
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

impl TryFrom<Capsule> for Mat {
    fn try_from(value: Capsule) -> Result<Self, Self::Error> {
        match value.to_mat() {
            Some(v) => Ok(v),
            None => Err(anyhow::Error::from(JuizError::CapsuleIsNotValueTypeError{}))
        }
    }
    
    type Error = anyhow::Error;
}


impl Capsule {

    pub fn is_empty(&self) -> bool { self.value.is_empty() }

    pub fn is_value(&self) -> bool { self.value.is_value() }

    pub fn as_value(&self) -> Option<&Value> { self.value.as_value() }

    pub fn to_value(&self) -> Option<Value> { self.value.to_value() }

    pub fn is_mat(&self) -> bool { self.value.is_mat() }

    pub fn as_mat(&self) -> Option<&opencv::core::Mat> { self.value.as_mat() }

    pub fn to_mat(&self) -> Option<opencv::core::Mat> { self.value.to_mat() }

    pub fn set_option(&mut self, key: &str, value: &str) -> &mut Self {
        self.option.insert(key.to_owned(), value.to_owned());
        self
    }

    pub fn get_option(&self, key: &str) -> Option<&String> {
        self.option.get(&key.to_owned())
    }
    
    pub(crate) fn empty() -> Capsule {
        Self {
            value: CapsuleValue::Empty(()),
            option: HashMap::new(),
        }
    }

    pub fn set_function_name(&mut self, name: &str) -> () {
        self.set_option("function_name", name);
        //self
    }

    pub(crate) fn replace(&mut self, capsule: Capsule) -> () {
        self.value = capsule.value;
        self.option = capsule.option;
    }
}

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
}

impl TryFrom<Value> for CapsuleMap {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let mut c = CapsuleMap::new();
        for (k, v) in get_hashmap(&value)?.into_iter() {
            c.insert(k.to_owned(), Arc::new(Mutex::new((*v).clone().into())));
        }
        Ok(c)
    }
}

impl From<CapsuleMap> for Value {
    fn from(value: CapsuleMap) -> Self {
        let mut v = jvalue!({});
        let map = v.as_object_mut().unwrap();
        for (k, v) in value.map.into_iter() {
            map.insert(k, Value::try_from(v.lock().unwrap().clone()).unwrap());
        }
        return v;
    }
}

impl From<Vec<(&str, Value)>> for CapsuleMap {
    fn from(value: Vec<(&str, Value)>) -> Self {
        let mut c = CapsuleMap::new();
        for (k, v) in value {
            c.insert(k.to_owned(), Arc::new(Mutex::new(v.into())));
        }
        c
    }
}


impl From<&[(&str, Value)]> for CapsuleMap {
    fn from(value: &[(&str, Value)]) -> Self {
        let mut c = CapsuleMap::new();
        for (k, v) in value {
            c.insert((*k).to_owned(), Arc::new(Mutex::new((*v).clone().into())));
        }
        c
    }
}

pub type CapsulePtr = Arc<Mutex<Capsule>>;

pub fn capsule_to_value(capsule: CapsulePtr) -> JuizResult<Value> {
    match capsule.lock() {
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
    Arc::new(Mutex::new(value.into()))
}

pub fn unwrap_arc_capsule(arc: CapsulePtr) -> JuizResult<Capsule> {
    let lock = Arc::try_unwrap(arc).or_else(|_| {Err(anyhow::Error::from(JuizError::MutexLockFailedError { error: "Arc unwrapping error".to_owned() }))})?;
    lock.into_inner().or_else(|_| {Err(anyhow::Error::from(JuizError::MutexLockFailedError { error: "Arc unwrapping error".to_owned() }))})
}
        