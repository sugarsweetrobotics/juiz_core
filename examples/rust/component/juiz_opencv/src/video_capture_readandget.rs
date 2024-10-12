

use juiz_core::{image::{DynamicImage, RgbImage}, prelude::*};
use opencv::{core::Mat, imgproc::{cvt_color, COLOR_BGR2RGB}, videoio::VideoCaptureTrait};
use crate::video_capture::CvVideoCapture;

use cv_convert::TryFromCv;

fn cv_video_capture_readandget_function(container: &mut ContainerImpl<CvVideoCapture>, _v: CapsuleMap) -> JuizResult<Capsule> {
    let mut frame : Mat = Mat::default();
    let mut dst: Mat = Mat::default();
    container.camera.read(&mut frame)?;
    cvt_color(&frame, &mut dst, COLOR_BGR2RGB, 0)?;
    let img : DynamicImage = DynamicImage::try_from_cv(dst)?;
    let value: Capsule = img.into();
    return Ok(value);
}

fn manifest() -> Value {
    ContainerProcessManifest::new(CvVideoCapture::manifest(), "cv_video_capture_readandget").into()
}

#[no_mangle]
pub unsafe extern "Rust" fn cv_video_capture_readandget_factory() -> JuizResult<ContainerProcessFactoryPtr> {
    ContainerProcessFactoryImpl::create(
        manifest(),
        &cv_video_capture_readandget_function)
}


