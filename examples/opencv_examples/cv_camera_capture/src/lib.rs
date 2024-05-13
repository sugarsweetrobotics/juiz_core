

mod camera_capture;
mod camera_read;
mod cvt;

pub mod cv_camera_capture {
    use juiz_core::{jvalue, Value};

    #[no_mangle]
    pub unsafe extern "Rust" fn component_profile() -> Value {
        env_logger::init();
        return jvalue!({
            "type_name": "cv_camera",
            "containers": [
                {
                    "type_name": "cv_camera_capture",
                    "factory": "cv_camera_capture_factory",
                    "processes": [ {
                        "type_name": "cv_camera_capture_read",
                        "factory": "cv_camera_capture_read_factory"
                    }]
                }
            ],
            "processes": [
                {
                    "type_name": "cv_cvt_color",
                    "factory": "cv_cvt_color_factory",
                }
            ]
        }); 
    }
}