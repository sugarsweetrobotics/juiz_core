
use juiz_core::{prelude::*, anyhow};
use opencv::highgui::{imshow, wait_key};
use crate::window::CvWindow;

fn imshow_function(container: &mut ContainerImpl<CvWindow>, args: CapsuleMap) -> JuizResult<Capsule> {
    let window_name = container.name.as_str();
    println!("imshow_function(name={window_name:})");
    args.get("src")?.lock_as_image(|image| {
        todo!()

        // let mat: Mat = image.into();
        // match imshow(window_name, mat) {
        //     Ok(()) => {
        //         println!("ok");
        //         wait_key(0)?;
        //         Ok(Capsule::empty())
        //     },
        //     Err(e) => {
        //         println!("error: {e:?}");
        //         Err(anyhow::Error::from(e))
        //     }
        // }
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


