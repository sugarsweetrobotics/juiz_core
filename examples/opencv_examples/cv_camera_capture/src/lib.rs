
pub mod cv_camera_capture {

    use std::sync::{Arc, Mutex};

    use opencv::{highgui, prelude::*, videoio, Result};
    use cv_camera_container::cv_camera_container::CvCaptureContainer;
    use juiz_core::{jvalue, JuizResult, Value, ContainerProcessFactory, containers::create_container_process_factory, processes::{Argument, Output}};

    
    #[no_mangle]
    pub unsafe extern "Rust" fn _manifest() -> Value { 
        return jvalue!({
            "container_type_name": "example_container",
            "type_name": "example_container_get",
            "arguments" : {
            }, 
        }); 
    }


    fn capture_function(container: &mut Box<CvCaptureContainer>, _v: Vec<Argument>) -> JuizResult<Output> {
        let mut frame: opencv::core::Mat = Mat::default();
		container.camera.read(&mut frame)?;
		
        return Ok(Output::new(jvalue!({})));
    }


    #[no_mangle]
    pub unsafe extern "Rust" fn container_process_factory() -> JuizResult<Arc<Mutex<dyn ContainerProcessFactory>>> {
        env_logger::init();
        create_container_process_factory::<CvCaptureContainer>(_manifest(), capture_function)
    }

}