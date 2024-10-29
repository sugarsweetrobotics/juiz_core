

use juiz_sdk::{image::DynamicImage, prelude::*};
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

pub(crate) fn manifest() -> ProcessManifest {
    ProcessManifest::new("cv_video_capture_readandget")
        .container(CvVideoCapture::manifest())
        .factory("cv_video_capture_readandget_factory")
}

#[no_mangle]
pub unsafe extern "Rust" fn cv_video_capture_readandget_factory() -> JuizResult<ContainerProcessFactoryStruct> {
    Ok(juiz_sdk::container_process_factory(
        manifest(),
        &cv_video_capture_readandget_function))
}


