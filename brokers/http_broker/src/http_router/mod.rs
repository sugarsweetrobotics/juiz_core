use std::collections::HashMap;

use std::sync::{Mutex, Arc};
use juiz_core::brokers::CRUDBroker;

use juiz_core::processes::capsule::{unwrap_arc_capsule, Capsule};

use juiz_core::processes::capsule_to_value;
use utoipa::OpenApi;
use axum::Router;
use utoipa_swagger_ui::SwaggerUi;
use axum::{response::{Response, IntoResponse}, body::Body, http::StatusCode, Json, extract::Query};
use serde::Deserialize;
use utoipa::IntoParams;

use juiz_core::{jvalue, JuizResult, Value};

pub mod any;
pub mod system;
pub mod process;
pub mod any_process;
pub mod container;
pub mod container_process;
pub mod broker;
pub mod execution_context;
pub mod connection;
use cv_convert::TryFromCv;

#[derive(Deserialize, IntoParams, Debug)]
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

pub fn json_wrap(result: JuizResult<Arc<Mutex<Capsule>>>) -> impl IntoResponse {
    match result {
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(
                jvalue!({
                    "message": format!("Internal Server Error:  {:#}, {:}", e, e.to_string())
                }))).into_response()
        },
        Ok(arc) => {
            
            let result = capsule_to_value(arc);
            if result.is_err() {
                let e = result.err().unwrap();
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(
                jvalue!({
                    "message": format!("Internal Server Error:  {:#}, {:}", e, e.to_string())
                }))).into_response()
            }
            Json::<Value>(result.unwrap()).into_response()
        }
    }
}

pub fn json_output_wrap(result: JuizResult<Arc<Mutex<Capsule>>>) -> impl IntoResponse {
    //log::trace!("json_output_wrap() called");
    if result.is_err() {
        let e = result.err().unwrap();
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(
            jvalue!({
                "message": format!("Internal Server Error:  {:#}, {:}", e, e.to_string())
            }))).into_response()
    }
    match unwrap_arc_capsule(result.unwrap()) {
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(
                jvalue!({
                    "message": format!("Internal Server Error:  {:#}, {:}", e, e.to_string())
                }))).into_response()
        },
        Ok(v) => {
            if v.is_value() {
                let result = v.as_value();
                Json(result.unwrap()).into_response()
            } else if v.is_mat() {
                let result = v.as_mat();
                //Json(jvalue!({"message": "ERROR.this is image"})).into_response()
                let img = image::RgbImage::try_from_cv(result.unwrap()).unwrap();

                use image::ImageFormat;

                use std::io::{BufWriter, Cursor};

                let mut buffer = BufWriter::new(Cursor::new(Vec::new()));
                img.write_to(&mut buffer, ImageFormat::Png).unwrap();

                //Json(jvalue!({"message": "ERROR.this is image"})).into_response()
                
                let bytes: Vec<u8> = buffer.into_inner().unwrap().into_inner(); 
                
                let response =  Response::builder()
                    .extension("png")
                    .header("Content-Type", "image/png")
                    .status(StatusCode::OK)
                    .body(Body::from(bytes)).unwrap();
                //response.into_response()
                response.into_response()
            } else {
                Json(jvalue!({})).into_response()
            }
        }
    }
}





#[derive(OpenApi)]
#[openapi(
    tags(),
    paths(
        
    ),
    components(schemas(
    ))
)]
struct ApiDoc;

pub fn app_new(crud_broker: Arc<Mutex<CRUDBroker>>) -> Router {
    let mut api = ApiDoc::openapi();
    api.merge(ApiDoc::openapi());
    api.merge(system::ApiDoc::openapi());
    api.merge(process::ApiDoc::openapi());
    api.merge(container::ApiDoc::openapi());
    api.merge(container_process::ApiDoc::openapi());
    api.merge(broker::ApiDoc::openapi());
    api.merge(execution_context::ApiDoc::openapi());
    api.merge(connection::ApiDoc::openapi());
    
    Router::new()
            .merge(SwaggerUi::new("/docs")
            .url("/api-docs/openapi.json", api))
            .nest("/api/", any::object_router(crud_broker.clone()))

}