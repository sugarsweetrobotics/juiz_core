extern crate juiz_core;
use std::sync::Arc;
use std::sync::Mutex;

use crate::juiz_core::*;
use juiz_core::processes::Process;
use crate::juiz_core::connections::connect;

mod common;


fn setup() -> (Arc<Mutex<dyn Process>>, Arc<Mutex<dyn Process>>){

    let p1 = common::new_increment_process("process1");
    let p2 = common::new_increment_process("process2");

    let rp1: Arc<Mutex<dyn Process>> = Arc::new(Mutex::new(p1));
    let rp2: Arc<Mutex<dyn Process>> = Arc::new(Mutex::new(p2));

    return (rp1, rp2);

}

#[cfg(test)]
#[test]
fn simple_connection_invoke_test() {
    let (rp1, rp2) = setup();

    let manifeset =jvalue!({
        "id": "con1"
    });
    // rp1 -> rp2
    let result1 = rp2.lock().unwrap().notify_connected_from(Arc::clone(&rp1), &"arg1".to_string(), manifeset.clone());
    assert!(result1.is_ok(), "Failed to connected_from function. Error is {:?}", result1.err());
    let result2 = rp1.lock().unwrap().try_connect_to(Arc::clone(&rp2), &"arg1".to_string(), manifeset.clone());
    assert!(result2.is_ok(), "Failed to connect_to function. Error is {:?}", result2.err());

    let result = rp2.lock().unwrap().invoke();
    assert_eq!(result.unwrap().as_i64().unwrap(), 3);
}

#[test]
fn simple_connection_execute_test() {
    let (rp1, rp2) = setup();


    let manifeset =jvalue!({
        "id": "con1"
    });

    // rp1 -> rp2
    let result1 = rp2.lock().unwrap().notify_connected_from(Arc::clone(&rp1), &"arg1".to_string(), manifeset.clone());
    assert!(result1.is_ok(), "Failed to connected_from function. Error is {:?}", result1.err());
    let result2 = rp1.lock().unwrap().try_connect_to(Arc::clone(&rp2), &"arg1".to_string(), manifeset.clone());
    assert!(result2.is_ok(), "Failed to connect_to function. Error is {:?}", result2.err());


    let result_old = rp2.lock().unwrap().get_output();
    assert!(result_old.is_none());

    let result1 = rp1.lock().unwrap().execute();
    assert!(result1.is_ok(), "Error of ConnectionRack.execute(). Error is {:?}", result1.err());
    let result = rp2.lock().unwrap().get_output();
    assert!(result.is_some());
    assert_eq!(result.unwrap().as_i64().unwrap(), 3);
}




#[cfg(test)]
#[test]
fn simple_connection_builder_invoke_test() {
    let (rp1, rp2) = setup();

    let manifest =jvalue!({
        "id": "con1"
    });

    let result1 = connect(Arc::clone(&rp1), Arc::clone(&rp2), &"arg1".to_string(), manifest);
    // rp1 -> rp2
    assert!(result1.is_ok(), "Failed to ConnectionBuilder::connected function. Error is {:?}", result1.err());
    
    let result = rp2.lock().unwrap().invoke();
    assert_eq!(result.unwrap().as_i64().unwrap(), 3);
}