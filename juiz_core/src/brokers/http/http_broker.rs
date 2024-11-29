use std::{net::SocketAddr, path::PathBuf, sync::{Arc, Mutex}};
use tokio::net::TcpListener;

use super::super::core_broker::CoreBrokerPtr;
use crate::{brokers::broker_ptr::BrokerPtr, prelude::*};
use crate::brokers::{broker_factory_impl::create_broker_factory_impl, BrokerFactory, CRUDBrokerHolder};
use crate::brokers::CRUDBroker;

use super::http_router::app_new;

async fn on_start(broker_manifest: Value, crud_broker: Arc<Mutex<CRUDBroker>>) -> () {
    // log::trace!("http_broker::on_start(broker_manifest={broker_manifest:}) called");
    let host = obj_get_str(&broker_manifest, "host").or::<&str>(Ok("0.0.0.0") ).unwrap();
    let port  = obj_get_i64(&broker_manifest, "port").or::<i64>( Ok(8080)).unwrap();
    let address = format!("{:}:{:}", host, port);
    log::info!("http_broker::on_start(address={address}, {broker_manifest:?})) called");
    let static_filepaths: Option<Vec<(String, PathBuf)>> = match obj_get_obj(&broker_manifest, "static_filepaths") {
        Ok(v) => {
            Some(v.iter().map(|(s, val)| {
                let path: PathBuf = val.as_str().or(Some("")).unwrap().into();
                (s.clone(), path)
            }).collect::<Vec<(String, PathBuf)>>())
        }
        Err(_e) => {
            None
        }
    };
    match TcpListener::bind( address ).await {
        Ok(listener) => {
            axum::serve(listener, app_new(crud_broker, static_filepaths).into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
        },
        Err(e) => {
            // TODO: ここで同じポートにすでにhttpがあったら、スレーブになってそこに接続する
            log::error!("on_start(broker_manifest='{broker_manifest:}') failed. Error({e})");
            return ();
        },
    }
//    axum::serve(TcpListener::bind( address ).await.unwrap(), app_new(crud_broker)).await.unwrap();
    log::trace!("http_broker::on_start() exit");
}


pub fn http_broker_factory(core_broker: CoreBrokerPtr) -> JuizResult<Arc<Mutex<dyn BrokerFactory>>> {
    fn create_broker_function(core_broker: CoreBrokerPtr, manifest: Value) -> JuizResult<BrokerPtr> {
        Ok(BrokerPtr::new(CRUDBrokerHolder::new("HTTPBroker", "http", core_broker, &on_start, manifest.clone())?))
    }

    let manifest = jvalue!({
        "type_name": "http"
    });
    create_broker_factory_impl(core_broker, manifest, create_broker_function)
}

