use std::sync::{Mutex, Arc};

use crate::{Process, Value, JuizResult};


pub trait ProcessFactory {

    fn type_name(&self) -> &str;

    fn create_process(&self, manifest: Value) -> JuizResult<Arc<Mutex<dyn Process>>>;

    fn profile_full(&self) -> JuizResult<Value>;
}