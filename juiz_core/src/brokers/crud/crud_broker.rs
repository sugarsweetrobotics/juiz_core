
use std::net::SocketAddr;
use juiz_sdk::anyhow::anyhow;
use crate::prelude::*;
use super::super::core_broker::CoreBrokerPtr;
use super::crud_callback_container::{create_callback_container, delete_callback_container, read_callback_container, update_callback_container, ClassCallbackContainerType};

//#[derive(Debug)]
pub struct CRUDBroker {
    core_broker: CoreBrokerPtr,
    identifier: Identifier,
    _name: String,
    manifest: Value,
    create_callback_container: ClassCallbackContainerType, //HashMap<&'static str, FnType>
    read_callback_container: ClassCallbackContainerType, //HashMap<&'static str, FnType>
    update_callback_container: ClassCallbackContainerType, //HashMap<&'static str, FnType>
    delete_callback_container: ClassCallbackContainerType, //HashMap<&'static str, FnType>
}

fn _resource_name_to_cls_and_id<'a>(resource_name: &'a str, _params: &Vec<String>) -> JuizResult<(&'a str, Identifier)> {
    let mut split = resource_name.split('/');
    let class_name = split.next().ok_or( anyhow!(JuizError::CRUDBrokerGivenResourseNameHasNoClassNameError{resource_name: resource_name.to_string()} ))?;
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


// fn extract_class_name<'a>(args: &'a CapsuleMap) -> JuizResult<String> {
//     // method_name, class_name, function_name, params
//     let err = |name: &str | anyhow::Error::from(JuizError::CapsuleDoesNotIncludeParamError{ name: name.to_owned() });
//     let class_name = args.get_param("class_name").ok_or_else( || err("class_name") )?;
//     Ok(class_name.to_owned())
// }

// fn extract_function_name<'a>(args: &'a CapsuleMap) -> JuizResult<&String> {
//     let err = |name: &str | anyhow::Error::from(JuizError::CapsuleDoesNotIncludeParamError{ name: name.to_owned() });
//     let function_name = args.get_param("function_name").ok_or_else( || err("function_name") )?;
//     Ok(function_name)
// }

impl CRUDBroker {
    pub fn new(core_broker: CoreBrokerPtr, manifest: Value) -> JuizResult<CRUDBroker> {
        let broker_name = manifest.as_object().unwrap().get("name").unwrap().as_str().unwrap();
        let broker_type = manifest.as_object().unwrap().get("type_name").unwrap().as_str().unwrap();
        let id = format!("core://core/Broker/{broker_name}::{broker_type}");
        Ok(CRUDBroker{core_broker, 
            identifier: id,
            _name: broker_name.to_owned(),
            create_callback_container: create_callback_container(), 
            read_callback_container: read_callback_container(),
            update_callback_container: update_callback_container(),
            delete_callback_container: delete_callback_container(),
            manifest
        })
    }

    pub fn name(&self) -> String {
        self._name.clone()
    }

    pub fn identifier(&self) -> Identifier {
        self.identifier.clone()
    }

    pub fn manifest(&self) -> Value {
        self.manifest.clone()
    }

    pub fn update_broker_name(&mut self, name: &str) -> () {
        log::error!("update_broker_name({name}) called");
        self._name = name.to_owned();
        self.manifest.as_object_mut().unwrap().insert("name".to_owned(), name.into());
        let broker_name = self.manifest.as_object().unwrap().get("name").unwrap().as_str().unwrap();
        let broker_type = self.manifest.as_object().unwrap().get("type_name").unwrap().as_str().unwrap();
        self.identifier = format!("core://core/Broker/{broker_name}::{broker_type}");
    }

    pub fn reserve_master_broker(&mut self, master_info: Value) -> JuizResult<()> {
        log::trace!("reserve_master_broker({master_info:}) called");
        self.core_broker.lock_mut()?.reserve_master_broker(master_info)
    }

    fn on_update(&self, class_name:&str, function_name:&str, cp: CapsuleMap, opt_remote_addr: Option<SocketAddr>) -> JuizResult<CapsulePtr>{
        log::trace!("on_update({cp:?}) called");
        if class_name == "system" && function_name == "add_mastersystem" {
            if let Some(remote_addr) = opt_remote_addr {
                let remote_addr_str = remote_addr.to_string();
                let _r = match cp.get("profile") {
                    Ok(capsule_ptr) => {
                        capsule_ptr.lock_modify_as_value(|v|{
                            match v.as_object_mut().unwrap().get_mut("subsystem").unwrap().as_object_mut() {
                                Some(obj) => {
                                    let broker_name = obj.get("broker_name").unwrap().as_str().unwrap().to_owned();
                                    let broker_tokens = broker_name.split(":").collect::<Vec<&str>>();
                                    let port_str = broker_tokens.get(1).unwrap();
                                    let remote_tokens = remote_addr_str.split(":").collect::<Vec<&str>>();
                                    let addr_str = (*remote_tokens.get(0).unwrap()).to_owned();
                                    let new_broker_name = addr_str + ":" + port_str;
                                    obj.insert("broker_name".to_owned(), jvalue!(new_broker_name));
                                }
                                None => todo!(),
                            }
                        })
                    }
                    Err(_) => todo!(),
                };
            }
        }
        let retval = self.update_class(class_name, function_name, cp);
        log::info!("retval: {retval:?}");

        return retval;
    }



    pub fn on_value_request(&self, val: Value, opt_requesting_host_addr: Option<SocketAddr>) -> JuizResult<CapsulePtr> {
        let mut cp: CapsuleMap = val.try_into()?;
        let class_name = cp.get_param("class_name").unwrap().clone();
        let method_name = cp.get_param("method_name").unwrap().clone();
        let function_name = cp.get_param("function_name").unwrap().clone();
        if let Some(requesting_host_addr) = opt_requesting_host_addr {
            cp.set_param("requesting_host_addr", requesting_host_addr.to_string().as_str());
        }
        //let (class_name, function_name, method_name, payload, param) = to_request(&val)?;
        match method_name.as_str() {
            "create" => self.create_class(class_name.as_str(), function_name.as_str(), cp),
            "delete" => self.delete_class(class_name.as_str(), function_name.as_str(), cp),
            "read" => self.read_class(class_name.as_str(), function_name.as_str(), cp),
            "update" => {
                self.on_update(class_name.as_str(), function_name.as_str(), cp, opt_requesting_host_addr)
            }
            _ => {
                Err(anyhow!(JuizError::InvalidValueError{message: format!("qmp_broker received invalid value. Its method name is unknown ({})", method_name)}))
            }
        }
    }

    pub fn create_class(&self, class_name: &str, function_name: &str, args: CapsuleMap) -> JuizResult<CapsulePtr> {
        self.call_callback(&self.create_callback_container, class_name, function_name, args)
    }

    pub fn read_class_value2cap(&self, class_name: &str, function_name: &str, args: Value) -> JuizResult<CapsulePtr> {
        self.call_callback(&self.read_callback_container, class_name, function_name, args.try_into()?)
    }

    pub fn update_class_value2cap(&self, class_name: &str, function_name: &str, args: Value) -> JuizResult<CapsulePtr> {
        self.call_callback(&self.update_callback_container, class_name, function_name, args.try_into()?)
    }

    pub fn delete_class_value2cap(&self, class_name: &str, function_name: &str, args: Value) -> JuizResult<CapsulePtr> {
        self.call_callback(&self.delete_callback_container, class_name, function_name, args.try_into()?)
    }

    pub fn read_class(&self, class_name: &str, function_name: &str, args: CapsuleMap) -> JuizResult<CapsulePtr> {
        self.call_callback(&self.read_callback_container, class_name, function_name, args)
    }

    pub fn update_class(&self, class_name: &str, function_name: &str, args: CapsuleMap) -> JuizResult<CapsulePtr> {
        self.call_callback(&self.update_callback_container, class_name, function_name, args)
    }

    pub fn delete_class(&self, class_name: &str, function_name: &str, args: CapsuleMap) -> JuizResult<CapsulePtr> {
        self.call_callback(&self.delete_callback_container, class_name, function_name, args)
    }

    fn call_callback(&self, cb_container: &ClassCallbackContainerType, class_name: &str, function_name:&str, args: CapsuleMap) -> JuizResult<CapsulePtr> {
        cb_container.get(class_name).and_then(|cbs| {
            cbs.get(function_name)
        }).and_then(|cb|{
            Some(cb(self.core_broker.clone(), args).and_then(|mut capsule|{
                capsule.set_function_name(function_name)?;
                capsule.set_class_name(class_name)?;
                Ok(capsule)
            }))
        }).or_else(||{
            Some(Err(anyhow!(JuizError::CRUDBrokerCanNotFindFunctionError { class_name: class_name.to_owned(), function_name: function_name.to_owned()})))
        }).unwrap()
    }
    
}
