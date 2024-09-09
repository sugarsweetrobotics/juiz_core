use std::sync::{Arc, Mutex};

use crate::prelude::*;
use crate::brokers::BrokerProxy;
use super::crud_callback_container::{create_callback_container, delete_callback_container, read_callback_container, update_callback_container, ClassCallbackContainerType};

pub struct CRUDBroker {
    core_broker: Arc<Mutex<dyn BrokerProxy>>,

    create_callback_container: ClassCallbackContainerType, //HashMap<&'static str, FnType>
    read_callback_container: ClassCallbackContainerType, //HashMap<&'static str, FnType>
    update_callback_container: ClassCallbackContainerType, //HashMap<&'static str, FnType>
    delete_callback_container: ClassCallbackContainerType, //HashMap<&'static str, FnType>
}

fn _resource_name_to_cls_and_id<'a>(resource_name: &'a str, _params: &Vec<String>) -> JuizResult<(&'a str, Identifier)> {
    let mut split = resource_name.split('/');
    let class_name = split.next().ok_or( anyhow::Error::from(JuizError::CRUDBrokerGivenResourseNameHasNoClassNameError{resource_name: resource_name.to_string()} ))?;
    Ok((class_name, "".to_string()))
}
/*
fn params_get(map: &HashMap<String, String>, key: &str) -> JuizResult<String> {
    match map.get(key) {
        None => Err(anyhow::Error::from(JuizError::CRUDBrokerCanNotParameterFunctionError { key_name: key.to_string() })),
        Some(v) => Ok(v.clone())
    }
}
*/

/*
fn extract_method_parameters<'a>(args: &'a CapsuleMap) -> JuizResult<(&'a str, &'a str, &'a str, &'a HashMap<String, String>)> {
    // method_name, class_name, function_name, params
    let err = |name: &str | anyhow::Error::from(JuizError::CapsuleDoesNotIncludeParamError{ name: name.to_owned() });
    let method_name = args.get_param("method_name").ok_or_else( || err("method_name") )?;
    let class_name = args.get_param("class_name").ok_or_else( || err("class_name") )?;
    let function_name = args.get_param("function_name").ok_or_else( || err("function_name") )?;
    let params = args.get_params();
    Ok((method_name, class_name, function_name, params))
    //todo!("ここにArgumentMapからmethod_nameなどのパラメータを抽出するコードを書く")
}
*/


fn extract_class_name<'a>(args: &'a CapsuleMap) -> JuizResult<String> {
    // method_name, class_name, function_name, params
    let err = |name: &str | anyhow::Error::from(JuizError::CapsuleDoesNotIncludeParamError{ name: name.to_owned() });
    let class_name = args.get_param("class_name").ok_or_else( || err("class_name") )?;
    Ok(class_name.to_owned())
}

fn extract_function_name<'a>(args: &'a CapsuleMap) -> JuizResult<&String> {
    let err = |name: &str | anyhow::Error::from(JuizError::CapsuleDoesNotIncludeParamError{ name: name.to_owned() });
    let function_name = args.get_param("function_name").ok_or_else( || err("function_name") )?;
    Ok(function_name)
}

impl CRUDBroker {
    pub fn new(core_broker: Arc<Mutex<dyn BrokerProxy>>) -> JuizResult<CRUDBroker> {
        Ok(CRUDBroker{core_broker, 
            create_callback_container: create_callback_container(), 
            read_callback_container: read_callback_container(),
            update_callback_container: update_callback_container(),
            delete_callback_container: delete_callback_container()
        })
    }

    /*
    pub fn create_class_old(&self, arg_value: CapsuleMap) -> JuizResult<Capsule> {
        let class_name = extract_class_name(&arg_value)?;
        //let function_name = extract_function_name(&arg_value)?;

        //let (method_name, class_name, function_name, params) = extract_method_parameters(&arg_value)?;
        log::trace!("CRUDBroker::create_class({class_name}, function_name, value) called");
        //let value = extract_create_parameter(arg_value);
        let mut cb = juiz_lock(&self.core_broker)?;
        match class_name.as_str() {
            "system" => {
                match extract_function_name(&arg_value)? {
                    _ => Err(anyhow::Error::from(JuizError::CRUDBrokerCanNotFindFunctionError{class_name:class_name, function_name: extract_function_name(&arg_value)?.clone()}))
                }
            },
            "process" => {
                match extract_function_name(&arg_value)? {
                    _ => {
                        return Err(anyhow::Error::from(JuizError::CRUDBrokerCanNotFindFunctionError{class_name:class_name, function_name: extract_function_name(&arg_value)?.clone()}));
                    }
                }
            },
            "connection" => {
                match extract_function_name(&arg_value)?.as_str() {
                    "create" => {
                        let mut result = cb.connection_create(extract_create_parameter(arg_value))?;
                        result.set_option("function_name", "create");
                        Ok(result)
                    },
                    _ => Err(anyhow::Error::from(JuizError::CRUDBrokerCanNotFindFunctionError{class_name:class_name, function_name: extract_function_name(&arg_value)?.clone()}))
                }
            }
            _ => {
                Ok(Capsule::from(jvalue!({})))
            }
        }
    }
    */

    fn call_callback(&self, cb_container: &ClassCallbackContainerType, args: CapsuleMap) -> JuizResult<CapsulePtr> {
        match cb_container.get(extract_class_name(&args)?.as_str()) {
            None => {
                Err(anyhow::Error::from(JuizError::CRUDBrokerCanNotFindFunctionError { class_name: extract_class_name(&args)?, function_name: extract_function_name(&args)?.clone()}))
            },
            Some(cbs) => {
                let function_name = extract_function_name(&args)?.clone();
                let class_name = extract_class_name(&args)?.clone();
                
                match cbs.get(function_name.as_str()) {
                    None => {
                        Err(anyhow::Error::from(JuizError::CRUDBrokerCanNotFindFunctionError { class_name: extract_class_name(&args)?, function_name: extract_function_name(&args)?.clone()}))
                    },
                    Some(cb) => {
                        let mut capsule = cb(Arc::clone(&self.core_broker), args)?;
                        let _  = capsule.set_function_name(function_name.as_str())?;
                        let _ = capsule.set_class_name(class_name.as_str())?;
                        Ok(capsule)
                    }
                }
            }
        }
    }

    pub fn create_class(&self, args: CapsuleMap) -> JuizResult<CapsulePtr> {
        self.call_callback(&self.create_callback_container, args)
    }

    pub fn read_class(&self, args: CapsuleMap) -> JuizResult<CapsulePtr> {
        self.call_callback(&self.read_callback_container, args)
    }

    pub fn update_class(&self, args: CapsuleMap) -> JuizResult<CapsulePtr> {
        self.call_callback(&self.update_callback_container, args)
    }

    pub fn delete_class(&self, args: CapsuleMap) -> JuizResult<CapsulePtr> {
        self.call_callback(&self.delete_callback_container, args)
    }
    /*
    pub fn read_class_old(&self, args: CapsuleMap) -> JuizResult<Capsule> {
        let (method_name, class_name, function_name, params) = extract_method_parameters(&args)?;
        log::trace!("CRUDBroker::read_class({class_name}, {function_name}, {params:?}) called");
        let cb = juiz_lock(&self.core_broker)?;
        match class_name {
            "system" => {
                match function_name {
                    "profile_full" => {
                        let mut result = cb.system_profile_full()?;
                        result.set_option("function_name", "profile_full");
                        Ok(result)
                    },
                    _ => {
                        return Err(anyhow::Error::from(JuizError::CRUDBrokerCanNotFindFunctionError{class_name:class_name.to_string(), function_name: function_name.to_string()}));
                    }
                }
            },
            "broker" => {
                match function_name {
                    "profile_full" => {
                        let id = params_get(params, "identifier").context("CRUDBroker.read_class()")?;
                        return Ok(Capsule::from(cb.broker_profile_full(&id)?))
                    },
                    "list" => return Ok(Capsule::from(cb.broker_list()?)),
                    _ => {
                        return Err(anyhow::Error::from(JuizError::CRUDBrokerCanNotFindFunctionError{class_name:class_name.to_string(), function_name: function_name.to_string()}));
                    }
                }
            },
            "process" => {
                match function_name {
                    "profile_full" => {
                        let id = params_get(params, "identifier")?;
                        return Ok(Capsule::from(cb.process_profile_full(&id)?))
                    },
                    "list" => return Ok(Capsule::from(cb.process_list()?)),
                    _ => {
                        return Err(anyhow::Error::from(JuizError::CRUDBrokerCanNotFindFunctionError{class_name:class_name.to_string(), function_name: function_name.to_string()}));
                    }
                }
            },
            "container" => {
                
                match function_name {
                    "profile_full" => return Ok(Capsule::from(cb.container_profile_full(&params_get(params, "identifier")?)?)),
                    "list" => return Ok(Capsule::from(cb.container_list()?)),
                    _ => {
                        return Err(anyhow::Error::from(JuizError::CRUDBrokerCanNotFindFunctionError{class_name:class_name.to_string(), function_name: function_name.to_string()}));
                    }
                }
            },
            "container_process" => {
                match function_name {
                    "profile_full" => return Ok(Capsule::from(cb.container_process_profile_full(&params_get(params, "identifier")?)?)),
                    "list" => return Ok(Capsule::from(cb.container_process_list()?)),
                    _ => {
                        return Err(anyhow::Error::from(JuizError::CRUDBrokerCanNotFindFunctionError{class_name:class_name.to_string(), function_name: function_name.to_string()}));
                    }
                }
            },
            "connection" => {
                match function_name {
                    "profile_full" => {
                        let id = params_get(params, "identifier")?;
                        return Ok(Capsule::from(cb.connection_profile_full(&id)?))
                    },
                    "list" => return Ok(Capsule::from(cb.connection_list()?)),
                    _ => {
                        return Err(anyhow::Error::from(JuizError::CRUDBrokerCanNotFindFunctionError{class_name:class_name.to_string(), function_name: function_name.to_string()}));
                    }
                }
            },
            "execution_context" => {
                match function_name {
                    "profile_full" => return Ok(Capsule::from(cb.ec_profile_full(&params_get(params, "identifier")?)?)),
                    "list" => return Ok(Capsule::from(cb.ec_list()?)),
                    "get_state" => return Ok(Capsule::from(cb.ec_get_state(&params_get(params, "identifier")?)?)),
                    _ => {
                        return Err(anyhow::Error::from(JuizError::CRUDBrokerCanNotFindFunctionError{class_name:class_name.to_string(), function_name: function_name.to_string()}));
                    }
                }
            },
            _ => {
                Ok(Capsule::from(jvalue!({})))
            }
        }
    }
    */
    /*
    pub fn update_class(&self, args: CapsuleMap) -> JuizResult<Capsule> {
        let (method_name, class_name, function_name, params) = extract_method_parameters(&args)?;
        
        log::trace!("CRUDBroker::update_class(class_name={class_name:}, function_name={function_name:}, params={params:?}) called");
        let mut cb = juiz_lock(&self.core_broker)?;
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
                    "call" => return cb.process_call(&id, args),
                    "execute" => return cb.process_execute(&id),
                    _ => {
                        return Err(anyhow::Error::from(JuizError::CRUDBrokerCanNotFindFunctionError{class_name:class_name.to_string(), function_name: function_name.to_string()}));
                    },
                }
            },
            "container_process" => {
                let id = params_get(params, "identifier")?;
                match function_name {
                    "call" => return cb.container_process_call(&id, args),
                    "execute" => return cb.container_process_execute(&id),
                    _ => {
                        return Err(anyhow::Error::from(JuizError::CRUDBrokerCanNotFindFunctionError{class_name:class_name.to_string(), function_name: function_name.to_string()}));
                    },
                }
            },
            "execution_context" => {
                match function_name {
                    "start" => return Ok(Capsule::from(cb.ec_start(&params_get(params, "identifier")?)?)),
                    "stop" => return Ok(Capsule::from(cb.ec_stop(&params_get(params, "identifier")?)?)),
                    _ => {
                        return Err(anyhow::Error::from(JuizError::CRUDBrokerCanNotFindFunctionError{class_name:class_name.to_string(), function_name: function_name.to_string()}));
                    }
                }
            },
            _ => {
                Ok(Capsule::from(jvalue!({})))
            }
        }
    }
    */
    /* 
    pub fn delete_class(&self, args: CapsuleMap) -> JuizResult<Capsule> {
        log::trace!("CRUDBroker::read_class called");
        let (method_name, class_name, function_name, params) = extract_method_parameters(&args)?;
        
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
                Ok(Capsule::from(jvalue!({})))
            }
        }
    }
    */
}

/* 
pub fn create_class(crud_broker: &Arc<Mutex<CRUDBroker>>, args: CapsuleMap) -> JuizResult<Capsule> {
    juiz_lock(crud_broker)?.create_class(args)
}

pub fn read_class(crud_broker: &Arc<Mutex<CRUDBroker>>, args: CapsuleMap) -> JuizResult<Capsule> {
    juiz_lock(crud_broker)?.read_class(args)
}

pub fn delete_class(crud_broker: &Arc<Mutex<CRUDBroker>>, args: CapsuleMap) -> JuizResult<Capsule> {
    juiz_lock(crud_broker)?.delete_class(args)
}

pub fn update_class(crud_broker: &Arc<Mutex<CRUDBroker>>, args: CapsuleMap) -> JuizResult<Capsule> {
    juiz_lock(crud_broker)?.update_class(args)
}
*/