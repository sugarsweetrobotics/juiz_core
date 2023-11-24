
extern crate juiz_core;
use crate::juiz_core::value::*;
use crate::juiz_core::error::*;
use crate::juiz_core::process_impl::*;
use juiz_core::process::Process;
    
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
                "description": "test_argument",
                "default": 1,
            }, 
        }, 
    });
    let p = ProcessImpl::new(manifest, increment_function);
    assert!(p.is_ok());
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
    assert!(p.unwrap_err() == JuizError::ManifestNameMissingError{});
}

#[test]
fn no_arguments_manifest_process_test() {
    let manifest = serde_json::json!({
        "name": "hoge",
        
    });
    let p = ProcessImpl::new(manifest, increment_function);
    assert!(p.is_err());
    assert!(p.unwrap_err() == JuizError::ManifestArgumentsMissingError{});
}


#[test]
fn no_default_manifest_process_test() {
    let manifest = serde_json::json!({
        "name": "hoge",
        "arguments": {
            "arg1": {
                "description": "test_argument",
            }, 
        }
    });
    let p = ProcessImpl::new(manifest, increment_function);
    assert!(p.is_err());
    let e = p.unwrap_err();
    assert!(e == JuizError::ManifestArgumentDefaultValueMissingError{}, "Error is {:?})", e);
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