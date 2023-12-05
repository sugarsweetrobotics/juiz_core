use std::sync::{Mutex, Arc};

use crate::{Process, JuizError, Value};


pub trait ProcessFactory {

    fn type_name(&self) -> &str;

    fn create_process(&self, manifest: Value) -> Result<Arc<Mutex<dyn Process>>, JuizError>;

}