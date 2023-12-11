use std::sync::{Arc, Mutex};

use crate::{jvalue, BrokerProxy, utils::juiz_lock, Value, JuizResult, JuizError, Identifier};




pub struct CRUDBroker {
    core_broker: Arc<Mutex<dyn BrokerProxy>>,
}

fn resource_name_to_cls_and_id<'a>(resource_name: &'a str, params: &Vec<String>) -> JuizResult<(&'a str, Identifier)> {
    let mut split = resource_name.split('/');
    let class_name = split.next().ok_or( anyhow::Error::from(JuizError::CRUDBrokerGivenResourseNameHasNoClassNameError{resource_name: resource_name.to_string()} ))?;
    Ok((class_name, "".to_string()))
}

impl CRUDBroker {


    pub fn new(core_broker: Arc<Mutex<dyn BrokerProxy>>) -> JuizResult<Arc<Mutex<CRUDBroker>>> {
        Ok(Arc::new(Mutex::new(CRUDBroker{core_broker})))
    }

    pub fn create(&self, resource_name: &str, params: Vec<String>, value: Value) -> JuizResult<Value> {
        todo!()
    }

    pub fn read(&self, resource_name: &str) -> JuizResult<Value> {
        match resource_name {
            "system/profile_full" => {
                juiz_lock(&self.core_broker)?.profile_full()
            },
            _ => {
                Ok(jvalue!({}))
            }
        }
    }  

    pub fn read_with_param(&self, resource_name: &str, params: Vec<String>) -> JuizResult<Value> {
        let (class_name, id) = resource_name_to_cls_and_id(resource_name, &params)?;
        match class_name {
            "core_broker" => {
                juiz_lock(&self.core_broker)?.profile_full()
            },
            _ => {
                Ok(jvalue!({}))
            }
        }
    }   

    pub fn update(&self, resource_name: &str, params: Vec<String>, value: Value) -> JuizResult<Value> {
        todo!()
    }

    pub fn delete(&self, resource_name: &str, params: Vec<String>) -> JuizResult<Value> {
        todo!()
    }
}