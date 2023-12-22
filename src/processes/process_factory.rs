use std::sync::{Mutex, Arc};

use crate::{Process, Value, JuizResult, JuizObject};


pub trait ProcessFactory: JuizObject {


    fn create_process(&self, manifest: Value) -> JuizResult<Arc<Mutex<dyn Process>>>;
}
