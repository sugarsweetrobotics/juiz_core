
extern crate juiz_core;
use crate::juiz_core::value::*;
use crate::juiz_core::error::*;
use crate::juiz_core::process_impl::*;
use crate::juiz_core::process_factory_impl::ProcessFactoryImpl;
    
#[allow(dead_code)]
fn increment_function(v: Value) -> Result<Value, JuizError> {
    let i = v["arg1"].as_i64().unwrap();
    return Ok(jvalue!(i+1));
}

fn new_increment_process<'a> () -> ProcessImpl {
    let manifest = serde_json::json!({
        "name": "test_function",
        "arguments" : {
            "arg1": {
                "description": "test_argument",
                "default": 1,
            }, 
        }, 
    });
    let p = ProcessImpl::new(manifest, increment_function);
    assert!(p.is_ok());
    p.unwrap()
}

static mut COUNTER: i64 = 0;
  

#[allow(dead_code)]
fn execution_function(_v: Value) -> Result<Value, JuizError> {
    #[allow(unused)]
    let mut val: i64 = 0;
    unsafe {
        COUNTER = COUNTER + 1;
        val = COUNTER;
    }
    return Ok(jvalue!(val));
}

fn new_execution_process<'a> () -> ProcessImpl {
    let manifest = serde_json::json!({
        "name": "test_function",
        "arguments" : {
            "arg1": {
                "description": "test_argument",
                "default": 1,
            }, 
        }, 
    });
    let p = ProcessImpl::new(manifest, execution_function);
    assert!(p.is_ok());
    p.unwrap()
}



#[test]
fn simple_process_create_test() {
    let manifest = serde_json::json!({
        "type_name" : "increment",
        "arguments" : {
            "arg1": {
                "description": "test_argument",
                "default": 1,
            }, 
        }, 
    });
    let result_pf = ProcessFactoryImpl::new(manifest, increment_function);
    assert!(result_pf.is_ok());
    let p = result_pf.ok().unwrap().borrow_mut().create_process("hoge".to_string());
    assert!(p.is_ok(), "ProcessImpl::new() failed. Error is {:?}", p.err());
    let result = p.ok().unwrap().borrow().call(jvalue!({"arg1": 3}));
    assert!(result.is_ok());
    let res_value = result.ok().unwrap();
    assert!(res_value.is_i64());
    assert!(res_value.as_i64().unwrap() == 4);
}
