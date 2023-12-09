extern crate juiz_core;
use std::sync::{Arc, Mutex};

use crate::juiz_core::*;

#[allow(dead_code)]
fn increment_function(v: Value) -> JuizResult<Value> {
    let i = v["arg1"].as_i64().unwrap();
    return Ok(jvalue!(i+1));
}


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
    let result_pf = cb.register_process_factory(manifest, increment_function);
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

#[cfg(test)]
#[test]
fn core_broker_process_factory_integration_test() {
    let mut cb = new_core_broker();
    let _pf = new_process_factory(&mut cb);

    //let mut id = "".to_string();

    let p_result = cb.create_process(jvalue!({
        "name": "test_function",
        "type_name": "increment",
    }));
    assert!(p_result.is_ok(), "process_create failed. Error is {:?}", p_result.err());

    let arc_p = p_result.ok().unwrap();
    let p = arc_p.lock().unwrap();
    
    let id = p.identifier().clone();
    

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



#[cfg(test)]
#[test]
fn core_broker_process_factory_integration_connection_test() {
    let mut cb = new_core_broker();
    let _pf = new_process_factory(&mut cb);
    
    let p1_result = cb.create_process(jvalue!({
        "name": "test_function1",
        "type_name": "increment",
    }));
    assert!(p1_result.is_ok(), "process_create failed. Error is {:?}", p1_result.err());

    let arc_p1 = p1_result.ok().unwrap();
    
    let id1 = arc_p1.lock().unwrap().identifier().clone();

    let p2_result = cb.create_process(jvalue!({
        "name": "test_function2",
        "type_name": "increment",
    }));
    assert!(p2_result.is_ok(), "process_create failed. Error is {:?}", p2_result.err());

    let arc_p2 = p2_result.ok().unwrap();
    let id2 = arc_p2.lock().unwrap().identifier().clone();
    
    assert!(cb.is_in_charge_for_process(&id1));
    assert!(cb.is_in_charge_for_process(&id2));
    

    let con_result = cb.connect_process_to(&id1,
         &"arg1".to_string(), 
         &id2, 
        jvalue!({
            "id": "con01",
        }));
    assert!(con_result.is_ok(), "CoreBroker::connect() failed. Error is {:?}", con_result.err());

    let retval = cb.execute_process(&id1);
    match retval {
        Ok(vv) => {
            assert_eq!(vv.as_i64().unwrap(), 2);
        }, 
        Err(ev) => {
            print!("Return value is {:?}", ev);
        }
    }
    let p2_result2 = cb.process(&id2);
    assert!(p2_result2.is_ok(), "Process 2 can not acquire. Error is {:?}", p2_result2.err());
    
    let output = p2_result2.ok().unwrap().lock().unwrap().get_output();
    assert!(output.is_some(), "Error. Process2 Output is None.");

    //
    // 1 (default) -> proc1 -> 2 -> procec2 -> 3. Answer must be 3.
    assert_eq!(output, Some(jvalue!(3)), "Error. Execution output of Process 2 is wrong.");

}
