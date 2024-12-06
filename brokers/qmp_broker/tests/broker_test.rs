use std::sync::{Arc, Mutex};


use juiz_core::{prelude::*, SystemStore, SystemStorePtr};
use juiz_core::{CRUDBroker, futures, prelude::*, tokio};

extern crate qmp_broker;
extern crate juiz_core;
#[test]
fn broker_test() {
    let system_store = SystemStorePtr::new(SystemStore::new());
    let core = CoreBrokerPtr::new(CoreBroker::new(jvalue!({}), system_store).unwrap());
    let manifest = jvalue!({
        "type_name": "qmp",
        "name": "qmp_broker0",
        "host": "127.0.0.1",
        "port": 5001
    });
    let crud = Arc::new(Mutex::new(CRUDBroker::new(core, manifest).unwrap()));
    
    //let result = futures::executor::block_on(qmp_broker::on_start(manifest, crud));

    assert_eq!(true, true);
}