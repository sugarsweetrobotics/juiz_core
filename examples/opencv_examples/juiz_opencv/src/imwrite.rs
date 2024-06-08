

use std::sync::{Arc, Mutex};

use opencv::core::Vector;

use opencv::imgcodecs::*;
use juiz_core::{containers::{container_impl::ContainerImpl, create_container_process_factory}, jvalue, processes::capsule::{Capsule, CapsuleMap}, ContainerProcessFactory, JuizResult};

use crate::filesystem::CvFilesystem;

fn imwrite_function(_container: &mut ContainerImpl<CvFilesystem>, args: CapsuleMap) -> JuizResult<Capsule> {
    let mut file_name: String = "".to_owned();
    //println!("imshow_function(name={window_name:})");

    args.get("filename")?.lock_as_value(|value| {
        file_name = value.as_str().unwrap().to_owned();
    })?;
    println!("imwrite_function(file_name={file_name:})");

    args.get("src")?.lock_as_mat(|img| {
        let params: Vector<i32> = Vector::new();
        //imwrite("hoge.png", img, &params);
        match imwrite(file_name.as_str(), img, &params) {
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


#[no_mangle]
pub unsafe extern "Rust" fn imwrite_factory() -> JuizResult<Arc<Mutex<dyn ContainerProcessFactory>>> {
    create_container_process_factory::<CvFilesystem>(
        jvalue!({
            "container_type_name": "cv_filesystem",
            "type_name": "imwrite",
            "arguments" : {
                "src": {
                    "type": "image",
                    "description": "",
                    "default": {}
                },
                "filename": {
                    "type": "string",
                    "description": "",
                    "default": "img.png"
                },
            }, 
        }),
        imwrite_function)
}


