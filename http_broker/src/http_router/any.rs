

use std::sync::{Mutex, Arc};
use axum::{extract::{State, Path, Query}, response::IntoResponse, Json, Router, routing};

use juiz_core::{brokers::crud_broker::{CRUDBroker, update_class, read_class, create_class, delete_class}, Value};

use super::{IdentifierQuery, json_wrap, query_to_map};
use utoipa::OpenApi;

#[utoipa::path(
    post,
    path = "/api/{class_name}/{function_name}",
    params(
        IdentifierQuery
    ),
    request_body = Value,
    responses(
        (status = 200, description = "Post object parameter", body = [String])
    ),
    tag = "any",
)]
pub async fn object_post_handler(
    Path((class_name, function_name)): Path<(String, String)>,
    query: Query<IdentifierQuery>,
    State(crud_broker): State<Arc<Mutex<CRUDBroker>>>, 
    Json(body): Json<Value>,
) -> impl IntoResponse {
    let map = query_to_map(&query);
    log::trace!("HTTPBroker/object_post_handler({class_name}, {function_name}, {body}, {map:?}) called");
    json_wrap(create_class(&crud_broker, class_name.as_str(), function_name.as_str(), body, map))
}

#[utoipa::path(
    patch,
    path = "/api/{class_name}/{function_name}",
    params(
        IdentifierQuery
    ),
    request_body = Value,
    responses(
        (status = 200, description = "Get object parameter", body = [String])
    ),
    tag = "any",
)]
pub async fn object_patch_handler(
    Path((class_name, function_name)): Path<(String, String)>,
    query: Query<IdentifierQuery>,
    State(crud_broker): State<Arc<Mutex<CRUDBroker>>>, 
    Json(body): Json<Value>,
) -> impl IntoResponse {

    let map = query_to_map(&query);
    log::trace!("HTTPBroker/object_patch_handler({class_name}, {function_name}, {body}, {map:?}) called");
    json_wrap(update_class(&crud_broker, class_name.as_str(), function_name.as_str(), body, map))
}


#[utoipa::path(
    get,
    path = "/api/{class_name}/{function_name}",
    params(
        IdentifierQuery
    ),
    responses(
        (status = 200, description = "Get object parameter", body = [String])
    ),
    tag = "any",
)]
pub async fn object_get_handler(
    Path((class_name, function_name)): Path<(String, String)>,
    query: Query<IdentifierQuery>,
    State(crud_broker): State<Arc<Mutex<CRUDBroker>>>, 
) -> impl IntoResponse {
    let map = query_to_map(&query);
    log::trace!("HTTPBroker/object_get_handler({class_name}, {function_name}, {map:?}) called");
    json_wrap(read_class(&crud_broker, class_name.as_str(), function_name.as_str(), map))
}

#[utoipa::path(
    delete,
    path = "/api/{class_name}/{function_name}",
    params(
        IdentifierQuery
    ),
    responses(
        (status = 200, description = "Delete object parameter", body = [String])
    ),
    tag = "any"
)]
pub async fn object_delete_handler(
    Path((class_name, function_name)): Path<(String, String)>,
    query: Query<IdentifierQuery>,
    State(crud_broker): State<Arc<Mutex<CRUDBroker>>>, 
) -> impl IntoResponse {
    let map = query_to_map(&query);
    log::trace!("HTTPBroker/object_delete_handler({class_name}, {function_name}, {map:?}) called");
    json_wrap(delete_class(&crud_broker, class_name.as_str(), function_name.as_str(), map))
}

pub fn object_router(crud_broker: Arc<Mutex<CRUDBroker>>) -> Router {
    Router::new()
        .route("/:class_name/:function_name", 
                routing::patch(object_patch_handler)
                .get(object_get_handler)
                .delete(object_delete_handler)
                .post(object_post_handler)
        )
        .with_state(Arc::clone(&crud_broker))
} 





#[derive(OpenApi)]
#[openapi(
    paths(
        object_get_handler,
        object_patch_handler,
        object_delete_handler,
        object_post_handler,
    ),
    components(schemas(
    ))
)]
pub struct ApiDoc;