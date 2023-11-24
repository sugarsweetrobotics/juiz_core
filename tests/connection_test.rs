extern crate juiz_core;
use std::cell::RefCell;
use std::rc::Rc;

use crate::juiz_core::value::*;
use crate::juiz_core::error::*;
use crate::juiz_core::process_impl::*;
use juiz_core::process::Process;
    
#[allow(dead_code)]
fn increment_function(v: Value) -> Result<Value, JuizError> {
    let i = v["arg1"].as_i64().unwrap();
    return Ok(jvalue!(i+1));
}

fn new_increment_process<'a> (name: String) -> ProcessImpl<'a> {
    let manifest = serde_json::json!({
        "name": name,
        "arguments" : {
            "arg1": {
                "description": "test_argument",
                "default": 1,
            }, 
        }, 
    });
    let mut p = ProcessImpl::new(manifest, increment_function);
    assert!(p.is_ok());
    p.unwrap()
}

#[cfg(test)]
#[test]
fn simple_connection_test() {
    use std::borrow::BorrowMut;

    use juiz_core::process::Connectable;

    let mut p1 = new_increment_process("process1".to_string());
    let mut p2 = new_increment_process("process2".to_string());

    let rp: &Rc<RefCell<&mut dyn Process>> = &Rc::new(RefCell::new(p1.borrow_mut()));
    p2.connected_from(rp, &"arg1".to_string());
    let result = p2.invoke();
    assert_eq!(result.unwrap().as_i64().unwrap(), 3);
}


