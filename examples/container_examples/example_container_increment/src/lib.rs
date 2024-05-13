use std::sync::{Arc, Mutex};

use example_container::example_container::ExampleContainer;
//use example_container::ExampleContainer;
use juiz_core::{jvalue, JuizResult, Value, ContainerProcessFactory, processes::capsule::{Capsule, CapsuleMap}, containers::create_container_process_factory};


#[no_mangle]
pub unsafe extern "Rust" fn manifest() -> Value { 

    return jvalue!({
        "container_type_name": "example_container",
        "type_name": "example_container_increment",
        "arguments" : {
            "arg1": {
                "type": "int",
                "description": "test_argument",
                "default": 1,
            }, 
        }, 
    }); 
}


fn increment_function(container: &mut Box<ExampleContainer>, v: CapsuleMap) -> JuizResult<Capsule> {
    let i = v.get("arg1").unwrap().as_value().unwrap().as_i64().unwrap();
    //let i = arg(&v, "arg1")?.as_i64().unwrap();
    container.value = container.value + i;
    return Ok(jvalue!(container.value).into());
}


#[no_mangle]
pub unsafe extern "Rust" fn container_process_factory() -> JuizResult<Arc<Mutex<dyn ContainerProcessFactory>>> {
    env_logger::init();
    let manifest = jvalue!({
        "container_type_name": "example_container",
        "type_name": "example_container_increment",
        "arguments" : {
            "arg1": {
                "type": "int",
                "description": "test_argument",
                "default": 1,
            }, 
        }, 
    });
    create_container_process_factory::<ExampleContainer>(manifest, increment_function)
}
