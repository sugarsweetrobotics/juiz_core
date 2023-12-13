use std::{sync::{Arc, Mutex, atomic::AtomicBool}, thread::Builder, time::Duration};

use crate::{jvalue, Broker, BrokerProxy, JuizResult, JuizObject, Identifier, identifier::identifier_new, object::{ObjectCore, JuizObjectClass, JuizObjectCoreHolder}};

use super::crud_broker::CRUDBroker;

use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};

use serde::{Deserialize, Serialize};
use utoipa_swagger_ui::SwaggerUi;

pub struct HTTPBroker {
    core: ObjectCore,
    crud_broker: Arc<Mutex<CRUDBroker>>,
    thread_handle: Option<tokio::task::JoinHandle<()>>,
    end_flag: Arc<Mutex<AtomicBool>>,
}

impl HTTPBroker {
    pub fn new(core_broker: Arc<Mutex<dyn BrokerProxy>>, object_name: &str) -> JuizResult<Arc<Mutex<HTTPBroker>>> {
        let type_name = "http";
        Ok(Arc::new(Mutex::new(HTTPBroker{
            core: ObjectCore::create(JuizObjectClass::Broker("HTTPBroker"), type_name, object_name), 
            crud_broker: CRUDBroker::new(core_broker)?,
            thread_handle: None,
            end_flag: Arc::new(Mutex::new(AtomicBool::from(false)))
        })))
    }
}

impl JuizObjectCoreHolder for HTTPBroker {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}
impl JuizObject for HTTPBroker {}

use super::http_router::*;
#[derive(OpenApi)]
#[openapi(
    paths(
        any::object_get_handler,
        any::object_patch_handler,
    ),
    components(schemas(
    ))
)]
struct ApiDoc;

impl Broker for HTTPBroker {
    fn start(&mut self) -> JuizResult<()> {
        
        log::trace!("HTTPBroker::start() called");
        let crud_broker = Arc::clone(&self.crud_broker);
        let join_handle = tokio::task::spawn(
            async move  {
                let app = Router::new()
                    .merge(SwaggerUi::new("/swagger-ui")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()))
                    .nest("/api/", any::object_router(crud_broker.clone()))
                    ;

                let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
                axum::serve(listener, app).await.unwrap();
                println!("HTTPBroker::routine() end!!!");
            }
        );
        self.thread_handle = Some(join_handle);

        Ok(())
    }

    fn stop(&mut self) -> crate::JuizResult<()> {
        self.thread_handle.take().unwrap().abort();
        Ok(())
    }
}