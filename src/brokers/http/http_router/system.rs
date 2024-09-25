use utoipa::OpenApi;

use axum::{extract::Query, Json};
use super::{IdentifierQuery, PathQuery, Value};

#[allow(unused)]
#[utoipa::path(
    get,
    path = "/api/system/profile_full",
    responses(
        (status = 200, description = "System")
    ),
    tag = "universal.system",
)]
pub async fn profile_handler_dummy(){
}


#[allow(unused)]
#[utoipa::path(
    get,
    path = "/api/system/uuid",
    responses(
        (status = 200, description = "System")
    ),
    tag = "universal.system",
)]
pub async fn uuid_dummy(){
}


#[allow(unused)]
#[utoipa::path(
    get,
    path = "/api/system/filesystem_list",
    params(
        PathQuery
    ),
    responses(
        (status = 200, description = "System")
    ),
    tag = "universal.system",
)]
pub fn fslist_handler_dummy(
    _query: Query<PathQuery>,
){
}

#[allow(unused)]
#[utoipa::path(
    patch,
    path = "/api/system/add_subsystem",
    params(
        IdentifierQuery
    ),
    request_body = Value,
    responses(
        (status = 200, description = "System")
    ),
    tag = "universal.system",
)]
pub fn add_subsystem_dummy(
_query: Query<IdentifierQuery>,
Json(_body): Json<Value>) {
}

#[derive(OpenApi)]
#[openapi(
    paths(
        profile_handler_dummy,
        uuid_dummy,
        fslist_handler_dummy,
        add_subsystem_dummy,
    ),
    components(schemas(
    ))
)]
pub struct ApiDoc;