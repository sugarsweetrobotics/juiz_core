

use opencv::prelude::*;
use juiz_core::prelude::*;
use crate::video_capture::CvVideoCapture;

fn cv_video_capture_read_function(container: &mut ContainerImpl<CvVideoCapture>, _v: CapsuleMap) -> JuizResult<Capsule> {
    let mut frame: opencv::core::Mat = Mat::default();
    container.camera.read(&mut frame)?;
    return Ok(frame.into());
}

fn manifest() -> Value {
    ContainerProcessManifest::new(CvVideoCapture::manifest(), "cv_video_capture_read").into()
}

#[no_mangle]
pub unsafe extern "Rust" fn cv_video_capture_read_factory() -> JuizResult<ContainerProcessFactoryPtr> {
    ContainerProcessFactoryImpl::create(
        manifest(),
        &cv_video_capture_read_function)
}


