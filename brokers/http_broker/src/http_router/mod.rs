use std::collections::HashMap;

use std::sync::{Mutex, Arc};
use juiz_core::brokers::CRUDBroker;

use utoipa::OpenApi;
use axum::Router;
use utoipa_swagger_ui::SwaggerUi;
use axum::{response::IntoResponse, http::StatusCode, Json, extract::Query};
use serde::Deserialize;
use utoipa::IntoParams;

use juiz_core::{jvalue, JuizResult, Value};

pub mod any;
pub mod system;
pub mod process;
pub mod any_process;
pub mod container;
pub mod container_process;
pub mod broker;
pub mod execution_context;
pub mod connection;

#[derive(Deserialize, IntoParams, Debug)]
pub struct IdentifierQuery {
    identifier: Option<String>,
}

pub fn query_to_map(query: &Query<IdentifierQuery>) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    match query.identifier.clone() {
        None => {},
        Some(v) => {
            map.insert("identifier".to_string(), v);
        }
    }
    map
}

pub fn json_wrap(result: JuizResult<Value>) -> impl IntoResponse {
    match result {
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(
                jvalue!({
                    "message": format!("Internal Server Error:  {:#}, {:}", e, e.to_string())
                }))).into_response()
        },
        Ok(v) => {
            Json(v).into_response()
        }
    }
}




#[derive(OpenApi)]
#[openapi(
    tags(),
    paths(
        
    ),
    components(schemas(
    ))
)]
struct ApiDoc;

pub fn app_new(crud_broker: Arc<Mutex<CRUDBroker>>) -> Router {
    let mut api = ApiDoc::openapi();
    api.merge(ApiDoc::openapi());
    api.merge(system::ApiDoc::openapi());
    api.merge(process::ApiDoc::openapi());
    api.merge(container::ApiDoc::openapi());
    api.merge(container_process::ApiDoc::openapi());
    api.merge(broker::ApiDoc::openapi());
    api.merge(execution_context::ApiDoc::openapi());
    api.merge(connection::ApiDoc::openapi());
    
    Router::new()
            .merge(SwaggerUi::new("/docs")
            .url("/api-docs/openapi.json", api))
            .nest("/api/", any::object_router(crud_broker.clone()))

}