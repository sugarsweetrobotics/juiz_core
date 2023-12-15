

use std::sync::{Mutex, Arc};



use crate::{jvalue, JuizResult, Process, utils::juiz_lock, Value, JuizError};


pub struct ExecutionContextCore {
    target_process: Option<Arc<Mutex<dyn Process>>>
}

impl ExecutionContextCore {

    pub fn new() -> Arc<Mutex<ExecutionContextCore>> {
        Arc::new(Mutex::new(ExecutionContextCore{
            target_process: None
        }))
    }

    pub fn bind(&mut self, target_process: Arc<Mutex<dyn Process>>) -> JuizResult<()> {
        self.target_process = Some(target_process);
        Ok(())
    }

    pub fn svc(&self) -> JuizResult<Value> {
        match &self.target_process {
            None => Err(anyhow::Error::from(JuizError::ExecutionContextCoreNotConnectedToProcessError{})),
            Some(tp) => juiz_lock(tp)?.execute()
        }
    }

    pub fn profile(&self) -> JuizResult<Value> {
        let target_id = match &self.target_process {
            None => "None".to_string(),
            Some(tp) => juiz_lock(tp)?.identifier().clone()
        };
        Ok(jvalue!({
            "target": target_id,
        }))
    }
}

