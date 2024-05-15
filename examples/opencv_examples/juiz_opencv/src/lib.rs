

mod video_capture;
mod video_capture_read;
mod cvt;

pub mod cv_camera_capture {
    use juiz_core::{jvalue, Value};

    #[no_mangle]
    pub unsafe extern "Rust" fn component_profile() -> Value {
        env_logger::init();
        return jvalue!({
            "type_name": "opencv",
            "containers": [
                {
                    "type_name": "cv_video_capture",
                    "factory": "cv_video_capture_factory",
                    "processes": [ {
                        "type_name": "cv_video_capture_read",
                        "factory": "cv_video_capture_read_factory"
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