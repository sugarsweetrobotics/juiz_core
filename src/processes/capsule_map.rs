use std::collections::HashMap;


use serde_json::Map;

use crate::{jvalue, utils::get_hashmap, CapsulePtr, JuizError, JuizResult, Value};


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

    pub fn get_int(&self, key: &str) -> JuizResult<i64> {
        Ok(self.get(key)?.lock_as_value(|value| { value.as_i64().unwrap() })?)
    }

    pub fn get_str(&self, key: &str) -> JuizResult<String> {
        Ok(self.get(key)?.lock_as_value(|value| { value.as_str().unwrap().to_owned() })?)
    }

    pub fn get_str_then<T, F>(&self, key: &str, f: F) -> JuizResult<T> where F: Fn(&str) -> T {
        Ok(self.get(key)?.lock_as_str(|value| { f(value) })?)
    }

    pub fn get_map_then<T, F>(&self, key: &str, f: F) -> JuizResult<T> where F: Fn(&Map<String, Value>) -> T {
        Ok(self.get(key)?.lock_as_map(|value| { f(value) })?)
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
