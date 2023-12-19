use utoipa::OpenApi;

#[utoipa::path(
    get,
    path = "/api/system/profile_full",
    responses(
        (status = 200, description = "System")
    ),
    tag = "system",
)]
pub async fn profile_handler_dummy() {
}

#[derive(OpenApi)]
#[openapi(
    paths(
        profile_handler_dummy,
    ),
    components(schemas(
    ))
)]
pub struct ApiDoc;