

pub mod cv_camera_container {

    use std::sync::{Mutex, Arc};

    use juiz_core::{jvalue, JuizResult, Value, ContainerFactory, create_container_factory};
    use opencv::{highgui, prelude::*, videoio, Result};
    
    #[no_mangle]
    pub unsafe extern "Rust" fn manifest() -> Value { 
    
        return jvalue!({
            "type_name": "cv_camera_container",
        }); 
    }


    #[allow(dead_code)]
    #[repr(Rust)]
    pub struct CvCaptureContainer {
        pub camera: videoio::VideoCapture
    }

    pub fn create_cv_capture_container(_manifest: Value) -> JuizResult<Box<CvCaptureContainer>> {
        println!("create_cv_camera_container({})", _manifest);


        let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera
	

        Ok(Box::new(CvCaptureContainer{camera: cam}))
    }


    #[no_mangle]
    pub unsafe extern "Rust" fn container_factory() -> JuizResult<Arc<Mutex<dyn ContainerFactory>>> {
        env_logger::init();
        create_container_factory(manifest(), create_cv_capture_container)
    }

}
