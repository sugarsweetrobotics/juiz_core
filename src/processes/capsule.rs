use std::collections::HashMap;

use opencv::core::Mat;

use crate::{jvalue, utils::get_hashmap, JuizError, Value};

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

    pub fn set_function_name(mut self, name: &str) -> Self {
        self.set_option("function_name", name);
        self
    }
}

pub struct CapsuleMap {
    map: HashMap<String, Capsule>,
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

    pub fn get(&self, key: &str) -> Option<&Capsule> {
        self.map.get(key)
    }

    pub fn get_params<'a>(&'a self) -> &'a HashMap<String, String> {
        &self.param
    }
    
    pub fn insert(&mut self, key: String, value: Capsule) -> &Self {
        self.map.insert(key, value);
        self
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

impl From<CapsuleMap> for Value {
    fn from(value: CapsuleMap) -> Self {
        let mut v = jvalue!({});
        let map = v.as_object_mut().unwrap();
        for (k, v) in value.map.into_iter() {
            map.insert(k, v.try_into().unwrap());
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