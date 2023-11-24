extern crate juiz_core;
use crate::juiz_core::{Value, jvalue};
use crate::juiz_core::error::*;
use crate::juiz_core::process_impl::*;
use crate::juiz_core::process::Process;
use crate::juiz_core::broker::*;
use crate::juiz_core::core_broker::*;

#[allow(dead_code)]
fn increment_function(v: Value) -> Result<Value, JuizError> {
    let i = v["arg1"].as_i64().unwrap();
    return Ok(jvalue!(i+1));
}

fn new_increment_process<'a> () -> ProcessImpl<'a>  {
    let manifest = serde_json::json!({
        "name": "test_function",
        "arguments" : {
            "arg1": {
                "type": "int",
                "description": "test_argument",
                "default": 1,
            }, 
        }, 
    });
    let p = ProcessImpl::new(manifest, increment_function);
    assert!(p.is_ok());
    p.unwrap()
}


#[cfg(test)]
#[test]
fn core_broker_test() {
    let mut cb = CoreBroker::new(jvalue!(
        {
            "name": "core_broker"
        }
    ));

    let p = new_increment_process();
    let id = p.identifier();

    cb.push_process(Box::new(p));

    assert!(cb.is_in_charge_for_process(&id));

    let retval = cb.call_process(&id, jvalue!({
        "arg1": 1,
    }));
    match retval {
        Ok(vv) => {
            assert_eq!(vv.as_i64().unwrap(), 2);
        }, 
        Err(ev) => {
            print!("Return value is {:?}", ev);
        }
    }

}
