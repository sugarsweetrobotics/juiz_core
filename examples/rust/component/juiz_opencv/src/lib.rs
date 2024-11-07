

mod video_capture;
mod video_capture_read;
mod video_capture_get;
mod video_capture_readandget;
mod cv_cvt_color;
mod window;
mod window_show;
mod filesystem;
mod filesystem_imwrite;

pub use juiz_sdk::prelude::*;

use video_capture::*;
use video_capture_read::*;
use video_capture_get::*;
use video_capture_readandget::*;
use window::*;
use window_show::*;
use filesystem::*;
use cv_cvt_color::*;

juiz_component_manifest!(
    container_name = "juiz_opencv"
    containers = {
        video_capture = [
            video_capture_get,
            video_capture_read,
            video_capture_readandget
        ],
        window = [
            window_show,
        ],
        filesystem = [

        ]
    },
    processes = [
        cv_cvt_color
    ]
);