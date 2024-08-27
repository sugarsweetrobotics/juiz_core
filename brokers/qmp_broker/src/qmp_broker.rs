

use std::sync::Mutex;
use std::time::Duration;
use std::{net::SocketAddr, sync::Arc};
use juiz_core::{prelude::*, utils::juiz_lock, value::{obj_get_i64, obj_get_str}, tokio, anyhow::{self, anyhow}, brokers::CRUDBroker};

use quinn::{Connection, Endpoint, Incoming};
use rustls::pki_types::CertificateDer;
use rustls::crypto::ring::default_provider;
use quinn::ServerConfig;
use rustls::pki_types::PrivatePkcs8KeyDer;
use tracing::{info_span, Instrument, Span};

use crate::{to_request, value_to_vecu8, vecu8_to_value};



fn make_span(conn: &Connection) -> Span {
    info_span!(
        "connection",
        remote = %conn.remote_address(),
        protocol = %conn
            .handshake_data().unwrap()
            .downcast::<quinn::crypto::rustls::HandshakeData>().unwrap()
            .protocol
            .map_or_else(|| "<none>".into(), |x| String::from_utf8_lossy(&x).into_owned())
    )
}

#[allow(unused)]
pub fn make_server_endpoint(
    bind_addr: SocketAddr,
) -> anyhow::Result<(Endpoint, CertificateDer<'static>)> { // Result<(Endpoint, CertificateDer<'static>), Box<dyn Error + Send + Sync + 'static>> {
    let (server_config, server_cert) = configure_server()?;
    let endpoint = Endpoint::server(server_config, bind_addr)?;
    Ok((endpoint, server_cert))
}


/// Returns default server configuration along with its certificate.
fn configure_server(
) -> anyhow::Result<(ServerConfig, CertificateDer<'static>)> {//Result<(ServerConfig, CertificateDer<'static>), Box<dyn Error + Send + Sync + 'static>> {
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
    let cert_der = CertificateDer::from(cert.cert);
    let priv_key = PrivatePkcs8KeyDer::from(cert.key_pair.serialize_der());

    let mut server_config =
        ServerConfig::with_single_cert(vec![cert_der.clone()], priv_key.into())?;
    let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
    transport_config.max_concurrent_uni_streams(0_u8.into());
    transport_config.max_idle_timeout(Some(Duration::from_secs(1).try_into()?));
    Ok((server_config, cert_der))
}

#[allow(unused)]
pub const ALPN_QUIC_HTTP: &[&[u8]] = &[b"hq-29"];

//type CallbackType = Arc<Mutex<dyn Fn(Vec<u8>)-> anyhow::Result<Vec<u8>>  + Send + Sync >> ;


fn callback(request: Vec<u8>, crud_broker: Arc<Mutex<CRUDBroker>>) -> anyhow::Result<Vec<u8>> {
    let val = vecu8_to_value(request)?;
    let cp = to_request(val)?;
    let method_name = cp.get_param("method_name").unwrap();
    //let (class_name, function_name, method_name, payload, param) = to_request(&val)?;
    let capsule_ptr = match method_name.as_str() {
        "create" => juiz_lock(&crud_broker)?.create_class(cp),
        "delete" => juiz_lock(&crud_broker)?.delete_class(cp),
        "read" => juiz_lock(&crud_broker)?.read_class(cp),
        "update" => juiz_lock(&crud_broker)?.update_class(cp),
        _ => {
            Err(anyhow!(JuizError::InvalidValueError{message: format!("qmp_broker received invalid value. Its method name is unknown ({})", method_name)}))
        }
    }?;
    if capsule_ptr.is_value()? {
        capsule_ptr.lock_as_value(|v| {
            value_to_vecu8(v)
        })?
    } else if capsule_ptr.is_mat()? {
        todo!()
    } else {
        todo!()
    }
}

pub async fn on_start(broker_manifest: Value, crud_broker: Arc<Mutex<CRUDBroker>>) -> () {
    log::info!("qmp_broker::on_start(broker_manifest='{broker_manifest:}') called");
    let _ = default_provider().install_default();
    
    let host = obj_get_str(&broker_manifest, "host").or::<&str>(Ok("0.0.0.0") ).unwrap();
    let port  = obj_get_i64(&broker_manifest, "port").or::<i64>( Ok(8080)).unwrap();
    let address = format!("{:}:{:}", host, port);
    run_server(address.parse().unwrap(), Arc::new(move |req| {callback(req, crud_broker.clone())} )).await;
}
   

/// Runs a QUIC server bound to given address.
async fn run_server<F>(addr: SocketAddr, callback: Arc<F>) where F: Fn(Vec<u8>)-> anyhow::Result<Vec<u8>>  + Send + Sync + 'static {
    let (endpoint, _server_cert) = make_server_endpoint(addr).unwrap();
    //let cb = Arc::new(callback);
    loop {
        log::trace!("endpoint.accept in run_server()");
        match endpoint.accept().await {
            Some(incoming_conn) => {
                tokio::spawn(  handle_incoming(incoming_conn, callback.clone()) );
            },
            None => {
                log::error!("endpoint.accept returns None");
            },
        }
    }
}


async fn handle_incoming<F>(incoming: Incoming, callback: Arc<F>) -> anyhow::Result<()> where F: Fn(Vec<u8>)-> anyhow::Result<Vec<u8>>  + Send + Sync {
    log::trace!("handle_incoming() called");
    match incoming.await {
        Ok(connecting) => {
            handle_connection(connecting, callback).await
        },
        Err(e) => {
            log::error!("Incoming.accept() failed. Error {e:?}");
            Err(anyhow!(e))
        }
    }
}


async fn handle_connection<F>(conn: Connection, callback: Arc<F>) -> anyhow::Result<()>  where F: Fn(Vec<u8>)-> anyhow::Result<Vec<u8>>  + Send + Sync {
    log::trace!("handle_connection(from:{})", conn.remote_address());
    async {
        loop {
            match conn.accept_bi().await {
                Err(quinn::ConnectionError::ApplicationClosed { .. }) => {
                    log::info!("Connection Closed.");
                    return Ok(());
                }
                Err(e) => {
                    log::error!("Connection::accept_bi() error. {e:?}");
                    return Err(anyhow!(e))
                }
                Ok(mut s) => {
                    let  (send, recv) = &mut s;
                    match handle_stream(send, recv, callback.clone()).await {
                        Ok(_) => {},
                        Err(e) => {
                            return Err(anyhow!(e));
                        }
                    }
                 }
            }
        }
    }.instrument(make_span(&conn))
    .await
}


async fn handle_stream<F>(send: &mut quinn::SendStream, recv: &mut quinn::RecvStream, callback: Arc<F>) -> anyhow::Result<()> where F: Fn(Vec<u8>)-> anyhow::Result<Vec<u8>>  + Send + Sync{
    let mut req = recv.read_to_end(64 * 1024).await.or_else(|e| {
        log::error!("RecvStream::read_to_end() failed. {e:?}");
        return Err(anyhow!(e))
    })?;

    let req_msg = rmp_serde::from_slice::<serde_json::Value>(&mut &req.as_mut_slice()[..] )?;

    log::trace!("request : {:?}", req_msg);
    
    let response = callback(req)?;

    send.write_all(&response).await.or_else(|e| { 
        log::error!("write error. {e:?}");
        Err(anyhow!(e))}
    )?;
    send.finish().or_else(|e| { 
        log::error!("finish error. {e:?}");
        Err(anyhow!(e))
    })
}
