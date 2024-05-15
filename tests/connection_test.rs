extern crate juiz_core;
use std::sync::Arc;

use std::sync::RwLock;

use juiz_core::utils::juiz_lock;

use crate::juiz_core::*;
use crate::juiz_core::connections::connect;

mod common;


fn setup() -> (ProcessPtr, ProcessPtr){

    let p1 = common::new_increment_process("process1");
    let p2 = common::new_increment_process("process2");

    let rp1: ProcessPtr = Arc::new(RwLock::new(p1));
    let rp2: ProcessPtr = Arc::new(RwLock::new(p2));

    return (rp1, rp2);

}

#[cfg(test)]
#[test]
fn simple_connection_invoke_test() {
    use juiz_core::utils::juiz_lock;

    let (rp1, rp2) = setup();

    let manifeset =jvalue!({
        "id": "con1"
    });
    // rp1 -> rp2
    let result1 = rp2.write().unwrap().notify_connected_from(Arc::clone(&rp1), &"arg1".to_string(), manifeset.clone());
    assert!(result1.is_ok(), "Failed to connected_from function. Error is {:?}", result1.err());
    let result2 = rp1.write().unwrap().try_connect_to(Arc::clone(&rp2), &"arg1".to_string(), manifeset.clone());
    assert!(result2.is_ok(), "Failed to connect_to function. Error is {:?}", result2.err());

    let result = rp2.read().unwrap().invoke();
    assert_eq!(juiz_lock(&result.unwrap()).unwrap().as_value().unwrap().as_i64().unwrap(), 3);
}

#[test]
fn simple_connection_execute_test() {
    let (rp1, rp2) = setup();


    let manifeset =jvalue!({
        "id": "con1"
    });

    // rp1 -> rp2
    let result1 = rp2.write().unwrap().notify_connected_from(Arc::clone(&rp1), &"arg1".to_string(), manifeset.clone());
    assert!(result1.is_ok(), "Failed to connected_from function. Error is {:?}", result1.err());
    let result2 = rp1.write().unwrap().try_connect_to(Arc::clone(&rp2), &"arg1".to_string(), manifeset.clone());
    assert!(result2.is_ok(), "Failed to connect_to function. Error is {:?}", result2.err());

    //let p =  
    let result_old = rp2.read().unwrap().get_output();
    //println!("hogehoge= {:?}", result_old.is_none());
    let f = juiz_lock(&result_old).unwrap().is_empty();
    assert!(f == true);

    let result1 = rp1.read().unwrap().execute();
    assert!(result1.is_ok(), "Error of ConnectionRack.execute(). Error is {:?}", result1.err());
    //let arc = result1.unwrap();
    let output = rp2.read().unwrap().get_output();
    let result = juiz_lock(&output).unwrap();
    //assert_eq!(result.is_some(), true);
    //let v = result.unwrap();
    //println!("value = {:?}", result.as_value());
    assert_eq!(result.as_value().unwrap().as_i64().unwrap(), 3);
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
    let output = juiz_lock(&arc).unwrap();
    assert!(output.is_value());
    let value = output.as_value();
    assert!(value.is_some());
    let i64_val = value.unwrap();
    println!("i64_val={}",i64_val);
    assert!(i64_val.is_i64());
    assert_eq!(i64_val.as_i64().unwrap(), 3);
}