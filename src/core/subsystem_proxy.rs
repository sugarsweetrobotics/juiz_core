use std::sync::{Arc, Mutex};

use crate::prelude::*;

#[allow(unused)]
pub struct SubSystemProxy {
    broker_proxy: Arc<Mutex<dyn BrokerProxy>>,
}

#[allow(unused)]
fn assert_subsystem_manifest(manifest: Value) -> JuizResult<Value> {
    Ok(manifest)
}

impl SubSystemProxy {

    pub fn new(broker_proxy: Arc<Mutex<dyn BrokerProxy>>) -> JuizResult<Self> {
        Ok(SubSystemProxy{ 
            broker_proxy
        })
    }
}