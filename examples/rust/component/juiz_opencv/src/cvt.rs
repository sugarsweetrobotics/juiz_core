


use juiz_core::prelude::*;


fn cvt_color_function(args: CapsuleMap) -> JuizResult<Capsule> {
    println!("cvt_color_function called");
    let _mode_str = args.get("code")?;
    todo!()
    // let mut out_img = Mat::default();
    // args.get("src")?.lock_as_mat(|img| {
    //     match cvt_color(img, &mut out_img, COLOR_BGR2RGB, 0) {
    //         Ok(()) => {
    //             Ok(out_img.into())
    //         },
    //         Err(e) => {
    //             Err(anyhow::Error::from(e))
    //         }
    //     }
    // })?
}

fn manifest() -> Value{
    ProcessManifest::new("cv_cvt_color")
    .description("Convert Color")
    .add_image_arg("src", "")
    .add_string_arg("code", "ConvertMethod. (BGR2RGB)", "BGR2RGB").into()
}


#[no_mangle]
pub unsafe extern "Rust" fn cv_cvt_color_factory() -> JuizResult<ProcessFactoryPtr> {
    ProcessFactoryImpl::create(
        manifest(),
                cvt_color_function
    )
}

