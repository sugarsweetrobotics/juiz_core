use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use juiz_core::{jvalue, JuizResult, brokers::{CRUDBrokerHolder, broker_factory::create_broker_factory_impl, BrokerFactory}, Value, value::{obj_get_str, obj_get_i64}};
use juiz_core::brokers::{Broker, BrokerProxy, CRUDBroker};

use crate::http_router::app_new;

async fn on_start(broker_manifest: Value, crud_broker: Arc<Mutex<CRUDBroker>>) -> () {
    log::trace!("http_broker::on_start() called");
    let host = obj_get_str(&broker_manifest, "host").or::<&str>(Ok("0.0.0.0") ).unwrap();
    let port  = obj_get_i64(&broker_manifest, "port").or::<i64>( Ok(8080)).unwrap();
    let address = format!("{:}:{:}", host, port);
    log::info!("http_broker::on_start(address={address}, {broker_manifest:?})) called");
    axum::serve(TcpListener::bind( address ).await.unwrap(), app_new(crud_broker)).await.unwrap();
    log::trace!("http_broker::on_start() exit");
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

