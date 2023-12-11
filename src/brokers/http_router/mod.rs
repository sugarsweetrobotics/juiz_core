use std::sync::{Mutex, Arc};

use axum::{Router, routing::get, Json};
use crate::{jvalue, utils::juiz_lock};

use super::crud_broker::CRUDBroker;



async fn get_profile_full() -> Json<serde_json::Value> {
    Json(jvalue!({"system": "hoge"}))
}


pub fn system(crud_broker: Arc<Mutex<CRUDBroker>>) -> Router {
    Router::new()
        .route("/profile_full", get({
            Json(juiz_lock(&crud_broker).unwrap().read("system/profile_full").unwrap())
        }))
}