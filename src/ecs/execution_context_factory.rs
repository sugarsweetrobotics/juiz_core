use std::sync::{Mutex, Arc};

use crate::{JuizResult, Value};

use super::ExecutionContext;


pub trait ExecutionContextFactory {

    fn type_name(&self) -> &str;

    fn create(&self, manifest: Value) -> JuizResult<Arc<Mutex<dyn ExecutionContext>>>;
}

