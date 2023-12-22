use std::sync::{Arc, RwLock};

use crate::{JuizResult, Value};

use super::ExecutionContext;


pub trait ExecutionContextFactory {

    fn type_name(&self) -> &str;

    fn create(&self, manifest: Value) -> JuizResult<Arc<RwLock<dyn ExecutionContext>>>;
}

