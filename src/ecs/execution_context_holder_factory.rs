use std::sync::{Arc, Mutex};

use crate::{JuizObject, object::{JuizObjectCoreHolder, ObjectCore, JuizObjectClass}, JuizResult, utils::juiz_lock, Value, core::Plugin};

use super::{execution_context_holder::ExecutionContextHolder, execution_context_factory::ExecutionContextFactory};

#[allow(unused)]
pub struct ExecutionContextHolderFactory {
    core: ObjectCore,
    ec_factory: Arc<Mutex<dyn ExecutionContextFactory>>,
    plugin: Plugin,
}

impl ExecutionContextHolderFactory {
    pub fn new(plugin: Plugin, ec_factory: Arc<Mutex<dyn ExecutionContextFactory>>) -> JuizResult<Arc<Mutex<ExecutionContextHolderFactory>>> {
        let type_name = juiz_lock(&ec_factory)?.type_name().to_string();
        Ok(Arc::new(Mutex::new(
            ExecutionContextHolderFactory{
                plugin,
                core: ObjectCore::create_factory(JuizObjectClass::ExecutionContextFactory("ExecutionContextHolderFactory"), type_name.as_str()),
                ec_factory,
            }
        )))
    }


    pub fn create(&self, manifest: Value) -> JuizResult<Arc<Mutex<ExecutionContextHolder>>> {
        let f = juiz_lock(&self.ec_factory)?;
        Ok(
            ExecutionContextHolder::new(
                f.type_name(), 
                f.create(manifest)?
            )?
        )
    }
}

impl JuizObjectCoreHolder for ExecutionContextHolderFactory {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}

impl JuizObject for ExecutionContextHolderFactory {
    
}