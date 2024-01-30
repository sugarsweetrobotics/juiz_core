use std::{sync::{Arc, Mutex}, collections::HashMap};


use juiz_core::{JuizResult, brokers::{BrokerProxyFactory, BrokerProxy, create_broker_proxy_factory_impl}, Value, jvalue, value::obj_get_str, JuizError, processes::Output};

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

    Ok(("localhost", 3000))
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

fn construct_url(base_url: &String, class_name: &str, function_name: &str, param: &HashMap<String, String>) -> String {
    let url = base_url.clone() + "/" + class_name + "/" + function_name;
    if param.len() == 0 {
        return url;
    }
    let m = param.iter().map(|(k,v)|{(k).clone() + "=" + (v).as_str()});
    return url + "?" + m.collect::<Vec<String>>().join("&").as_str();
}

impl CRUDBrokerProxy for HTTPBrokerProxy {
    fn create(&self, class_name: &str, function_name: &str, payload: Value, param: std::collections::HashMap<String, String>) -> JuizResult<Value> {
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
                response.json().map_err(|e| anyhow::Error::from(e))
            }
        }
    }

    fn delete(&self, class_name: &str, function_name: &str, param: std::collections::HashMap<String, String>) -> JuizResult<Value> {
        let client = reqwest::blocking::Client::new();
        match client.delete(construct_url(&self.base_url, class_name, function_name, &param)).send() {
            Err(e) => Err(anyhow::Error::from(e)),
            Ok(response) => {
                response.json().map_err(|e| anyhow::Error::from(e))
            }
        }
    }

    fn read(&self, class_name: &str, function_name: &str, param: std::collections::HashMap<String, String>) -> JuizResult<Value> {
        log::info!("HTTPBrokerProxy.read() called");
        
        let client = reqwest::blocking::Client::new();
        let url  =construct_url(&self.base_url, class_name, function_name, &param);
        log::trace!("HTTPBrokerProxy.read(url={url:})");
        match client.get(url).send() {
            Err(e) => Err(anyhow::Error::from(e)),
            Ok(response) => {
                if response.status() != 200 {
                    return Err(anyhow::Error::from(HTTPBrokerError::HTTPStatusError{status_code: response.status(), message: format!("{:?}", response) }));
                }
                response.json().map_err(|e| anyhow::Error::from(e))
            }
        }
    }

    fn update(&self, class_name: &str, function_name: &str, payload: Value, param: std::collections::HashMap<String, String>) -> JuizResult<Output> {
        let client = reqwest::blocking::Client::new();
        match client.patch(construct_url(&self.base_url, class_name, function_name, &param))
            .json(&payload)
            .send() {
            Err(e) => Err(anyhow::Error::from(e)),
            Ok(response) => {
                match response.json() {
                    Err(e) => Err(anyhow::Error::from(e)),
                    Ok(v) => Ok(Output::new_from_value(v))
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