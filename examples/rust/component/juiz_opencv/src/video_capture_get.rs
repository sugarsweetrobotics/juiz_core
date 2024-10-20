

use juiz_core::{image::DynamicImage, prelude::*};
use crate::video_capture::CvVideoCapture;

use cv_convert::TryFromCv;

fn cv_video_capture_get_function(container: &mut ContainerImpl<CvVideoCapture>, _v: CapsuleMap) -> JuizResult<Capsule> {
    let img : DynamicImage = DynamicImage::try_from_cv(container.image.clone().unwrap())?;
    let value: Capsule = img.into();
    return Ok(value);
}

fn manifest() -> Value {
    ContainerProcessManifest::new(CvVideoCapture::manifest(), "cv_video_capture_get").into()
}

#[no_mangle]
pub unsafe extern "Rust" fn cv_video_capture_get_factory() -> JuizResult<ContainerProcessFactoryPtr> {
    container_process_factory_create(
        manifest(),
        &cv_video_capture_get_function)
}


