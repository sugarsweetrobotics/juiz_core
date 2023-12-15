use std::sync::{Mutex, Arc};



use crate::{JuizResult, utils::juiz_lock, object::{ObjectCore, JuizObjectClass, JuizObjectCoreHolder}, JuizObject, Process, Value, value::obj_merge_mut};

use super::{execution_context::ExecutionContext, execution_context_core::ExecutionContextCore};

pub struct ExecutionContextHolder {
    object_core: ObjectCore,
    core: Arc<Mutex<ExecutionContextCore>>,
    execution_context: Arc<Mutex<dyn ExecutionContext>>,
}

impl ExecutionContextHolder {

    pub fn new(type_name: &str, ec: Arc<Mutex<dyn ExecutionContext>>) -> JuizResult<Arc<Mutex<ExecutionContextHolder>>> {
        Ok(Arc::new(Mutex::new(
            ExecutionContextHolder { 
                object_core: ObjectCore::create(JuizObjectClass::ExecutionContext("ExecutionContext"), 
                    type_name, 
                    juiz_lock(&ec)?.name()), 
                core: ExecutionContextCore::new(), 
                execution_context: ec.clone() }
        )))
    }
        
    pub fn start(&mut self) -> JuizResult<()> {
        juiz_lock(&self.execution_context)?.on_starting(self.core.clone())
    }

    pub fn stop(&mut self) -> JuizResult<()> {
        juiz_lock(&self.execution_context)?.on_stopping(Arc::clone(&self.core))
    }

    pub fn bind(&mut self, target_process: Arc<Mutex<dyn Process>>) -> JuizResult<()> {
        juiz_lock(&self.core)?.bind(target_process)
    }
}

impl JuizObjectCoreHolder for ExecutionContextHolder {
    fn core(&self) -> &ObjectCore {
        &self.object_core
    }
}

impl JuizObject for ExecutionContextHolder {

    fn profile_full(&self) -> JuizResult<Value> {
        let mut v = self.object_core.profile_full()?;
        let ecv = juiz_lock(&self.execution_context)?.profile()?;
        obj_merge_mut(&mut v, &ecv)?;

        let cv = juiz_lock(&self.core)?.profile()?;
        obj_merge_mut(&mut v, &cv)?;
        Ok(v)
    }
}