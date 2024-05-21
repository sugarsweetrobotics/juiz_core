
use std::sync::{Arc, Mutex};

use juiz_core::{create_container_factory, jvalue, ContainerFactory, JuizResult, Value};


#[allow(dead_code)]
#[repr(Rust)]
pub struct CvFilesystem {
    //pub name: String
}


fn create_cv_filesystem_container(_manifest: Value) -> JuizResult<Box<CvFilesystem>> {
    //let name = obj_get_str(&manifest, "name")?;
    //println!("opencv::named_window({name:})");
    //named_window(name, 1)?;
    Ok(Box::new(CvFilesystem{}))
}

#[no_mangle]
pub unsafe extern "Rust" fn cv_filesystem_factory() -> JuizResult<Arc<Mutex<dyn ContainerFactory>>> {
    // env_logger::init();
    create_container_factory(jvalue!({ "type_name": "cv_filesystem"}), create_cv_filesystem_container)
}


