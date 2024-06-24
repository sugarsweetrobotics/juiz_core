

use juiz_core::{jvalue, processes::capsule::{Capsule, CapsuleMap}, JuizResult, Value};


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

#[no_mangle]
pub unsafe extern "Rust" fn increment_process(args: CapsuleMap) -> JuizResult<Capsule> {
    let v = args.get("arg1")?;
    let i = v.lock_as_value(|value| { value.as_i64().unwrap() })?;
    return Ok(jvalue!(i+1).into());
}

//#[no_mangle]
//pub unsafe extern "Rust" fn process_factory() -> JuizResult<Arc<Mutex<dyn ProcessFactory>>> {
//    env_logger::init();
//    
//    create_process_factory(manifest(), increment_function)
//}
