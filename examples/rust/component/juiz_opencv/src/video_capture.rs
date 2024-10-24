

use juiz_core::prelude::*;
use opencv::{core::Mat, videoio::{VideoCapture, CAP_ANY}};

#[allow(dead_code)]
#[repr(Rust)]
pub struct CvVideoCapture {
    pub camera: VideoCapture,
    pub image: Option<Mat>,
}

impl CvVideoCapture {

    pub fn manifest() -> ContainerManifest {
        ContainerManifest::new("cv_video_capture")
            .factory("cv_video_capture_factory")
    }
}

impl Drop for CvVideoCapture {
    fn drop(&mut self) {
        log::info!("CvVideoCapture::drop() called");
    }
}

fn create_cv_capture_container(manifest: ContainerManifest) -> JuizResult<Box<CvVideoCapture>> {
    log::trace!("create_cv_capture_container({:?}) called", manifest);
    let index = obj_get_i64(&manifest.args, "index").or::<JuizError>(Ok(0)).unwrap();
    let cam = VideoCapture::new(index as i32, CAP_ANY)?; // 0 is the default camera
    Ok(Box::new(CvVideoCapture{camera: cam, image: Some(Mat::default()) }))
}

#[no_mangle]
pub unsafe extern "Rust" fn cv_video_capture_factory() -> JuizResult<ContainerFactoryPtr> {
    log::trace!("cv_video_capture_factory() called");
    container_factory_create(CvVideoCapture::manifest(), create_cv_capture_container)
}
