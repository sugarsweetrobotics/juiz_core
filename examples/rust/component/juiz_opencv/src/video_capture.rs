

use opencv::videoio;
use juiz_core::prelude::*;


#[allow(dead_code)]
#[repr(Rust)]
pub struct CvVideoCapture {
    pub camera: videoio::VideoCapture
}

impl CvVideoCapture {

    pub fn manifest() -> Value {
        ContainerManifest::new("cv_video_capture").into()
    }
}

fn create_cv_capture_container(_manifest: Value) -> JuizResult<Box<CvVideoCapture>> {
    let cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera
    Ok(Box::new(CvVideoCapture{camera: cam}))
}

#[no_mangle]
pub unsafe extern "Rust" fn cv_video_capture_factory() -> JuizResult<ContainerFactoryPtr> {
    // env_logger::init();
    ContainerFactoryImpl::create(CvVideoCapture::manifest(), create_cv_capture_container)
}
