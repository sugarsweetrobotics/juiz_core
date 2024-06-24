use std::{collections::HashMap, mem::swap, sync::{Arc, Mutex}};

use opencv::core::Mat;

use crate::{JuizError, JuizResult, Value};
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

    
    pub(crate) fn replace_value(&mut self, value: CapsuleValue) -> () {
        self.value = value;
    }

    pub(crate) fn take_value(&mut self) -> CapsuleValue {
        let mut emp = CapsuleValue::Empty(());
        swap(&mut self.value, &mut emp);
        emp
    }
}


/*
pub fn unwrap_arc_capsule(arc: CapsulePtr) -> JuizResult<Capsule> {
    let lock = Arc::try_unwrap(arc.value).or_else(|e| {Err(anyhow::Error::from(JuizError::MutexLockFailedError { error: "Arc unwrapping error when try_unwrap(). Error is ".to_owned() +  }))})?;
    lock.into_inner().or_else(|e| {Err(anyhow::Error::from(JuizError::MutexLockFailedError { error: "Arc unwrapping error{e.to_string()}".to_owned()}))})
}
*/
        