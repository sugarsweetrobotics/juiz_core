

use juiz_sdk::prelude::*;
use opencv::highgui::named_window;

#[allow(dead_code)]
#[repr(Rust)]
pub struct CvWindow {
    pub name: String
}


impl CvWindow {

    pub fn manifest() -> ContainerManifest {

        ContainerManifest::new("cv_window")
            .factory("cv_window_factory")
    }
}

fn create_cv_window_container(manifest: ContainerManifest) -> JuizResult<Box<CvWindow>> {
    let name = obj_get_str(&manifest.args, "name")?;
    //println!("opencv::named_window({name:})");
    named_window(name, 1)?;
    Ok(Box::new(CvWindow{name: name.to_owned()}))
}

#[no_mangle]
pub unsafe extern "Rust" fn cv_window_factory() -> JuizResult<ContainerFactoryStruct> {
    // env_logger::init();
    Ok(juiz_sdk::container_factory(CvWindow::manifest(), create_cv_window_container))
}


