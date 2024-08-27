
use juiz_core::{prelude::*, anyhow, opencv::highgui::*};
use crate::window::CvWindow;

fn imshow_function(container: &mut ContainerImpl<CvWindow>, args: CapsuleMap) -> JuizResult<Capsule> {
    let window_name = container.name.as_str();
    println!("imshow_function(name={window_name:})");
    args.get("src")?.lock_as_mat(|img| {
        match imshow(window_name, img) {
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
    ContainerProcessFactoryImpl::create(
        manifest(),
        &imshow_function)
}


