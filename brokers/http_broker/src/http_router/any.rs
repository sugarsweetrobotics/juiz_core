

use std::sync::{Mutex, Arc};
use axum::{response::IntoResponse, extract::{Path, Query, State}, routing, Json, Router};

use juiz_core::{brokers::crud_broker::CRUDBroker, processes::capsule::CapsuleMap, utils::juiz_lock, Value};

use super::{IdentifierQuery, json_output_wrap, query_to_map};
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
    tag = "universal.any",
)]
pub async fn object_post_handler(
    Path((class_name, function_name)): Path<(String, String)>,
    query: Query<IdentifierQuery>,
    State(crud_broker): State<Arc<Mutex<CRUDBroker>>>, 
    Json(body): Json<Value>,
) -> impl IntoResponse {
    let map = query_to_map(&query);
    log::trace!("HTTPBroker/object_post_handler({class_name}, {function_name}, {body}, {map:?}) called");
    //json_output_wrap(create_class(&crud_broker, class_name.as_str(), function_name.as_str(), body, map))
    json_output_wrap(juiz_lock(&crud_broker).and_then(|cb| {
        cb.create_class(construct_capsule_map(body_to_capsule_map(body)?, "CREATE", class_name.as_str(), function_name.as_str(), query))
    }))
}

fn body_to_capsule_map(body: Value) -> Result<CapsuleMap, anyhow::Error> {
    body.try_into()
}


fn construct_capsule_map(mut capsule_map: CapsuleMap, method_name: &str, class_name: &str, function_name: &str, query: Query<IdentifierQuery>) -> CapsuleMap {
    capsule_map.set_param("method_name", method_name);
    capsule_map.set_param("class_name", class_name);
    capsule_map.set_param("function_name", function_name);
    match query.identifier.clone() {
        None => {},
        Some(v) => {
            capsule_map.set_param("identifier", v.as_str());
        }
    }
    capsule_map
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
    tag = "universal.any",
)]
pub async fn object_patch_handler(
    Path((class_name, function_name)): Path<(String, String)>,
    query: Query<IdentifierQuery>,
    State(crud_broker): State<Arc<Mutex<CRUDBroker>>>, 
    Json(body): Json<Value>,
) -> impl IntoResponse {
    let map = query_to_map(&query);
    log::trace!("debug:{:?}", query);
    log::trace!("HTTPBroker/object_patch_handler({class_name}, {function_name}, {body}, {map:?}) called");
    //let result = update_class(&crud_broker, class_name.as_str(), function_name.as_str(), body.try_into().unwrap(), map);
    //let method_name = "UPDATE";
    json_output_wrap(juiz_lock(&crud_broker).and_then(|cb| {
        cb.update_class(construct_capsule_map(body_to_capsule_map(body)?, "UPDATE", class_name.as_str(), function_name.as_str(), query))
    }))
    /*
    let result = update_class(&crud_broker, construct_capsule_map(body_to_capsule_map(body), method_name, class_name.as_str(), function_name.as_str(), query));
    json_output_wrap(result)
    */
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
    tag = "universal.any",
)]
pub async fn object_get_handler(
    Path((class_name, function_name)): Path<(String, String)>,
    query: Query<IdentifierQuery>,
    State(crud_broker): State<Arc<Mutex<CRUDBroker>>>, 
) -> impl IntoResponse {
    log::trace!("debug:{:?}", query);
    let map = query_to_map(&query);
    log::trace!("HTTPBroker/object_get_handler({class_name}, {function_name}, {map:?}) called");
    //json_output_wrap(read_class(&crud_broker, class_name.as_str(), function_name.as_str(), map))
    //let method_name = "READ";
    //json_output_wrap(read_class(&crud_broker, construct_capsule_map(CapsuleMap::new(), method_name, class_name.as_str(), function_name.as_str(), query)))
    json_output_wrap(juiz_lock(&crud_broker).and_then(|cb| {
        cb.read_class(construct_capsule_map(CapsuleMap::new(), "READ", class_name.as_str(), function_name.as_str(), query))
    }))
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
    tag = "universal.any"
)]
pub async fn object_delete_handler(
    Path((class_name, function_name)): Path<(String, String)>,
    query: Query<IdentifierQuery>,
    State(crud_broker): State<Arc<Mutex<CRUDBroker>>>, 
) -> impl IntoResponse {
    let map = query_to_map(&query);
    log::trace!("HTTPBroker/object_delete_handler({class_name}, {function_name}, {map:?}) called");
    //json_wrap(delete_class(&crud_broker, class_name.as_str(), function_name.as_str(), map))
    //let method_name = "DELETE";
    //json_wrap(delete_class(&crud_broker, construct_capsule_map(CapsuleMap::new(), method_name, class_name.as_str(), function_name.as_str(), query)))
    json_output_wrap(juiz_lock(&crud_broker).and_then(|cb| {
        cb.create_class(construct_capsule_map(CapsuleMap::new(), "DELETE", class_name.as_str(), function_name.as_str(), query))
    }))
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