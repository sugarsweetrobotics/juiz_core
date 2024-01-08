use std::sync::{Mutex, Arc};

use juiz_core::{jvalue, JuizResult, Value, ProcessFactory, create_process_factory, processes::{arg, Argument, Output}};


#[no_mangle]
pub unsafe extern "Rust" fn manifest() -> Value { 

    return jvalue!({
        "type_name": "increment_process",
        "arguments" : {
            "arg1": {
                "type": "int",
                "description": "test_argument",
                "default": 1,
            }, 
        }, 
    }); 
}


fn increment_function(args: Vec<Argument>) -> JuizResult<Output> {
    let i = arg(&args, "arg1")?.as_i64().unwrap();
    return Ok(Output::new(jvalue!(i+1)));
}

#[no_mangle]
pub unsafe extern "Rust" fn process_factory() -> JuizResult<Arc<Mutex<dyn ProcessFactory>>> {
    env_logger::init();
    
    create_process_factory(manifest(), increment_function)
}
