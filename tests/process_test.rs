
mod identifier_test;

extern crate juiz_core;
use crate::juiz_core::*;
use crate::juiz_core::processes::process_impl::*;

mod common;

  

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
    let p = ProcessImpl::new(manifest, common::increment_function);
    assert!(p.is_err());
    // assert!(p.err() == Some(JuizError::ManifestNameMissingError{}));
}

#[test]
fn no_arguments_manifest_process_test() {
    let manifest = serde_json::json!({
        "name": "hoge",
        "type_name": "increment",
    });
    let p = ProcessImpl::new(manifest, common::increment_function);
    assert!(p.is_err());
    // assert!(p.err() == Some(JuizError::ManifestArgumentsMissingError{}));
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
    let p = ProcessImpl::new(manifest, common::increment_function);
    assert!(p.is_err());
    let _e = p.err();
    // assert!(e == Some(JuizError::ManifestArgumentDefaultValueMissingError{}), "Error is {:?})", e);
}

#[cfg(test)]
#[test]
fn call_process_test() {
    match common::new_increment_process("incremnet").call(jvalue!({
        "arg1": 1,
    })) {
        Ok(vv) => {
            assert_eq!(vv.get_value().unwrap().as_i64().unwrap(), 2);
        }, 
        Err(ev) => {
            print!("Return value is {:?}", ev);
        }
    }
}

#[cfg(test)]
#[test]
fn invoke_process_test() {
    match common::new_increment_process("increment").invoke() {
        Ok(vv) => {
            assert_eq!(vv.get_value().unwrap().as_i64(), Some(2), "Error. vv is {:?}", vv.get_value().unwrap().to_string());
        }, 
        Err(ev) => {
            print!("Return value is {:?}", ev);
        }
    }
}


#[cfg(test)]
#[test]
fn execute_process_test() {
    match common::new_execution_process("execute").execute() {
        Ok(vv) => {
            assert_eq!(vv.get_value().unwrap().as_i64(), Some(1), "Error. vv is {:?}", vv.get_value().unwrap().to_string());
        }, 
        Err(ev) => {
            print!("Return value is {:?}", ev);
        }
    }
}

#[cfg(test)]
#[test]
fn call_invalid_argument_process_test() {
    match common::new_increment_process("increment").call(jvalue!({
        "arg2": 1,
    })) {
        Ok(_vv) => {
            assert!(false, "Process must be return error.");
        }, 
        Err(ev) => {
            print!("Return value is {:?}", ev);
            // assert_eq!(ev, JuizError::ArgumentMissingWhenCallingError{});
        }
    }
}