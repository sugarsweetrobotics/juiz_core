extern crate juiz_core;
use std::sync::{Arc, Mutex};

use juiz_core::{processes::process_factory_impl::ProcessFactoryImpl, utils::juiz_lock};

use crate::juiz_core::*;

mod common;


fn new_process_factory(cb: &mut CoreBroker) -> Arc<Mutex<dyn ProcessFactory>> {
    let manifest = serde_json::json!({
        "type_name" : "increment",
        "arguments" : {
            "arg1": {
                "description": "test_argument",
                "default": 1,
            }, 
        }, 
    });
    let result_pf = cb.store_mut().processes.register_factory(
        Arc::new(Mutex::new(ProcessFactoryImpl::new(manifest, common::increment_function).unwrap())));
    assert!(result_pf.is_ok(), "register_process_factory failed. Error is {:?}", result_pf.err());
    Arc::clone(&result_pf.ok().unwrap())
}

fn new_core_broker() -> CoreBroker {
    let result = CoreBroker::new(jvalue!(
        {
            "name": "core_broker"
        }
    ));
    
    assert!(result.is_ok(), "CoreBroker::new failed. {:?}", result.err());
    result.ok().unwrap()
}

//#[cfg(test)]
//#[test]
#[allow(dead_code)]
fn core_broker_process_factory_integration_test() {
    use juiz_core::brokers::broker_proxy::ProcessBrokerProxy;

    let mut cb = new_core_broker();
    let _pf = new_process_factory(&mut cb);

    //let mut id = "".to_string();

    let p_result = cb.create_process_ref(jvalue!({
        "name": "test_function",
        "type_name": "increment",
    }));
    assert!(p_result.is_ok(), "process_create failed. Error is {:?}", p_result.err());

    let arc_p = p_result.ok().unwrap();
    let p = arc_p.read().unwrap();
    
    let id = p.identifier().clone();
    

    //assert!(cb.is_in_charge_for_process(&id));

    let retval = cb.process_call(&id, vec!(("arg1", jvalue!(
        1
    ))).into());
    match retval {
        Ok(arc) => {
            let vv = juiz_lock(&arc).unwrap();
            assert_eq!(vv.as_value().unwrap().as_i64().unwrap(), 2);
        }, 
        Err(ev) => {
            print!("Return value is {:?}", ev);
        }
    }

}



#[cfg(test)]
#[test]
fn core_broker_process_factory_integration_connection_test() {
    use juiz_core::{brokers::broker_proxy::{ConnectionBrokerProxy, ProcessBrokerProxy}, utils::juiz_lock};

    let mut cb = new_core_broker();
    let _pf = new_process_factory(&mut cb);
    
    let p1_result = cb.create_process_ref(jvalue!({
        "name": "test_function1",
        "type_name": "increment",
    }));
    assert!(p1_result.is_ok(), "process_create failed. Error is {:?}", p1_result.err());

    let arc_p1 = p1_result.ok().unwrap();
    
    let id1 = arc_p1.read().unwrap().identifier().clone();

    let p2_result = cb.create_process_ref(jvalue!({
        "name": "test_function2",
        "type_name": "increment",
    }));
    assert!(p2_result.is_ok(), "process_create failed. Error is {:?}", p2_result.err());

    let arc_p2 = p2_result.ok().unwrap();
    let id2 = arc_p2.read().unwrap().identifier().clone();
    
    //assert!(cb.is_in_charge_for_process(&id1));
    //assert!(cb.is_in_charge_for_process(&id2));
    

    let con_result = cb.connection_create(
         jvalue!({
            "source": {
                "identifier": id1
            },
            "destination": {
                "identifier": id2,
            },
            "arg_name": "arg1"
        }));
    assert!(con_result.is_ok(), "CoreBroker::connect() failed. Error is {:?}", con_result.err());

    let retval = cb.process_execute(&id1);
    match retval {
        Ok(arc) => {
            let vv = juiz_lock(&arc).unwrap();
            assert_eq!(vv.as_value().unwrap().as_i64().unwrap(), 2);
        }, 
        Err(ev) => {
            print!("Return value is {:?}", ev);
        }
    }
    let p2_result2 = cb.store().processes.get(&id2);
    assert!(p2_result2.is_ok(), "Process 2 can not acquire. Error is {:?}", p2_result2.err());
    
    let arc_out = p2_result2.ok().unwrap().read().unwrap().get_output();
    let output = juiz_lock(&arc_out).unwrap();
    assert!(output.is_empty(), "Error. Process2 Output is None.");

    //
    // 1 (default) -> proc1 -> 2 -> procec2 -> 3. Answer must be 3.
    assert_eq!(output.as_value().unwrap().clone(), jvalue!(3), "Error. Execution output of Process 2 is wrong.");

}
