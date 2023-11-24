extern crate juiz_core;
use crate::juiz_core::value::*;
use crate::juiz_core::error::*;
use crate::juiz_core::process_impl::*;
use crate::juiz_core::process::Process;
use crate::juiz_core::process_rack::*;
use crate::juiz_core::process_rack_impl::*;
#[allow(dead_code)]
fn increment_function(v: Value) -> Result<Value, JuizError> {
    let i = v["arg1"].as_i64().unwrap();
    return Ok(jvalue!(i+1));
}

fn new_increment_process<'a> () -> ProcessImpl<'a> {
    let manifest = serde_json::json!({
        "name": "test_function",
        "arguments" : {
            "arg1": {
                "type": "int",
                "description": "test_argument",
                "default": 1,
            }
        }, 
    });
    let p = ProcessImpl::new(manifest, increment_function);
    assert!(p.is_ok());
    p.unwrap()
}


#[cfg(test)]
#[test]
fn process_rack_test() {
    let p = new_increment_process();
    let id = p.identifier();
    let mut r = ProcessRackImpl::new();
    r.push(Box::new(p));

    let poped_p  = r.process(&id);
    assert!(poped_p.is_some());

    let invalid_id = "hogehoge".to_string();
    let poped_n  = r.process(&invalid_id);
    assert!(poped_n.is_none(), "Must be None with Invalid Identifier");

}
