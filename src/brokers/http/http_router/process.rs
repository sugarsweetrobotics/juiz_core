use crate::Value;
use utoipa::OpenApi;

use super::IdentifierQuery;

use axum::{extract::Query, Json};
#[utoipa::path(
    get,
    path = "/api/process/profile_full",
    params(
        IdentifierQuery
    ),
    responses(
        (status = 200, description = "System")
    ),
    tag = "universal.process",
)]
pub fn profile_handler_dummy(
    _query: Query<IdentifierQuery>,) {
}

#[utoipa::path(
    get,
    path = "/api/process/list",
    responses(
        (status = 200, description = "System")
    ),
    tag = "universal.process",
)]
pub fn list_dummy() {
}

#[utoipa::path(
    patch,
    path = "/api/process/call",
    params(
        IdentifierQuery
    ),
    request_body = Value,
    responses(
        (status = 200, description = "System")
    ),
    tag = "universal.process",
)]
pub fn call_dummy(
_query: Query<IdentifierQuery>,
Json(_body): Json<Value>) {
}


#[utoipa::path(
    patch,
    path = "/api/process/execute",
    params(
        IdentifierQuery
    ),
    request_body = Value,
    responses(
        (status = 200, description = "System")
    ),
    tag = "universal.process",
)]
pub fn execute_dummy(
_query: Query<IdentifierQuery>,
Json(_body): Json<Value>) {
}


#[derive(OpenApi)]
#[openapi(
    paths(
        profile_handler_dummy,
        list_dummy,
        call_dummy,
        execute_dummy,
    ),
    components(schemas(
    ))
)]
pub struct ApiDoc;