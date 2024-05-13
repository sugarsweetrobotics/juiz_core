

use std::sync::{Arc, Mutex};
use opencv::videoio;
use juiz_core::{create_container_factory, jvalue, ContainerFactory, JuizResult, Value};


#[allow(dead_code)]
#[repr(Rust)]
pub struct CvCameraCapture {
    pub camera: videoio::VideoCapture
}

fn create_cv_capture_container(_manifest: Value) -> JuizResult<Box<CvCameraCapture>> {
    println!("create_cv_camera_container({})", _manifest);
    let cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera
    Ok(Box::new(CvCameraCapture{camera: cam}))
}


#[no_mangle]
pub unsafe extern "Rust" fn cv_camera_capture_factory() -> JuizResult<Arc<Mutex<dyn ContainerFactory>>> {
    // env_logger::init();
    create_container_factory(jvalue!({ "type_name": "cv_camera_capture"}), create_cv_capture_container)
}
