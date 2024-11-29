use std::{io::ErrorKind, net::SocketAddr, path::PathBuf, sync::{Arc, Mutex}};
//use axum::extract::path::ErrorKind;
use tokio::net::TcpListener;

use super::super::core_broker::CoreBrokerPtr;
use crate::{brokers::broker_ptr::BrokerPtr, prelude::*};
use crate::brokers::{broker_factory_impl::create_broker_factory_impl, BrokerFactory, CRUDBrokerHolder};
use crate::brokers::CRUDBroker;

use super::http_router::app_new;

fn into_address(host: &str, port: i64) -> String {
    return format!("{:}:{:}", host, port);
}
fn check_server_is_juiz(host: &str, port: i64) -> bool {
    return true;
}

async fn on_start(broker_manifest: Value, crud_broker: Arc<Mutex<CRUDBroker>>) -> () {
    // log::trace!("http_broker::on_start(broker_manifest={broker_manifest:}) called");
    let host = obj_get_str(&broker_manifest, "host").or::<&str>(Ok("0.0.0.0") ).unwrap();
    let mut port  = obj_get_i64(&broker_manifest, "port").or::<i64>( Ok(8080)).unwrap();
    //let address = format!("{:}:{:}", host, port);
    log::info!("http_broker::on_start(host={host}, port={port}, {broker_manifest:?})) called");
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

    loop {
        let address = into_address(host, port);
        log::info!("-- connecting (host={host}, port={port}, {broker_manifest:?})) called");
        match TcpListener::bind( address ).await {
            Ok(listener) => {
                log::trace!("http_broker::on_start() exit");
                return axum::serve(listener, app_new(crud_broker, static_filepaths).into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
            },
            Err(e) => {
                if e.kind() == ErrorKind::AddrInUse {
                    // check server is juiz?
                    if !check_server_is_juiz(host, port) {
                        log::error!("on_start(broker_manifest='{broker_manifest:}') failed. Error({e:?}, {e})");
                        return ();
                    }
                    port = port + 1;
                } else {
                    // TODO: ここで同じポートにすでにhttpがあったら、スレーブになってそこに接続する
                    log::error!("on_start(broker_manifest='{broker_manifest:}') failed. Error({e:?}, {e})");
                    return ();
                }
            },
        }
    }// loop
//    axum::serve(TcpListener::bind( address ).await.unwrap(), app_new(crud_broker)).await.unwrap();
    
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

