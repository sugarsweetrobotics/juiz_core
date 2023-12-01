use std::sync::{Mutex, Arc};

use crate::{process::Process, error::JuizError, Value};







pub trait ProcessFactory {

    fn type_name(&self) -> &str;

    fn create_process<T>(&self, name: T, manifest: Value) -> Result<Arc<Mutex<dyn Process>>, JuizError>;


}