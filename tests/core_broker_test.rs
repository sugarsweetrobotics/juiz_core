extern crate juiz_core;

use juiz_core::prelude::*;
//use crate::juiz_core::processes::process_impl::*;

use std::sync::Arc;

mod common;

fn new_increment_process<'a> () -> ProcessImpl  {
    let manifest = serde_json::json!({
        "name": "test_function",
        "type_name": "increment",
        "arguments" : {
            "arg1": {
                "type": "int",
                "description": "test_argument",
                "default": 1,
            }, 
        }, 
    });
    let p = ProcessImpl::new(manifest, common::increment_function);
    assert!(p.is_ok() , "ProcessImpl::new() failed. Error is {:?}", p.err());
    p.unwrap()
}


#[cfg(test)]
#[test]
fn core_broker_test() {
    use std::sync::RwLock;

    use juiz_core::{SystemStore, SystemStorePtr};

    //use juiz_core::brokers::broker_proxy::ProcessBrokerProxy;

    
    let result = CoreBroker::new(jvalue!(
        {
            "name": "core_broker"
        }
    ), SystemStorePtr::new(SystemStore::new()));
    if result.is_err() {
        assert!(false, "CoreBroker::new failed. {:?}", result.err())
    }

    let mut cb = result.unwrap();

    let p = new_increment_process();
    let id = p.identifier().clone();

    let result = cb.store_mut().processes.register(Arc::new(RwLock::new(p)));

    assert!(result.is_ok());

    //assert!(cb.is_in_charge_for_process(&id));

    let retval = cb.process_call(&id, vec!(("arg1", jvalue!(1))).into());
    match retval {
        Ok(arc) => {
            let iv = arc.lock_as_value(|value| { value.as_i64().unwrap() }).unwrap();
            assert_eq!(iv, 2);
        }, 
        Err(ev) => {
            print!("Return value is {:?}", ev);
        }
    }

}
