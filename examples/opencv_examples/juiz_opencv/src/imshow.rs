

use std::sync::{Arc, Mutex};


use opencv::highgui::*;

use juiz_core::{containers::create_container_process_factory, jvalue, processes::capsule::{Capsule, CapsuleMap}, ContainerProcessFactory, JuizResult};

use crate::window::CvWindow;

fn imshow_function(container: &mut Box<CvWindow>, args: CapsuleMap) -> JuizResult<Capsule> {
    let window_name = container.name.as_str();
    println!("imshow_function(name={window_name:})");
    args.get("src")?.lock_as_mat(|img| {
        //let mut params: Vector<i32> = Vector::new();
        //imwrite("hoge.png", img, &params);
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


#[no_mangle]
pub unsafe extern "Rust" fn imshow_factory() -> JuizResult<Arc<Mutex<dyn ContainerProcessFactory>>> {
    create_container_process_factory::<CvWindow>(
        jvalue!({
            "container_type_name": "cv_window",
            "type_name": "imshow",
            "arguments" : {
                "src": {
                    "type": "image",
                    "description": "",
                    "default": {}
                },
            }, 
        }),
        imshow_function)
}


