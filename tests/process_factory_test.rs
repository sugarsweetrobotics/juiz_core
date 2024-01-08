
extern crate juiz_core;
use juiz_core::JuizResult;

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
    let result_pf = ProcessFactoryImpl::new(manifest, common::increment_function);
    assert!(result_pf.is_ok());
    let proc_manifest = jvalue!(
        {
            "name": "hogehoge",
        }
    );
    let p = result_pf.ok().unwrap().lock().unwrap().create_process(proc_manifest);
    assert!(p.is_ok(), "ProcessImpl::new() failed. Error is {:?}", p.err());
    let result = p.ok().unwrap().lock().unwrap().call(jvalue!({"arg1": 3}));
    assert!(result.is_ok());
    let res_value = result.ok().unwrap();
    assert!(res_value.value.is_i64());
    assert!(res_value.value.as_i64().unwrap() == 4);
}
