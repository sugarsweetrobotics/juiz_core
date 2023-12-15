
use std::sync::{Mutex, Arc};
use juiz_core::brokers::CRUDBroker;

use utoipa::OpenApi;
use axum::Router;
use utoipa_swagger_ui::SwaggerUi;

use super::any;
#[derive(OpenApi)]
#[openapi(
    paths(
        any::object_get_handler,
        any::object_patch_handler,
    ),
    components(schemas(
    ))
)]
struct ApiDoc;

pub fn app_new(crud_broker: Arc<Mutex<CRUDBroker>>) -> Router {
    Router::new()
            .merge(SwaggerUi::new("/swagger-ui")
            .url("/api-docs/openapi.json", ApiDoc::openapi()))
            .nest("/api/", any::object_router(crud_broker.clone()))

}