use utoipa::OpenApi;

use super::IdentifierQuery;

use axum::extract::Query;

use axum::Json;

use crate::Value;


#[utoipa::path(
    patch,
    path = "/api/execution_context/start",
    params(
        IdentifierQuery
    ),
    responses(
        (status = 200, description = "System")
    ),
    tag = "universal.execution_context",
)]
pub fn start_dummy(
    _query: Query<IdentifierQuery>,
    Json(_body): Json<Value>) {
}

#[utoipa::path(
    patch,
    path = "/api/execution_context/stop",
    params(
        IdentifierQuery
    ),
    responses(
        (status = 200, description = "System")
    ),
    tag = "universal.execution_context",
)]
pub fn stop_dummy(
    _query: Query<IdentifierQuery>,
    Json(_body): Json<Value>) {
}


#[utoipa::path(
    get,
    path = "/api/execution_context/profile_full",
    params(
        IdentifierQuery
    ),
    responses(
        (status = 200, description = "System")
    ),
    tag = "universal.execution_context",
)]
pub fn profile_handler_dummy(
    _query: Query<IdentifierQuery>,) {
}


#[utoipa::path(
    get,
    path = "/api/execution_context/get_state",
    params(
        IdentifierQuery
    ),
    responses(
        (status = 200, description = "System")
    ),
    tag = "universal.execution_context",
)]
pub fn get_state_dummy(
    _query: Query<IdentifierQuery>,) {
}

#[utoipa::path(
    get,
    path = "/api/execution_context/list",
    responses(
        (status = 200, description = "System")
    ),
    tag = "universal.execution_context",
)]
pub fn list_dummy() {
}

#[derive(OpenApi)]
#[openapi(
    paths(
        profile_handler_dummy,
        list_dummy,
        get_state_dummy,
        start_dummy,
        stop_dummy,
    ),
    components(schemas(
    ))
)]
pub struct ApiDoc;