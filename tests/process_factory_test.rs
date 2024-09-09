
extern crate juiz_core;

use juiz_core::prelude::*;
use std::sync::{Arc, Mutex};

use juiz_core::utils::juiz_lock;

use crate::juiz_core::value::*;
use crate::juiz_core::processes::process_factory_impl::ProcessFactoryImpl;

mod common;

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
    let result_pf = Arc::new(Mutex::new(ProcessFactoryImpl::new(manifest, common::increment_function).unwrap()));
    /*
    assert!(result_pf.is_ok());
    */
    let proc_manifest = jvalue!(
        {
            "name": "hogehoge",
        }
    );
    let p = juiz_lock(&result_pf).unwrap().create_process(proc_manifest);
    assert!(p.is_ok(), "ProcessImpl::new() failed. Error is {:?}", p.err());
    let result = p.ok().unwrap().read().unwrap().call(vec!(("arg1", jvalue!(3))).into());
    assert!(result.is_ok());
    let res_value = result.ok().unwrap();
    
    let iv = res_value.lock_as_value(|value| { value.as_i64().unwrap() }).unwrap();
    //assert!(juiz_lock(&res_value).unwrap().as_value().unwrap().is_i64());
    assert!(iv == 4);
}
