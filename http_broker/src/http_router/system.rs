use std::sync::{Mutex, Arc};

use axum::{extract::{State, Path}, response::IntoResponse, Json, Router, routing};

use crate::{jvalue, brokers::crud_broker::{CRUDBroker, read_class}, utils::juiz_lock, Value, JuizResult};

use super::json_wrap;

#[utoipa::path(
    get,
    path = "/system/{function}",
    responses(
        (status = 200, description = "Get System Profile", body = [String])
    )
)]
pub async fn profile_handler(
    Path(function_name): Path<String>,
    State(crud_broker): State<Arc<Mutex<CRUDBroker>>>
) -> impl IntoResponse {
    json_wrap(read_class(&crud_broker, "system", function_name.as_str()))
}

pub fn system_router(crud_broker: Arc<Mutex<CRUDBroker>>) -> Router {
    Router::new()
        //.route("/:function_name", get(system_get_handler))
        .route("/system/:function", routing::get(profile_handler))
        .with_state(Arc::clone(&crud_broker))
}