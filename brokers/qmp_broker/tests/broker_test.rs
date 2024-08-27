use std::sync::{Arc, Mutex};

use juiz_core::{brokers::CRUDBroker, futures, prelude::*, tokio, CoreBroker};

extern crate qmp_broker;
extern crate juiz_core;
#[test]
fn broker_test() {
    let core = Arc::new(Mutex::new(CoreBroker::new(jvalue!({})).unwrap()));
    let crud = Arc::new(Mutex::new(CRUDBroker::new(core).unwrap()));
    let manifest = jvalue!({
        "type_name": "qmp",
        "name": "qmp_broker0",
        "host": "127.0.0.1",
        "port": 5000
    });
    let result = futures::executor::block_on(qmp_broker::on_start(manifest, crud));

    assert_eq!(true, true);
}