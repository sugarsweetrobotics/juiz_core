

use juiz_core::prelude::*;
use juiz_core::{processes::process_impl::ProcessImpl};


#[allow(dead_code)]
pub fn add_function(v: CapsuleMap) -> JuizResult<Capsule> {
    //let i = arg(&v, "arg1")?.as_i64().unwrap();
    //let iv = juiz_lock(&v.get("arg1").unwrap())?.as_value().unwrap().as_i64().unwrap();
    let iv1 = v.get("arg1")?.lock_as_value(|value| {
        value.as_i64().unwrap()
    } )?;
    let iv2 = v.get("arg2")?.lock_as_value(|value| {
        value.as_i64().unwrap()
    } )?;
    return Ok(jvalue!(iv1+iv2).into());
}

#[allow(dead_code)]
pub fn new_add_process<'a> (name: &str) -> ProcessImpl {
    let manifest = serde_json::json!({
        "name": name,
        "type_name": "add",
        "arguments" : {
            "arg1": {
                "description": "test_argument_1",
                "default": 1,
            }, 
            "arg2": {
                "description": "test_argument_2",
                "default": 1,
            }, 
        }, 
    });
    let p = ProcessImpl::new(manifest, add_function);
    assert!(p.is_ok(), "ProcessImpl::new() failed. Error is '{:?}'", p.err());
    p.unwrap()
}



#[allow(dead_code)]
pub fn increment_function(v: CapsuleMap) -> JuizResult<Capsule> {
    //let i = arg(&v, "arg1")?.as_i64().unwrap();
    //let iv = juiz_lock(&v.get("arg1").unwrap())?.as_value().unwrap().as_i64().unwrap();
    let iv = v.get("arg1")?.lock_as_value(|value| {
        value.as_i64().unwrap()
    } )?;
    return Ok(jvalue!(iv+1).into());
}

#[allow(dead_code)]
pub fn new_increment_process<'a> (name: &str) -> ProcessImpl {
    let manifest = serde_json::json!({
        "name": name,
        "type_name": "increment",
        "arguments" : {
            "arg1": {
                "description": "test_argument",
                "default": 1,
            }, 
        }, 
    });
    let p = ProcessImpl::new(manifest, increment_function);
    assert!(p.is_ok(), "ProcessImpl::new() failed. Error is '{:?}'", p.err());
    p.unwrap()
}

static mut COUNTER: i64 = 0;

#[allow(dead_code)]
pub fn execution_function(_v: CapsuleMap) -> JuizResult<Capsule> {
    #[allow(unused)]
    let mut val: i64 = 0;
    unsafe {
        COUNTER = COUNTER + 1;
        val = COUNTER;
    }
    return Ok(jvalue!(val).into());
}

#[allow(dead_code)]
pub fn new_execution_process<'a> (name: &str) -> ProcessImpl {
    let manifest = serde_json::json!({
        "name": "test_function",
        "type_name": name,
        "arguments" : {
            "arg1": {
                "description": "test_argument",
                "default": 1,
            }, 
        }, 
    });
    let p = ProcessImpl::new(manifest, execution_function);
    assert!(p.is_ok(), "ProcessImpl::new() failed. Error is {:?}", p.err());
    p.unwrap()
}
