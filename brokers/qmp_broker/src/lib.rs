use std::sync::{Arc, Mutex};

use juiz_core::{anyhow::{self, anyhow}, brokers::{create_broker_factory_impl, create_broker_proxy_factory_impl, CRUDBroker, CRUDBrokerHolder}, prelude::*};

mod qmp_broker;
mod qmp_broker_proxy;

pub(crate) fn value_to_vecu8(val: &Value) -> anyhow::Result<Vec<u8>> {
    rmp_serde::to_vec(val).or_else(|e| { Err(anyhow!(e)) })
}

pub(crate) fn vecu8_to_value(mut vecu8: Vec<u8>) -> anyhow::Result<Value> {
    rmp_serde::from_slice::<serde_json::Value>(&mut vecu8.as_mut_slice()[..] ).or_else(|e|{Err(anyhow!(e))})
}

pub(crate) fn payload_to_request_value(class_name: &str, function_name: &str, method_name: &str, payload: Value, mut param: std::collections::HashMap<String, String>) -> Value {
    param.insert("class_name".to_owned(), class_name.into());
    param.insert("function_name".to_owned(), function_name.into());
    param.insert("method_name".to_owned(), method_name.into());
    jvalue!({
        "map": payload,
        "param": param
    })
}

pub(crate) fn to_request_value(class_name: &str, function_name: &str, method_name: &str, mut param: std::collections::HashMap<String, String>) -> Value {
    param.insert("class_name".to_owned(), class_name.into());
    param.insert("function_name".to_owned(), function_name.into());
    param.insert("method_name".to_owned(), method_name.into());
    jvalue!({
        "map": {},
        "param": param
    })
}

pub(crate) fn to_request(value: Value) -> anyhow::Result<CapsuleMap> {
    value.try_into()
}


pub async fn on_start(broker_manifest: Value, crud_broker: Arc<Mutex<CRUDBroker>>) -> () {
    qmp_broker::on_start(broker_manifest, crud_broker).await
}


#[no_mangle]
pub unsafe extern "Rust" fn broker_factory(core_broker: Arc<Mutex<dyn BrokerProxy>>) -> JuizResult<Arc<Mutex<dyn BrokerFactory>>> {
    juiz_core::env_logger::init();

    fn create_broker_function(core_broker: Arc<Mutex<dyn BrokerProxy>>, manifest: Value) -> JuizResult<Arc<Mutex<dyn Broker>>> {
        CRUDBrokerHolder::new("QuinnBroker", "qmp", core_broker, &on_start, manifest.clone())
    }

    let manifest = jvalue!({
        "type_name": "qmp"
    });

    create_broker_factory_impl(core_broker, manifest, create_broker_function)
}

#[no_mangle]
pub unsafe extern "Rust" fn broker_proxy_factory() -> JuizResult<Arc<Mutex<dyn BrokerProxyFactory>>> {
    let manifest = jvalue!({
        "type_name": "qmp"
    });
    create_broker_proxy_factory_impl(manifest, qmp_broker_proxy::create_broker_proxy_function)
}