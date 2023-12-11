use std::{sync::{Arc, Mutex, atomic::AtomicBool}, thread::Builder, time::Duration};

use crate::{Broker, BrokerProxy, JuizResult};

use super::crud_broker::CRUDBroker;


use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};

use serde::{Deserialize, Serialize};



pub struct HTTPBroker {
    crud_broker: Arc<Mutex<CRUDBroker>>,
    thread_handle: Option<tokio::task::JoinHandle<()>>,
    //receiver: Arc<Mutex<mpsc::Receiver<Value>>>,
    //sender: Arc<Mutex<mpsc::Sender<Value>>>,
    //core_broker: Arc<Mutex<dyn BrokerProxy>>,
    end_flag: Arc<Mutex<AtomicBool>>,
}

impl HTTPBroker {
    pub fn new(core_broker: Arc<Mutex<dyn BrokerProxy>>) -> JuizResult<Arc<Mutex<HTTPBroker>>> {
        Ok(Arc::new(Mutex::new(HTTPBroker{
            crud_broker: CRUDBroker::new(core_broker)?,
            thread_handle: None,
            end_flag: Arc::new(Mutex::new(AtomicBool::from(false)))
        })))
    }
}

async fn root() -> &'static str {
    "Hello, World!"
}

impl Broker for HTTPBroker {
    fn type_name(&self) -> &str {
        "http"
    }

    fn start(&mut self) -> JuizResult<()> {
        
        log::trace!("HTTPBroker::start() called");
        let crud_broker = Arc::clone(&self.crud_broker);
        let join_handle = tokio::task::spawn(
            async move  {
                let app = Router::new()
                  .nest("/system/", super::http_router::system(crud_broker.clone()));

                let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
                axum::serve(listener, app).await.unwrap();
                println!("HTTPBroker::routine() end!!!");
            }
        );
        //self.thread_builder = Some(thread_builder);
        self.thread_handle = Some(join_handle);

        Ok(())
    }

    fn stop(&mut self) -> crate::JuizResult<()> {
        self.thread_handle.take().unwrap().abort();
        Ok(())
    }
}