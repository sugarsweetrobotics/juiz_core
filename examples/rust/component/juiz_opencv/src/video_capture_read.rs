

use juiz_core::prelude::*;
use opencv::{core::Mat, videoio::VideoCaptureTrait};
use crate::video_capture::CvVideoCapture;

fn cv_video_capture_read_function(container: &mut ContainerImpl<CvVideoCapture>, _v: CapsuleMap) -> JuizResult<Capsule> {
    let mut frame : Mat = container.image.take().unwrap();
    container.camera.read(&mut frame)?;
    container.image = Some(frame);
    return Ok(jvalue!(true).into());
}

pub(crate) fn manifest() -> ProcessManifest {
    ProcessManifest::new("cv_video_capture_read")
        .container(CvVideoCapture::manifest())
        .factory("cv_video_capture_read_factory")
}

#[no_mangle]
pub unsafe extern "Rust" fn cv_video_capture_read_factory() -> JuizResult<ContainerProcessFactoryPtr> {
    container_process_factory_create(
        manifest(),
        &cv_video_capture_read_function)
}


