


use std::sync::{Arc, Mutex};
use opencv::{core::Mat, imgproc::{cvt_color, COLOR_BGR2RGB}};
use juiz_core::{create_process_factory, jvalue, processes::capsule::{Capsule, CapsuleMap}, JuizResult, ProcessFactory};




fn cvt_color_function(args: CapsuleMap) -> JuizResult<Capsule> {
    println!("cvt_color_function called");
    let _mode_str = args.get("code")?;
    let mut out_img = Mat::default();
    args.get("src")?.lock_as_mat(|img| {
        match cvt_color(img, &mut out_img, COLOR_BGR2RGB, 0) {
            Ok(()) => {
                Ok(out_img.into())
            },
            Err(e) => {
                Err(anyhow::Error::from(e))
            }
        }
    })?
}


#[no_mangle]
pub unsafe extern "Rust" fn cv_cvt_color_factory() -> JuizResult<Arc<Mutex<dyn ProcessFactory>>> {
    //env_logger::init();
    
    create_process_factory(jvalue!({
        "type_name": "cv_cvt_color",
        "arguments" : {
            "src": {
                "type": "image",
                "description": "",
                "default": {}
            },
            "code": {
                "type": "string",
                "description": "Convert Method. (BGR2RGB)",
                "default": "BGR2RGB",
            }, 
        }, 
    }) , cvt_color_function)
}

