


use std::sync::{Arc, Mutex};
use opencv::{core::Mat, imgproc::{cvt_color, COLOR_BGR2RGB}};
use juiz_core::{jvalue, create_process_factory, processes::capsule::{Capsule, CapsuleMap}, JuizResult, ProcessFactory};




fn cvt_color_function(args: CapsuleMap) -> JuizResult<Capsule> {
    println!("cvt_color_function called");
    match args.get("code") {
        Some(mode_str) => {
            let img: opencv::core::Mat = args.get("src").unwrap().clone().to_mat().unwrap();
            let mut out_img = Mat::default();
            match cvt_color(&img, &mut out_img, COLOR_BGR2RGB, 0) {
                Ok(()) => {
                    Ok(out_img.into())
                },
                Err(e) => {
                    Err(anyhow::Error::from(e))
                }
            }
        }, None => {
            todo!()
        }
    }
    
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

