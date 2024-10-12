

use juiz_core::{prelude::*, anyhow};
use opencv::{core::{Mat, Vector}, imgcodecs::imwrite};
use cv_convert::TryFromCv;
use crate::filesystem::CvFilesystem;

fn imwrite_function(_container: &mut ContainerImpl<CvFilesystem>, args: CapsuleMap) -> JuizResult<Capsule> {
    let file_name = args.get("filename")?.lock_as_value(|value| {
        value.as_str().unwrap().to_owned()
    })?;
    println!("imwrite_function(file_name={file_name:})");

    args.get("src")?.lock_as_image(|img| {
        let mat = Mat::try_from_cv(img)?;
        let params: Vector<i32> = Vector::new();
        match imwrite(file_name.as_str(), &mat, &params) {
             Ok(_) => {
                 println!("ok");
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
    ContainerProcessManifest::new(CvFilesystem::manifest(), "imwrite")
        .description("")
        .add_image_arg("src", "")
        .add_string_arg("filename", "", "img.png")
        .into()
}


#[no_mangle]
pub unsafe extern "Rust" fn imwrite_factory() -> JuizResult<ContainerProcessFactoryPtr> {
    ContainerProcessFactoryImpl::create(
        manifest(),
        &imwrite_function)
}


