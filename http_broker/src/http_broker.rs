use std::sync::{Arc, Mutex};

use juiz_core::{jvalue, JuizResult, brokers::{CRUDBrokerHolder, broker_factory::create_broker_factory_impl, BrokerFactory}, Value, value::{obj_get_str, obj_get_i64}};
use juiz_core::brokers::{Broker, BrokerProxy, CRUDBroker};

use crate::http_router::app_new;

async fn on_start(broker_manifest: Value, crud_broker: Arc<Mutex<CRUDBroker>>) -> () {
    let host: JuizResult<&str> = obj_get_str(&broker_manifest, "host").or(Ok("0.0.0.0") );
    let port: JuizResult<i64> = obj_get_i64(&broker_manifest, "port").or( Ok(3000));
    let address = host.unwrap().to_string() + ":" + i64::to_string(&port.unwrap()).as_str();
    let app = app_new(crud_broker);
    let listener = tokio::net::TcpListener::bind( address ).await.unwrap();
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

