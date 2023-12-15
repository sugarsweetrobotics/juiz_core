

use std::sync::{Mutex, Arc};

use axum::{extract::{State, Path, Query}, response::IntoResponse, Json, Router, routing};

use juiz_core::{brokers::crud_broker::{CRUDBroker, update_class, read_class, create_class, delete_class}, Value};

use super::{IdentifierQuery, json_wrap, query_to_map};


#[utoipa::path(
    post,
    path = "/api/{class_name}/{function_name}",
    params(
        IdentifierQuery
    ),
    request_body = Value,
    responses(
        (status = 200, description = "Post object parameter", body = [String])
    )
)]
pub async fn object_post_handler(
    Path((class_name, function_name)): Path<(String, String)>,
    query: Query<IdentifierQuery>,
    State(crud_broker): State<Arc<Mutex<CRUDBroker>>>, 
    Json(body): Json<Value>,
) -> impl IntoResponse {
    json_wrap(create_class(&crud_broker, class_name.as_str(), function_name.as_str(), body, query_to_map(&query)))
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
    )
)]
pub async fn object_patch_handler(
    Path((class_name, function_name)): Path<(String, String)>,
    query: Query<IdentifierQuery>,
    State(crud_broker): State<Arc<Mutex<CRUDBroker>>>, 
    Json(body): Json<Value>,
) -> impl IntoResponse {
    json_wrap(update_class(&crud_broker, class_name.as_str(), function_name.as_str(), body, query_to_map(&query)))
}


#[utoipa::path(
    get,
    path = "/api/{class_name}/{function_name}",
    params(
        IdentifierQuery
    ),
    responses(
        (status = 200, description = "Get object parameter", body = [String])
    )
)]
pub async fn object_get_handler(
    //log::trace!("HTTPBroker/object_get_handler() called");
    Path((class_name, function_name)): Path<(String, String)>,
    query: Query<IdentifierQuery>,
    State(crud_broker): State<Arc<Mutex<CRUDBroker>>>, 
) -> impl IntoResponse {
    json_wrap(read_class(&crud_broker, class_name.as_str(), function_name.as_str(), query_to_map(&query)))
}

#[utoipa::path(
    delete,
    path = "/api/{class_name}/{function_name}",
    params(
        IdentifierQuery
    ),
    responses(
        (status = 200, description = "Delete object parameter", body = [String])
    )
)]
pub async fn object_delete_handler(
    //log::trace!("HTTPBroker/object_get_handler() called");
    Path((class_name, function_name)): Path<(String, String)>,
    query: Query<IdentifierQuery>,
    State(crud_broker): State<Arc<Mutex<CRUDBroker>>>, 
) -> impl IntoResponse {
    json_wrap(delete_class(&crud_broker, class_name.as_str(), function_name.as_str(), query_to_map(&query)))
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