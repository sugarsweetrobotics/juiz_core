use std::collections::HashMap;

use axum::{response::IntoResponse, http::{StatusCode, Response}, Json, extract::Query};
use serde::Deserialize;
use utoipa::IntoParams;

use crate::{JuizResult, Value, JuizError};

pub mod any;
//pub mod system;
// pub mod process;
//pub mod container;

#[derive(Deserialize, IntoParams)]
pub struct IdentifierQuery {
    identifier: Option<String>,
}

pub fn query_to_map(query: &Query<IdentifierQuery>) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    match query.identifier.clone() {
        None => {},
        Some(v) => {
            map.insert("identifier".to_string(), v);
        }
    }
    map
}

pub fn json_wrap(result: JuizResult<Value>) -> impl IntoResponse {
    match result {
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(format!("Internal Server Error: {}", e.to_string()))).into_response()
        },
        Ok(v) => {
            Json(v).into_response()
        }
    }
}