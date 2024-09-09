use utoipa::OpenApi;

use crate::prelude::*;
use super::IdentifierQuery;
use axum::{extract::Query, Json};


#[utoipa::path(
    get,
    path = "/api/container_process/profile_full",
    params(
        IdentifierQuery
    ),
    responses(
        (status = 200, description = "System")
    ),
    tag = "universal.container_process",
)]
pub fn profile_handler_dummy(
    _query: Query<IdentifierQuery>,) {
}

#[utoipa::path(
    get,
    path = "/api/container_process/list",
    responses(
        (status = 200, description = "System")
    ),
    tag = "universal.container_process",
)]
pub fn list_dummy() {
}

#[utoipa::path(
    patch,
    path = "/api/container_process/call",
    params(
        IdentifierQuery
    ),
    request_body = Value,
    responses(
        (status = 200, description = "System")
    ),
    tag = "universal.container_process",
)]
pub fn call_dummy(
_query: Query<IdentifierQuery>,
Json(_body): Json<Value>) {
}


#[utoipa::path(
    patch,
    path = "/api/container_process/execute",
    params(
        IdentifierQuery
    ),
    request_body = Value,
    responses(
        (status = 200, description = "System")
    ),
    tag = "universal.container_process",
)]
pub fn execute_dummy(
_query: Query<IdentifierQuery>,
Json(_body): Json<Value>) {
}


#[utoipa::path(
    delete,
    path = "/api/container_process/destroy",
    params(
        IdentifierQuery
    ),
    responses(
        (status = 200, description = "System")
    ),
    tag = "universal.container_process",
)]
pub fn delete_dummy(
_query: Query<IdentifierQuery>) {
}

#[derive(OpenApi)]
#[openapi(
    paths(
        profile_handler_dummy,
        list_dummy,
        call_dummy,
        execute_dummy,
        delete_dummy,
    ),
    components(schemas(
    ))
)]
pub struct ApiDoc;