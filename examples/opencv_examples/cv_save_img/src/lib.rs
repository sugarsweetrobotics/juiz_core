
pub mod cv_save_img {

    use std::sync::{Mutex, Arc};

    use opencv::{highgui, prelude::*, videoio, Result};
    use juiz_core::{jvalue, JuizResult, Value, ProcessFactory, create_process_factory, processes::{arg, Argument, Output}};


    #[no_mangle]
    pub unsafe extern "Rust" fn manifest() -> Value { 

        return jvalue!({
            "type_name": "cv_save_image",
            "arguments" : {
                "filename": {
                    "type": "string",
                    "description": "filename",
                    "default": "image.png",
                }, 
            }, 
        }); 
    }


    fn save_function(args: Vec<Argument>) -> JuizResult<Output> {
        let filename = arg(&args, "filename")?.as_str().unwrap();



        return Ok(Output::new(jvalue!(filename)));
    }

    #[no_mangle]
    pub unsafe extern "Rust" fn process_factory() -> JuizResult<Arc<Mutex<dyn ProcessFactory>>> {
        env_logger::init();
        
        create_process_factory(manifest(), save_function)
    }


}