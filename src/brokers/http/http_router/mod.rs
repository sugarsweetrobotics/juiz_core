
use std::collections::HashMap;

use std::path::PathBuf;
use std::sync::{Mutex, Arc};
// use crate::prelude::*;
use crate::brokers::CRUDBroker;
use crate::value::capsule_to_value;

use axum::http::HeaderValue;

#[cfg(feature="opencv4")]
use opencv::{core::{Vector, VectorToVec}, imgcodecs::imencode};

use tower_http::services::ServeDir;
use utoipa::{IntoParams, OpenApi};
use utoipa::openapi::{path::OperationBuilder, request_body::RequestBodyBuilder, ContentBuilder, PathItem, PathItemType};
use utoipa_swagger_ui::SwaggerUi;

use axum::{Router, response::{Response, IntoResponse}, body::Body, http::StatusCode, Json, extract::Query};
use serde::Deserialize;

use crate::prelude::*;

pub mod any;
pub mod system;
pub mod process;
pub mod topic;
pub mod any_process;
pub mod container;
pub mod container_process;
pub mod broker;
pub mod execution_context;
pub mod connection;
// use cv_convert::TryFromCv;

#[derive(Deserialize, IntoParams, Debug)]
pub struct IdentifierQuery {
    identifier: Option<String>,
}

#[derive(Deserialize, IntoParams, Debug)]
pub struct PathQuery {
    path: Option<String>,
}

#[allow(unused)]
#[derive(Deserialize, IntoParams, Debug)]
pub struct RecursiveQuery {
    recursive: Option<String>,
}

#[derive(Deserialize, IntoParams, Debug)]
pub struct IdAndRecurQuery {
    identifier: Option<String>,
    recursive: Option<String>,
}

#[derive(Deserialize, IntoParams, Debug)]
pub struct IdAndUuidQuery {
    identifier: Option<String>,
    system_uuid: Option<String>,
}

#[allow(unused)]
#[derive(Deserialize, IntoParams, Debug)]
pub struct TopicNameAndUuidQuery {
    topic_name: Option<String>,
    system_uuid: Option<String>,
}

#[derive(Deserialize, IntoParams, Debug)]
pub struct FullQuery {
    identifier: Option<String>,
    path: Option<String>,
    recursive: Option<String>,
    system_uuid: Option<String>,
    topic_name: Option<String>,
}

#[allow(unused)]
pub fn id_query_to_map(query: &Query<IdentifierQuery>) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    match query.identifier.clone() {
        None => {},
        Some(v) => {
            map.insert("identifier".to_owned(), v);
        }
    }
    map
}

#[allow(unused)]
pub fn path_query_to_map(query: &Query<PathQuery>) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    match query.path.clone() {
        None => {},
        Some(v) => {
            map.insert("path".to_owned(), v);
        }
    }
    map
}


#[allow(unused)]
pub fn id_and_recur_query_to_map(query: &Query<IdAndRecurQuery>) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    match query.identifier.clone() {
        None => {},
        Some(v) => {
            map.insert("identifier".to_owned(), v);
        }
    }
    match query.recursive.clone() {
        None => {},
        Some(v) => {
            map.insert("recursive".to_owned(), v);
        }
    }
    map
}

#[allow(unused)]
pub fn id_and_uuid_query_to_map(query: &Query<IdAndUuidQuery>) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    match query.identifier.clone() {
        None => {},
        Some(v) => {
            map.insert("identifier".to_owned(), v);
        }
    }
    match query.system_uuid.clone() {
        None => {},
        Some(v) => {
            map.insert("system_uuid".to_owned(), v);
        }
    }
    map
}



pub fn full_query_to_map(query: &Query<FullQuery>) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    match query.identifier.clone() {
        None => {},
        Some(v) => {
            map.insert("identifier".to_owned(), v);
        }
    }
    match query.recursive.clone() {
        None => {},
        Some(v) => {
            map.insert("recursive".to_owned(), v);
        }
    }
    match query.path.clone() {
        None => {},
        Some(v) => {
            map.insert("path".to_owned(), v);
        }
    }
    match query.system_uuid.clone() {
        None => {},
        Some(v) => {
            map.insert("system_uuid".to_owned(), v);
        }
    }
    match query.topic_name.clone() {
        None => {},
        Some(v) => {
            map.insert("topic_name".to_owned(), v);
        }
    }
    map
}

#[allow(unused)]
pub fn json_wrap(result: JuizResult<CapsulePtr>) -> impl IntoResponse {
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

pub fn json_output_wrap(result: JuizResult<CapsulePtr>) -> impl IntoResponse {
    //log::trace!("json_output_wrap() called");
    if result.is_err() {
        let e = result.err().unwrap();
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(
            jvalue!({
                "message": format!("Internal Server Error:  {:#}, {:}", e, e.to_string())
            }))).into_response()
    }
    match result {
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(
                jvalue!({
                    "message": format!("Internal Server Error:  {:#}, {:}", e, e.to_string())
                }))).into_response()
        },
        Ok(v) => {
            capsule_ptr_to_response(v)
        }
    }
}


#[cfg(not(feature= "opencv4"))]
fn capsule_ptr_to_response(v: CapsulePtr) -> axum::http::Response<Body> {
    use image::{EncodableLayout, ImageFormat};
    use reqwest::header;

    if v.is_value().unwrap() {
        v.lock_as_value(|value| {
            let mut r = Json(value).into_response();
            let hdrs = r.headers_mut();
            hdrs.append("Cache-Control", HeaderValue::from_str("no-cache").unwrap());
            r
        }).unwrap()
    } else if v.is_image().unwrap() {

        use std::io::{BufWriter, Cursor};
        // log::trace!("Detect Image Response v={v:?}");
        v.lock_as_image(|image| {

            let mut buffer = BufWriter::new(Cursor::new(Vec::new()));
            image.write_to(&mut buffer, ImageFormat::Png).unwrap();

            //let bytes = buffer.into_inner().unwrap().into_inner().as_bytes();
            Response::builder()
                        .extension("png")
                        .header("Content-Type", "image/png")
                        .status(StatusCode::OK)
                        .body(Body::from(buffer.into_inner().unwrap().into_inner())).unwrap().into_response()
        }).unwrap()
    } else {
        Json(jvalue!({})).into_response()
    }
}

#[cfg(feature= "opencv4")]
fn capsule_ptr_to_response(v: CapsulePtr) -> axum::http::Response<Body>  {
    if v.is_value().unwrap() {
        v.lock_as_value(|value| {
            let mut r = Json(value).into_response();
            let hdrs = r.headers_mut();
            hdrs.append("Cache-Control", HeaderValue::from_str("no-cache").unwrap());
            r
        }).unwrap()
    } else if v.is_mat().unwrap() {
        v.lock_as_mat(|result| {
            let mut buf : opencv::core::Vector<u8> = Vector::new();

            match imencode(".png", result, &mut buf, &Vector::new()) {
                Ok(_result) => {
                    Response::builder()
                        .extension("png")
                        .header("Content-Type", "image/png")
                        .status(StatusCode::OK)
                        .body(Body::from(buf.to_vec())).unwrap().into_response()
                },
                Err(e) => {
                    Json(jvalue!({
                        "result": "Error",
                        "error": format!("{}", e)
                    })).into_response()
                },
            }
            
        }).unwrap()
    } else {
        Json(jvalue!({})).into_response()
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

pub fn app_new(crud_broker: Arc<Mutex<CRUDBroker>>, static_filepaths: Option<Vec<(String, PathBuf)>>) -> Router {
    let mut api = ApiDoc::openapi();
    api.merge(system::ApiDoc::openapi());
    api.merge(process::ApiDoc::openapi());
    api.merge(container::ApiDoc::openapi());
    api.merge(container_process::ApiDoc::openapi());
    api.merge(broker::ApiDoc::openapi());
    api.merge(execution_context::ApiDoc::openapi());
    api.merge(connection::ApiDoc::openapi());
    api.merge(topic::ApiDoc::openapi());
    log::warn!("static_filepaths: {:?}", static_filepaths);
    let mut r = Router::new()
            .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", api))
            .nest("/api/", any::object_router(crud_broker.clone()));

    match static_filepaths {
        Some(paths) => {
            for (url, path) in paths.iter() {
                log::debug!("http_broker serves url={url} for path={path:?}");
                r = r.nest_service(url.as_str(), ServeDir::new(path));
            }
            r
        },
        None => r
    }
}

#[allow(unused)]
pub fn append_route(api: &mut utoipa::openapi::OpenApi, body_context: Value, description: &str, operation_key: &str) -> ()  {
    let ctt = ContentBuilder::new().example(Some(body_context)).build();
    let rb = RequestBodyBuilder::new().description(Some(description.to_owned()))
        .content("application/json", ctt)
        .build();
    let operation = OperationBuilder::new().request_body(Some(rb)).build();
    api.paths.paths.insert(operation_key.to_owned(), PathItem::new(PathItemType::Patch, operation));
}

