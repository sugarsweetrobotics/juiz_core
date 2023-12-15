use std::sync::{Arc, Mutex};

use juiz_core::{jvalue, JuizResult, brokers::{CRUDBrokerHolder, broker_factory::create_broker_factory_impl, broker_proxy_factory::create_broker_proxy_factory_impl, BrokerProxyFactory, BrokerFactory}, Value};
use juiz_core::brokers::{Broker, BrokerProxy, CRUDBroker};



use crate::http_router::app::app_new;

async fn on_start(broker_manifest: Value, crud_broker: Arc<Mutex<CRUDBroker>>) -> () {
    let app = app_new(crud_broker);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


#[no_mangle]
pub unsafe extern "Rust" fn broker_factory(core_broker: Arc<Mutex<dyn BrokerProxy>>) -> JuizResult<Arc<Mutex<dyn BrokerFactory>>> {
    env_logger::init();

    fn create_broker_function(core_broker: Arc<Mutex<dyn BrokerProxy>>, manifest: Value) -> JuizResult<Arc<Mutex<dyn Broker>>> {
        CRUDBrokerHolder::new("HTTPBroker", "http", core_broker, &on_start, manifest.clone())
    }

    let manifest = jvalue!({
        "type_name": "http"
    });
    create_broker_factory_impl(core_broker, manifest, create_broker_function)
}


#[no_mangle]
pub unsafe extern "Rust" fn broker_proxy_factory() -> JuizResult<Arc<Mutex<dyn BrokerProxyFactory>>> {
    
    fn create_broker_proxy_function(_manifest: Value) -> JuizResult<Arc<Mutex<dyn BrokerProxy>>> {
        todo!()
    }

    let manifest = jvalue!({
        "type_name": "http"
    });
    create_broker_proxy_factory_impl(manifest, create_broker_proxy_function)
}