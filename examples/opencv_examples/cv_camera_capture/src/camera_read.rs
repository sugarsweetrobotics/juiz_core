

use std::sync::{Arc, Mutex};

use opencv::prelude::*;
use juiz_core::{containers::create_container_process_factory, jvalue, processes::capsule::{Capsule, CapsuleMap}, ContainerProcessFactory, JuizResult};

use crate::camera_capture::CvCameraCapture;

fn cv_camera_capture_read_function(container: &mut Box<CvCameraCapture>, _v: CapsuleMap) -> JuizResult<Capsule> {
    println!("Camera Capture");
    let mut frame: opencv::core::Mat = Mat::default();
    container.camera.read(&mut frame)?;
    return Ok(frame.into());
}


#[no_mangle]
pub unsafe extern "Rust" fn cv_camera_capture_read_factory() -> JuizResult<Arc<Mutex<dyn ContainerProcessFactory>>> {
    create_container_process_factory::<CvCameraCapture>(
        jvalue!({
            "container_type_name": "cv_camera_capture",
            "type_name": "cv_camera_capture_read",
            "arguments" : {
            }, 
        }),
        cv_camera_capture_read_function)
}


