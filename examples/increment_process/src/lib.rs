use std::sync::{Mutex, Arc};

use juiz_core::{jvalue, JuizResult, Value, ProcessFactory, create_process_factory};


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


fn increment_function(v: Value) -> JuizResult<Value> {
    let i = v["arg1"].as_i64().unwrap();
    return Ok(jvalue!(i+1));
}

#[no_mangle]
pub unsafe extern "Rust" fn process_factory() -> JuizResult<Arc<Mutex<dyn ProcessFactory>>> {
    env_logger::init();
    
    create_process_factory(manifest(), increment_function)
}
