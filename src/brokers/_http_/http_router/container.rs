use std::{sync::{Mutex, Arc}, collections::HashMap};

use axum::{extract::{State, Path, Query}, response::IntoResponse, Json, Router, routing};

use crate::{jvalue, brokers::crud_broker::{CRUDBroker, update_class}, Value};

use super::{IdentifierQuery, json_wrap};

#[utoipa::path(
    get,
    path = "/container/{function[call, execute]}",
    params(
        IdentifierQuery
    ),
    request_body = Value,
    responses(
        (status = 200, description = "Get System Profile", body = [String])
    )
)]
pub async fn container_update_handler(
    Path(function_name): Path<String>,
    query: Query<IdentifierQuery>,
    State(crud_broker): State<Arc<Mutex<CRUDBroker>>>, 
    Json(body): Json<Value>,
) -> impl IntoResponse {
    // process_update_handler function.
    let mut map : HashMap<String, String> = HashMap::new();
    let id = query_get_id(query)?;
    map.insert("identifier".to_string(),  query.identifier.clone().unwrap());
    json_wrap(update_class(&crud_broker, "container", function_name.as_str(), body, map))
}

pub fn container_router(crud_broker: Arc<Mutex<CRUDBroker>>) -> Router {
    Router::new()
        .route("/container/:function", routing::post(container_update_handler))
        .with_state(Arc::clone(&crud_broker))
}