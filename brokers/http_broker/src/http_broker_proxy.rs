use std::{sync::{Arc, Mutex}, collections::HashMap};


use juiz_core::{brokers::{create_broker_proxy_factory_impl, BrokerProxy, BrokerProxyFactory}, identifier::IdentifierStruct, jvalue, processes::capsule::CapsuleMap, value::obj_get_str, CapsulePtr, JuizError, JuizResult, Value};

use juiz_core::brokers::{CRUDBrokerProxy, CRUDBrokerProxyHolder};

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
enum HTTPBrokerError {
    #[error("General Error")]
    GeneralError{},

    #[error("HTTPStatusError (status_code={status_code:}), message_body={message}")]
    HTTPStatusError { status_code: reqwest::StatusCode, message: String },
}

fn name_to_host_and_port<'a>(name: &'a String) -> JuizResult<(&'a str, i64)> {
    let mut tokens = name.split(':');
    let host =  tokens.next();
    if host.is_none() {
        return Err(anyhow::Error::from(JuizError::BrokerNameCanNotResolveToURLError{given_name: name.clone()}));
    }
    let port = tokens.next();
    if port.is_none() {
        return Ok((host.unwrap(), 8080))
    }
    Ok((host.unwrap(), port.unwrap().parse()?))
}


struct HTTPBrokerProxy {
    base_url: String,
    
}

impl HTTPBrokerProxy {

    pub fn new(manifest: &Value) -> JuizResult<HTTPBrokerProxy> {
        let name = obj_get_str(manifest, "name")?.to_string();
        let (addr, port) = name_to_host_and_port(&name)?;
        Ok(HTTPBrokerProxy{
            base_url: "http://".to_string() + addr + ":" + i64::to_string(&port).as_str() + "/api"
        })
    }
}

fn construct_param(key: &String, value: &String) -> String {
    if key == "identifier" {
        let mut id_struct = IdentifierStruct::from(value.clone());
        id_struct.broker_type_name = "core".to_owned();
        id_struct.broker_name = "core".to_owned();
        let new_id: String = id_struct.into();
        key.clone() + "=" + new_id.as_str()
    } else {
        (key).clone() + "=" + (value).as_str()
    }
}

fn construct_url(base_url: &String, class_name: &str, function_name: &str, param: &HashMap<String, String>) -> String {
    let url = base_url.clone() + "/" + class_name + "/" + function_name;
    if param.len() == 0 {
        return url;
    }
    let m = param.iter().map(|(k,v)|{construct_param(k, v)});
    return url + "?" + m.collect::<Vec<String>>().join("&").as_str();
}


fn to_payload<'a>(_payload: &'a CapsuleMap) -> JuizResult<&'a Value> {
    todo!();
}

impl CRUDBrokerProxy for HTTPBrokerProxy {
    fn create(&self, class_name: &str, function_name: &str, payload: Value, param: std::collections::HashMap<String, String>) -> JuizResult<CapsulePtr> {
        let client = reqwest::blocking::Client::new();
        match client.post(construct_url(&self.base_url, class_name, function_name, &param))
            .json(&payload)
            .send() {
            Err(e) => Err(anyhow::Error::from(e)),
            Ok(response) => {
                log::error!("response: {:?}", response);
                if response.status() != 200 {
                    return Err(anyhow::Error::from(HTTPBrokerError::GeneralError{}));
                }
                Ok(response.json::<Value>().map_err(|e| anyhow::Error::from(e))?.into())
            }
        }
    }

    fn delete(&self, class_name: &str, function_name: &str, param: std::collections::HashMap<String, String>) -> JuizResult<CapsulePtr> {
        let client = reqwest::blocking::Client::new();
        match client.delete(construct_url(&self.base_url, class_name, function_name, &param)).send() {
            Err(e) => Err(anyhow::Error::from(e)),
            Ok(response) => {
                Ok(response.json::<Value>().map_err(|e| anyhow::Error::from(e))?.into())
            }
        }
    }


    fn read(&self, class_name: &str, function_name: &str, param: std::collections::HashMap<String, String>) -> JuizResult<CapsulePtr> {
        log::trace!("HTTPBrokerProxy({class_name:}, {function_name}, {param:?}).read() called");
        
        let client = reqwest::blocking::Client::new();
        let url  =construct_url(&self.base_url, class_name, function_name, &param);
        log::trace!("HTTPBrokerProxy.read(url={url:})");
        match client.get(url.clone()).send() {
            Err(e) => Err(anyhow::Error::from(e)),
            Ok(response) => {
                if response.status() != 200 {
                    log::error!("HTTPBrokerProxy.read(url={url:}) failed. Response is {response:?}");
                    return Err(anyhow::Error::from(HTTPBrokerError::HTTPStatusError{status_code: response.status(), message: format!("{:?}", response) }));
                }
                let value = response.json::<Value>().map_err(|e| anyhow::Error::from(e))?;
                log::trace!("HTTPBrokerProxy.read() Response = {value:?}");
                Ok(value.into())
            }
        }
    }


    fn update(&self, class_name: &str, function_name: &str, payload: CapsuleMap, param: std::collections::HashMap<String, String>) -> JuizResult<CapsulePtr>{
        let client = reqwest::blocking::Client::new();
        match client.patch(construct_url(&self.base_url, class_name, function_name, &param))
            .json(to_payload(&payload)?)
            .send() {
            Err(e) => Err(anyhow::Error::from(e)),
            Ok(response) => {
                match response.json::<Value>() {
                    Err(e) => Err(anyhow::Error::from(e)),
                    Ok(v) => Ok(v.into())
                }
            }
        }
    }
}


fn create_broker_proxy_function(manifest: Value) -> JuizResult<Arc<Mutex<dyn BrokerProxy>>> {
    let name = obj_get_str(&manifest, "name")?;
    Ok(CRUDBrokerProxyHolder::new("HTTPBrokerProxy", "http", name, Box::new(HTTPBrokerProxy::new(&manifest)?))?)
}

#[no_mangle]
pub unsafe extern "Rust" fn broker_proxy_factory() -> JuizResult<Arc<Mutex<dyn BrokerProxyFactory>>> {
    let manifest = jvalue!({
        "type_name": "http"
    });
    create_broker_proxy_factory_impl(manifest, create_broker_proxy_function)
}