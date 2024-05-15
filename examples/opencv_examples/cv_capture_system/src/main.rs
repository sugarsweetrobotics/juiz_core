
use juiz_core::{System, jvalue, JuizResult};


// #[tokio::main]
fn main() -> JuizResult<()> {
    let manifest = jvalue!(
        {

            "name": "test_system",
            "plugins": {
                "process_factories": {
                    
                },
                "ec_factories": {
                    "timer_ec": {
                        "path": "./target/debug"
                    }
                },
                "broker_factories": {
                    "http_broker": {
                        "path": "./target/debug"
                    }
                },
                /*
                "container_factories": {
                    "cv_camera_container": {
                        "path": "./target/debug",
                        "processes": {
                            "cv_camera_capture": {
                                "path": "./target/debug",
                            },
                            "cv_filesystem_imsave": {
                                "path": "./target/debug",
                            }
                        }
                    }
                }, */
                "components": {
                    "cv_camera_capture": {
                        "path": "./target/debug",
                    },
                    /*
                    "cv_filesystem": {
                        "path": "./target/debug",
                    } */
                }
            },
            "brokers": [
                {
                    "type_name": "http",
                    "name": "localhost:8080"
                }  
            ],
            "processes": [

                {
                    "type_name": "cv_cvt_color",
                    "name": "cv_cvt_color0",
                    "processes": [
                        {
                            "type_name": "cv_camera_capture_read",
                            "name": "read0",
                        }
                    ]
                },
            ],
            "containers": [
                {
                    "type_name": "cv_camera_capture",
                    "name": "cv_camera0",
                    "processes": [
                        {
                            "type_name": "cv_camera_capture_read",
                            "name": "read0",
                        }
                    ]
                },
                /*
                {
                    "type_name": "cv_filesystem",
                    "name": "cv_filesystem0",
                    "processes": [
                        {
                            "type_name": "cv_filesystem_save",
                            "name": "save0",
                        }
                    ]
                }
                */
                /*
                {
                    "type_name": "cv_camera_container",
                    "name": "cv_camera0",
                    "processes": [
                        {
                            "type_name": "cv_camera_capture",
                            "name": "camera0",
                        },
                        {
                            "type_name": "cv_filesystem_imsave",
                            "name": "imsave"
                        }
                    ]
                }
                */
            ],
            "ecs": [
                {
                    "type_name": "TimerEC",
                    "name": "timer0",
                    "rate": 0.05,
                    "bind": [
                        
                    ]
                }
            ],
            
            "connections": [
                {
                    //"id": "con01",
                    "arg_name": "src",
                    "source" : {
                        "id": "core://core/ContainerProcess/read0::cv_camera_capture_read"
                    }, 
                    "destination" : {
                        "id": "core://core/Process/cv_cvt_color0::cv_cvt_color"
                    }
                }
                
            ]
        }
    );
    {
        System::new(manifest)?.run_and_do(|system|{
            let v = system.broker_proxy(&jvalue!({"type_name":"local"}))?.lock().unwrap().system_profile_full()?;
            println!("System: {:#}", v);
            Ok(())
        })
    }
}