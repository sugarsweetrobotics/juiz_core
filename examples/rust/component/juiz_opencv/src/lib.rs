

mod video_capture;
mod video_capture_read;
mod video_capture_get;
mod video_capture_readandget;
mod cvt;
mod window;
mod imshow;
mod filesystem;
mod imwrite;

pub mod cv_camera_capture {

    use juiz_base::prelude::*;
    use juiz_base::env_logger;

    use crate::filesystem;
    use crate::imshow;
    use crate::imwrite;
    use crate::video_capture;
    use crate::video_capture_get;
    use crate::video_capture_read;
    use crate::video_capture_readandget;
    use crate::window;


    #[no_mangle]
    pub unsafe extern "Rust" fn component_manifest() -> ComponentManifest {
        env_logger::init();
        ComponentManifest::new("opencv")
            .add_container(video_capture::CvVideoCapture::manifest()
                .add_process(video_capture_get::manifest())
                .add_process(video_capture_read::manifest())
                .add_process(video_capture_readandget::manifest()))
            .add_container(window::CvWindow::manifest()
                .add_process(imshow::manifest()))
            .add_container(filesystem::CvFilesystem::manifest()
                .add_process(imwrite::manifest()))   
    }

    #[no_mangle]
    pub unsafe extern "Rust" fn component_manifest_value() -> Value {
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
                    },
                    {
                        "type_name": "cv_video_capture_readandget",
                        "factory": "cv_video_capture_readandget_factory"
                    },
                    {
                        "type_name": "cv_video_capture_get",
                        "factory": "cv_video_capture_get_factory"
                    }
                    ]
                },
                {
                    "type_name": "cv_window",
                    "factory": "cv_window_factory",
                    "processes": [ {
                        "type_name": "imshow",
                        "factory": "imshow_factory"
                    }]
                },
                {
                    "type_name": "cv_filesystem",
                    "factory": "cv_filesystem_factory",
                    "processes": [ {
                        "type_name": "imwrite",
                        "factory": "imwrite_factory"
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