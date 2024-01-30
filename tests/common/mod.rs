use juiz_core::{jvalue, Value, JuizResult, processes::{process_impl::ProcessImpl, arg, Output}, Argument};




#[allow(dead_code)]
pub fn increment_function(v: Vec<Argument>) -> JuizResult<Output> {
    let i = arg(&v, "arg1")?.as_i64().unwrap();
    return Ok(Output::new_from_value(jvalue!(i+1)));
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
pub fn execution_function(_v: Vec<Argument>) -> JuizResult<Output> {
    #[allow(unused)]
    let mut val: i64 = 0;
    unsafe {
        COUNTER = COUNTER + 1;
        val = COUNTER;
    }
    return Ok(Output::new_from_value(jvalue!(val)));
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
