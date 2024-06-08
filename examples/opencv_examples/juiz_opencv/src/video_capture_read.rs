

use std::sync::{Arc, Mutex};

use opencv::prelude::*;
use juiz_core::{containers::{container_impl::ContainerImpl, create_container_process_factory}, jvalue, processes::capsule::{Capsule, CapsuleMap}, ContainerProcessFactory, JuizResult};

use crate::video_capture::CvVideoCapture;

fn cv_video_capture_read_function(container: &mut ContainerImpl<CvVideoCapture>, _v: CapsuleMap) -> JuizResult<Capsule> {
    let mut frame: opencv::core::Mat = Mat::default();
    container.camera.read(&mut frame)?;
    return Ok(frame.into());
}


#[no_mangle]
pub unsafe extern "Rust" fn cv_video_capture_read_factory() -> JuizResult<Arc<Mutex<dyn ContainerProcessFactory>>> {
    create_container_process_factory::<CvVideoCapture>(
        jvalue!({
            "container_type_name": "cv_video_capture",
            "type_name": "cv_video_capture_read",
            "arguments" : {
            }, 
        }),
        cv_video_capture_read_function)
}


