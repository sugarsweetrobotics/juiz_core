use std::sync::{Arc, Mutex};

use crate::prelude::*;

pub struct SubSystemProxy {
    broker_proxy: Arc<Mutex<dyn BrokerProxy>>,
}

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