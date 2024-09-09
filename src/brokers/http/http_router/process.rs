
use crate::prelude::*;
use utoipa::OpenApi;

use super::IdentifierQuery;
use axum::{extract::Query, Json};

#[utoipa::path(
    post,
    path = "/api/process/create",
    request_body = Value,
    responses(
        (status = 200, description = "System")
    ),
    tag = "universal.process",
)]
pub fn create_dummy(
_query: Query<IdentifierQuery>,
Json(_body): Json<Value>) {
}

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

#[utoipa::path(
    delete,
    path = "/api/process/destroy",
    params(
        IdentifierQuery
    ),
    responses(
        (status = 200, description = "System")
    ),
    tag = "universal.process",
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
        create_dummy,
    ),
    components(schemas(
    ))
)]
pub struct ApiDoc;