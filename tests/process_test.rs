
extern crate juiz_core;
use crate::juiz_core::*;
use crate::juiz_core::process::process_impl::*;
    
#[allow(dead_code)]
fn increment_function(v: Value) -> Result<Value, JuizError> {
    let i = v["arg1"].as_i64().unwrap();
    return Ok(jvalue!(i+1));
}

fn new_increment_process<'a> () -> ProcessImpl {
    let manifest = serde_json::json!({
        "name": "test_function",
        "type_name": "increment",
        "arguments" : {
            "arg1": {
                "description": "test_argument",
                "default": 1,
            }, 
        }, 
    });
    let p = ProcessImpl::new(manifest, increment_function);
    assert!(p.is_ok(), "ProcessImpl::new() failed. Error is {:?}", p.err());
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
        "type_name": "increment",
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



#[test]
fn no_name_manifest_process_test() {
    let manifest = serde_json::json!({
        "arguments" : {
            "arg1": {
                "description": "test_argument",
                "default": 1,
            }, 
        }, 
    });
    let p = ProcessImpl::new(manifest, increment_function);
    assert!(p.is_err());
    assert!(p.err() == Some(JuizError::ManifestNameMissingError{}));
}

#[test]
fn no_arguments_manifest_process_test() {
    let manifest = serde_json::json!({
        "name": "hoge",
        "type_name": "increment",
    });
    let p = ProcessImpl::new(manifest, increment_function);
    assert!(p.is_err());
    assert!(p.err() == Some(JuizError::ManifestArgumentsMissingError{}));
}


#[test]
fn no_default_manifest_process_test() {
    let manifest = serde_json::json!({
        "name": "hoge",
        "type_name": "increment",
        "arguments": {
            "arg1": {
                "description": "test_argument",
            }, 
        }
    });
    let p = ProcessImpl::new(manifest, increment_function);
    assert!(p.is_err());
    let e = p.err();
    assert!(e == Some(JuizError::ManifestArgumentDefaultValueMissingError{}), "Error is {:?})", e);
}

#[cfg(test)]
#[test]
fn call_process_test() {
    match new_increment_process().call(jvalue!({
        "arg1": 1,
    })) {
        Ok(vv) => {
            assert_eq!(vv.as_i64().unwrap(), 2);
        }, 
        Err(ev) => {
            print!("Return value is {:?}", ev);
        }
    }
}

#[cfg(test)]
#[test]
fn invoke_process_test() {
    match new_increment_process().invoke() {
        Ok(vv) => {
            assert_eq!(vv.as_i64(), Some(2), "Error. vv is {:?}", vv.to_string());
        }, 
        Err(ev) => {
            print!("Return value is {:?}", ev);
        }
    }
}


#[cfg(test)]
#[test]
fn execute_process_test() {
    match new_execution_process().execute() {
        Ok(vv) => {
            assert_eq!(vv.as_i64(), Some(1), "Error. vv is {:?}", vv.to_string());
        }, 
        Err(ev) => {
            print!("Return value is {:?}", ev);
        }
    }
}

#[cfg(test)]
#[test]
fn call_invalid_argument_process_test() {
    match new_increment_process().call(jvalue!({
        "arg2": 1,
    })) {
        Ok(_vv) => {
            assert!(false, "Process must be return error.");
        }, 
        Err(ev) => {
            print!("Return value is {:?}", ev);
            assert_eq!(ev, JuizError::ArgumentMissingWhenCallingError{});
        }
    }
}