extern crate juiz_core;
use std::sync::Arc;

use std::sync::RwLock;



use crate::juiz_core::*;
use crate::juiz_core::connections::connect;

mod common;


fn setup() -> (ProcessPtr, ProcessPtr) {

    let p1 = common::new_increment_process("process1");
    let p2 = common::new_increment_process("process2");

    let rp1: ProcessPtr = Arc::new(RwLock::new(p1));
    let rp2: ProcessPtr = Arc::new(RwLock::new(p2));

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
    let result1 = rp2.write().unwrap().notify_connected_from(Arc::clone(&rp1), &"arg1".to_string(), manifeset.clone());
    assert!(result1.is_ok(), "Failed to connected_from function. Error is {:?}", result1.err());
    let result2 = rp1.write().unwrap().try_connect_to(Arc::clone(&rp2), &"arg1".to_string(), manifeset.clone());
    assert!(result2.is_ok(), "Failed to connect_to function. Error is {:?}", result2.err());

    let result = rp2.read().unwrap().invoke().unwrap();

    let iv = result.lock_as_value(|value| { value.as_i64().unwrap() }).unwrap();
    assert_eq!(iv, 3);
}

#[test]
fn simple_connection_execute_test() {
    let (rp1, rp2) = setup();


    let manifeset =jvalue!({
        "id": "con1",
        "type": "push",
    });

    // rp1 -> rp2
    let result1 = rp2.write().unwrap().notify_connected_from(Arc::clone(&rp1), &"arg1".to_string(), manifeset.clone());
    assert!(result1.is_ok(), "Failed to connected_from function. Error is {:?}", result1.err());
    let result2 = rp1.write().unwrap().try_connect_to(Arc::clone(&rp2), &"arg1".to_string(), manifeset.clone());
    assert!(result2.is_ok(), "Failed to connect_to function. Error is {:?}", result2.err());

    //let p =  
    let result_old = rp2.read().unwrap().get_output();
    //println!("hogehoge= {:?}", result_old.is_none());
    let f = result_old.is_empty().unwrap();
    assert!(f == true);

    let result1 = rp1.read().unwrap().execute();
    assert!(result1.is_ok(), "Error of ConnectionRack.execute(). Error is {:?}", result1.err());
    //let arc = result1.unwrap();
    let output = rp2.read().unwrap().get_output();
    //let result = juiz_lock(&output).unwrap();
    //assert_eq!(result.is_some(), true);
    //let v = result.unwrap();
    println!("value = {:?}", output);
    let iv = output.lock_as_value(|value| { value.as_i64().unwrap() }).unwrap();
    assert_eq!(iv, 3);
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
    
    let result = rp2.read().unwrap().invoke();
    assert!(result.is_ok());
    let arc = result.unwrap();

    let iv = arc.lock_as_value(|value| { value.as_i64().unwrap() }).unwrap();
    assert_eq!(iv, 3);
}