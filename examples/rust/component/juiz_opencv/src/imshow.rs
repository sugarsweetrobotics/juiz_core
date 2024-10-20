
use juiz_core::{anyhow, prelude::*};
use opencv::{core::Mat, highgui::{imshow, wait_key}};
use crate::window::CvWindow;
use cv_convert::TryFromCv;

fn imshow_function(container: &mut ContainerImpl<CvWindow>, args: CapsuleMap) -> JuizResult<Capsule> {
    let window_name = container.name.as_str();
    println!("imshow_function(name={window_name:})");
    args.get("src")?.lock_as_image(|img| {
        let mat = Mat::try_from_cv(img)?;
        match imshow(window_name, &mat) {
            Ok(()) => {
                println!("ok");
                wait_key(0)?;
                Ok(Capsule::empty())
            },
            Err(e) => {
                println!("error: {e:?}");
                Err(anyhow::Error::from(e))
            }
        }
    })?
}

fn manifest() -> Value {
    ContainerProcessManifest::new(CvWindow::manifest(), "imshow")
        .add_image_arg("src", "")
        .into()
}


#[no_mangle]
pub unsafe extern "Rust" fn imshow_factory() -> JuizResult<ContainerProcessFactoryPtr> {
    container_process_factory_create(
        manifest(),
        &imshow_function)
}


