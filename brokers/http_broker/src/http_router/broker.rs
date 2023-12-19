use utoipa::OpenApi;

use super::IdentifierQuery;


#[utoipa::path(
    get,
    path = "/api/broker/profile_full",
    params(
        IdentifierQuery
    ),
    responses(
        (status = 200, description = "System")
    ),
    tag = "broker",
)]
pub fn profile_handler_dummy() {
}

#[utoipa::path(
    get,
    path = "/api/broker/list",
    responses(
        (status = 200, description = "System")
    ),
    tag = "broker",
)]
pub fn list_dummy() {
}

#[derive(OpenApi)]
#[openapi(
    paths(
        profile_handler_dummy,
        list_dummy,
    ),
    components(schemas(
    ))
)]
pub struct ApiDoc;