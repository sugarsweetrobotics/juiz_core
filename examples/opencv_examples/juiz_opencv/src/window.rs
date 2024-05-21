
use std::sync::{Arc, Mutex};
use opencv::highgui::*;
use juiz_core::{create_container_factory, jvalue, value::obj_get_str, ContainerFactory, JuizResult, Value};


#[allow(dead_code)]
#[repr(Rust)]
pub struct CvWindow {
    pub name: String
}


fn create_cv_window_container(manifest: Value) -> JuizResult<Box<CvWindow>> {
    let name = obj_get_str(&manifest, "name")?;
    println!("opencv::named_window({name:})");
    named_window(name, 1)?;
    Ok(Box::new(CvWindow{name: name.to_owned()}))
}

#[no_mangle]
pub unsafe extern "Rust" fn cv_window_factory() -> JuizResult<Arc<Mutex<dyn ContainerFactory>>> {
    // env_logger::init();
    create_container_factory(jvalue!({ "type_name": "cv_window"}), create_cv_window_container)
}


