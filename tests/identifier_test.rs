


extern crate juiz_core;

use juiz_core::prelude::*;
#[cfg(test)]
#[test]
fn digest_identifier_test() {
    // use juiz_core::identifier::IdentifierStruct;


    let identifier = "core_broker://core/Process/hoge_func0::hoge_function";
    assert_eq!(IdentifierStruct::from(identifier.to_string()), IdentifierStruct{ 
        identifier: identifier.to_string(), 
        class_name: "Process".to_string(), 
        type_name: "hoge_function".to_string(),
        object_name: "hoge_func0".to_string(), 
        broker_name: "core".to_string(), 
        broker_type_name: "core_broker".to_string() });
}