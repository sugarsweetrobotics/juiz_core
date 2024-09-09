use std::sync::{Arc, Mutex};

use crate::prelude::*;

pub trait ProcessFactory: JuizObject {
    fn create_process(&self, manifest: Value) -> JuizResult<ProcessPtr>;
}

pub type ProcessFactoryPtr = Arc<Mutex<dyn ProcessFactory>>;
