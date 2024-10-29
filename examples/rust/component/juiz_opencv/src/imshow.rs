
use juiz_sdk::{anyhow, prelude::*};
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

pub(crate) fn manifest() -> ProcessManifest {
    ProcessManifest::new("imshow")
        .container(CvWindow::manifest())
        .factory("imshow_factory")
        .add_image_arg("src", "")
}


#[no_mangle]
pub unsafe extern "Rust" fn imshow_factory() -> JuizResult<ContainerProcessFactoryStruct> {
    Ok(juiz_sdk::container_process_factory(
        manifest(),
        &imshow_function))
}


