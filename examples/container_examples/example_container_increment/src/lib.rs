use std::sync::{Arc, Mutex};

use example_container::example_container::ExampleContainer;
//use example_container::ExampleContainer;
use juiz_core::{containers::{container_impl::ContainerImpl, create_container_process_factory}, jvalue, processes::capsule::{Capsule, CapsuleMap}, ContainerProcessFactory, JuizResult, Value};


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


fn increment_function(container: &mut ContainerImpl<ExampleContainer>, v: CapsuleMap) -> JuizResult<Capsule> {
    let i = v.get_int("arg1")?;
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
    create_container_process_factory::<ExampleContainer>(manifest, &increment_function)
}
