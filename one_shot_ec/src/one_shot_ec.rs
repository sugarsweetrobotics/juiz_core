use std::sync::{Arc, Mutex};

use juiz_core::{JuizResult, Value};
use juiz_core::utils::juiz_lock;

use juiz_core::ecs::{ExecutionContext, ExecutionContextCore};

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
    fn on_starting(&mut self, svc: Arc<Mutex<ExecutionContextCore>>) -> JuizResult<()> {
        juiz_lock(&svc)?.svc().and(Ok(()))
    }

    fn on_stopping(&mut self, _core: Arc<Mutex<ExecutionContextCore>>) -> JuizResult<()> {
        log::debug!("OneShotEC::on_stopping() called");
        
        log::debug!("OneShotEC stopped.");
        Ok(())
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn type_name(&self) -> &str {
        "OneShotEC"
    }

    fn profile(&self) -> JuizResult<Value> {
        todo!()
    }
}