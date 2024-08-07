
use std::sync::{Arc, Mutex};

use crate::{ContainerPtr, Value, JuizResult, JuizObject};


pub type ContainerConstructFunction<T>=fn(Value) -> JuizResult<Box<T>>;

pub trait ContainerFactory : JuizObject {

    fn create_container(&self, manifest: Value) -> JuizResult<ContainerPtr>;

    fn destroy_container(&mut self, c: ContainerPtr) -> JuizResult<Value>;
    
}

pub type ContainerFactoryPtr = Arc<Mutex<dyn ContainerFactory>>;