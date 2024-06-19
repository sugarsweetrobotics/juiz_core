
use std::sync::{Arc, Mutex};

use crate::{ContainerPtr, JuizObject, JuizResult, ProcessPtr, Value};


pub trait ContainerProcessFactory : JuizObject {
    fn create_container_process(&self, container: ContainerPtr, manifest: Value) -> JuizResult<ProcessPtr>;
}

pub type ContainerProcessFactoryPtr = Arc<Mutex<dyn ContainerProcessFactory>>;