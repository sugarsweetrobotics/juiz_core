use std::{sync::{Arc, Mutex}, collections::HashMap};

use anyhow::Context;

use crate::{jvalue, utils::juiz_lock, Value, JuizResult, JuizError, Identifier};
use crate::brokers::BrokerProxy;

pub struct CRUDBroker {
    core_broker: Arc<Mutex<dyn BrokerProxy>>,
}

fn _resource_name_to_cls_and_id<'a>(resource_name: &'a str, _params: &Vec<String>) -> JuizResult<(&'a str, Identifier)> {
    let mut split = resource_name.split('/');
    let class_name = split.next().ok_or( anyhow::Error::from(JuizError::CRUDBrokerGivenResourseNameHasNoClassNameError{resource_name: resource_name.to_string()} ))?;
    Ok((class_name, "".to_string()))
}

fn params_get(map: HashMap<String, String>, key: &str) -> JuizResult<String> {
    match map.get(key) {
        None => Err(anyhow::Error::from(JuizError::CRUDBrokerCanNotParameterFunctionError { key_name: key.to_string() })),
        Some(v) => Ok(v.clone())
    }
}
impl CRUDBroker {


    pub fn new(core_broker: Arc<Mutex<dyn BrokerProxy>>) -> JuizResult<Arc<Mutex<CRUDBroker>>> {
        Ok(Arc::new(Mutex::new(CRUDBroker{core_broker})))
    }

    pub fn create_class(&self, class_name: &str, function_name: &str, value: Value, params: HashMap<String, String>) -> JuizResult<Value> {
        log::trace!("CRUDBroker::create_class() called");
        let cb = juiz_lock(&self.core_broker)?;
        match class_name {
            "system" => {
                match function_name {
                    _ => {
                        return Err(anyhow::Error::from(JuizError::CRUDBrokerCanNotFindFunctionError{class_name:class_name.to_string(), function_name: function_name.to_string()}));
                    }
                }
            },
            "process" => {
                let id = params_get(params, "identifier")?;
                match function_name {
                    "call" => return cb.process_call(&id, value),
                    _ => {
                        return Err(anyhow::Error::from(JuizError::CRUDBrokerCanNotFindFunctionError{class_name:class_name.to_string(), function_name: function_name.to_string()}));
                    }
                }
            },
            _ => {
                Ok(jvalue!({}))
            }
        }
    }

    pub fn read_class(&self, class_name: &str, function_name: &str, params: HashMap<String, String>) -> JuizResult<Value> {
        log::trace!("CRUDBroker::read_class({class_name}, {function_name}, {params:?}) called");
        let cb = juiz_lock(&self.core_broker)?;
        match class_name {
            "system" => {
                match function_name {
                    "profile_full" => return cb.system_profile_full(),
                    _ => {
                        return Err(anyhow::Error::from(JuizError::CRUDBrokerCanNotFindFunctionError{class_name:class_name.to_string(), function_name: function_name.to_string()}));
                    }
                }
            },
            "broker" => {
                match function_name {
                    "profile_full" => {
                        let id = params_get(params, "identifier").context("CRUDBroker.read_class()")?;
                        return cb.broker_profile_full(&id)
                    },
                    "list" => return cb.broker_list(),
                    _ => {
                        return Err(anyhow::Error::from(JuizError::CRUDBrokerCanNotFindFunctionError{class_name:class_name.to_string(), function_name: function_name.to_string()}));
                    }
                }
            },
            "process" => {
                match function_name {
                    "profile_full" => {
                        let id = params_get(params, "identifier")?;
                        return cb.process_profile_full(&id)
                    },
                    "list" => return cb.process_list(),
                    _ => {
                        return Err(anyhow::Error::from(JuizError::CRUDBrokerCanNotFindFunctionError{class_name:class_name.to_string(), function_name: function_name.to_string()}));
                    }
                }
            },
            "container" => {
                
                match function_name {
                    "profile_full" => return cb.container_profile_full(&params_get(params, "identifier")?),
                    "list" => return cb.container_list(),
                    _ => {
                        return Err(anyhow::Error::from(JuizError::CRUDBrokerCanNotFindFunctionError{class_name:class_name.to_string(), function_name: function_name.to_string()}));
                    }
                }
            },
            "container_process" => {
                match function_name {
                    "profile_full" => return cb.container_process_profile_full(&params_get(params, "identifier")?),
                    "list" => return cb.container_process_list(),
                    _ => {
                        return Err(anyhow::Error::from(JuizError::CRUDBrokerCanNotFindFunctionError{class_name:class_name.to_string(), function_name: function_name.to_string()}));
                    }
                }
            },
            "connection" => {
                match function_name {
                    "profile_full" => {
                        let id = params_get(params, "identifier")?;
                        return cb.connection_profile_full(&id)
                    },
                    "list" => return cb.connection_list(),
                    _ => {
                        return Err(anyhow::Error::from(JuizError::CRUDBrokerCanNotFindFunctionError{class_name:class_name.to_string(), function_name: function_name.to_string()}));
                    }
                }
            },
            "execution_context" => {
                match function_name {
                    "profile_full" => return cb.ec_profile_full(&params_get(params, "identifier")?),
                    "list" => return cb.ec_list(),
                    _ => {
                        return Err(anyhow::Error::from(JuizError::CRUDBrokerCanNotFindFunctionError{class_name:class_name.to_string(), function_name: function_name.to_string()}));
                    }
                }
            },
            _ => {
                Ok(jvalue!({}))
            }
        }
    }

    pub fn update_class(&self, class_name: &str, function_name: &str, value: Value, params: HashMap<String, String>) -> JuizResult<Value> {
        log::trace!("CRUDBroker::update_class() called");
        let cb = juiz_lock(&self.core_broker)?;
        match class_name {
            "system" => {
                match function_name {
                    _ => {
                        return Err(anyhow::Error::from(JuizError::CRUDBrokerCanNotFindFunctionError{class_name:class_name.to_string(), function_name: function_name.to_string()}));
                    }
                }
            },
            "process" => {
                let id = params_get(params, "identifier")?;
                match function_name {
                    "call" => return cb.process_call(&id, value),
                    "execute" => return cb.process_execute(&id),
                    _ => {
                        return Err(anyhow::Error::from(JuizError::CRUDBrokerCanNotFindFunctionError{class_name:class_name.to_string(), function_name: function_name.to_string()}));
                    },
                }
            },
            _ => {
                Ok(jvalue!({}))
            }
        }
    }

    pub fn delete_class(&self, class_name: &str, function_name: &str, params: HashMap<String, String>) -> JuizResult<Value> {
        log::trace!("CRUDBroker::read_class called");
        let _cb = juiz_lock(&self.core_broker)?;
        match class_name {
            "system" => {
                match function_name {
                    _ => {
                        return Err(anyhow::Error::from(JuizError::CRUDBrokerCanNotFindFunctionError{class_name:class_name.to_string(), function_name: function_name.to_string()}));
                    }
                }
            },
            "process" => {
                
                let _id = params_get(params, "identifier")?;
                match function_name {
                    _ => {
                        return Err(anyhow::Error::from(JuizError::CRUDBrokerCanNotFindFunctionError{class_name:class_name.to_string(), function_name: function_name.to_string()}));
                    }
                }
            },
            "container" => {
                
                let _id = params_get(params, "identifier")?;
                match function_name {
                    _ => {
                        return Err(anyhow::Error::from(JuizError::CRUDBrokerCanNotFindFunctionError{class_name:class_name.to_string(), function_name: function_name.to_string()}));
                    }
                }
            },
            "container_process" => {
                let _id = params_get(params, "identifier")?;
                match function_name {
                    _ => {
                        return Err(anyhow::Error::from(JuizError::CRUDBrokerCanNotFindFunctionError{class_name:class_name.to_string(), function_name: function_name.to_string()}));
                    }
                }
            },
            _ => {
                Ok(jvalue!({}))
            }
        }
    }
}



pub fn create_class(crud_broker: &Arc<Mutex<CRUDBroker>>, class_name: &str, function_name: &str, args: Value, params: HashMap<String,String>) -> JuizResult<Value> {
    juiz_lock(crud_broker)?.create_class(class_name, function_name, args, params)
}

pub fn read_class(crud_broker: &Arc<Mutex<CRUDBroker>>, class_name: &str, function_name: &str, params: HashMap<String,String>) -> JuizResult<Value> {
    juiz_lock(crud_broker)?.read_class(class_name, function_name, params)
}

pub fn delete_class(crud_broker: &Arc<Mutex<CRUDBroker>>, class_name: &str, function_name: &str, params: HashMap<String,String>) -> JuizResult<Value> {
    juiz_lock(crud_broker)?.delete_class(class_name, function_name, params)
}

pub fn update_class(crud_broker: &Arc<Mutex<CRUDBroker>>, class_name: &str, function_name: &str, arg: Value, params: HashMap<String,String>) -> JuizResult<Value> {
    juiz_lock(crud_broker)?.update_class(class_name, function_name, arg, params)
}