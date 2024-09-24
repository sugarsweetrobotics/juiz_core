use std::sync::{Arc, Mutex};

use juiz_core::prelude::*;

use juiz_core::{ExecutionContext, ExecutionContextCore};

pub struct OneShotEC {
    name: String
}

impl OneShotEC {
    pub fn new(name: &str) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self{
            name: name.to_string(),
        }))
    }
}

impl ExecutionContext for OneShotEC {
    

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn type_name(&self) -> &str {
        "OneShotEC"
    }

    fn profile(&self) -> JuizResult<Value> {
        todo!()
    }

    fn execute(&self, core: &Arc<Mutex<ExecutionContextCore>>) -> JuizResult<bool> {
        juiz_lock(&core)?.svc().and(Ok(false))
    }

}