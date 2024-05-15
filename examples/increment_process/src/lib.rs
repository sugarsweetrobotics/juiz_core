use std::sync::{Mutex, Arc};

use juiz_core::{create_process_factory, jvalue, processes::capsule::{Capsule, CapsuleMap}, utils::juiz_lock, JuizResult, ProcessFactory, Value};


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


fn increment_function(args: CapsuleMap) -> JuizResult<Capsule> {
    let v = args.get("arg1")?;
    let i = juiz_lock(&v)?.as_value().unwrap().as_i64().unwrap();
    return Ok(jvalue!(i+1).into());
        
    //let i = arg(&args, "arg1")?.as_i64().unwrap();
    //return Ok(jvalue!(i+1).into());
}

#[no_mangle]
pub unsafe extern "Rust" fn process_factory() -> JuizResult<Arc<Mutex<dyn ProcessFactory>>> {
    env_logger::init();
    
    create_process_factory(manifest(), increment_function)
}
