

use juiz_core::{prelude::*, opencv::highgui::*};
use juiz_core::value::obj_get_str;

#[allow(dead_code)]
#[repr(Rust)]
pub struct CvWindow {
    pub name: String
}


impl CvWindow {

    pub fn manifest() -> Value {

        ContainerManifest::new("cv_window").into()
    }
}

fn create_cv_window_container(manifest: Value) -> JuizResult<Box<CvWindow>> {
    let name = obj_get_str(&manifest, "name")?;
    //println!("opencv::named_window({name:})");
    named_window(name, 1)?;
    Ok(Box::new(CvWindow{name: name.to_owned()}))
}

#[no_mangle]
pub unsafe extern "Rust" fn cv_window_factory() -> JuizResult<ContainerFactoryPtr> {
    // env_logger::init();
    ContainerFactoryImpl::create(CvWindow::manifest(), create_cv_window_container)
}


