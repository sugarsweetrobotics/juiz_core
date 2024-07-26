use utoipa::OpenApi;

use axum::extract::Query;
use super::PathQuery;

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

#[derive(OpenApi)]
#[openapi(
    paths(
        profile_handler_dummy,
        fslist_handler_dummy,
    ),
    components(schemas(
    ))
)]
pub struct ApiDoc;