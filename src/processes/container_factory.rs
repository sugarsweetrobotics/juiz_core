use std::sync::{Mutex, Arc};

use crate::{Container, Value, JuizResult};


pub type ContainerConstructFunction<T>=fn(Value) -> JuizResult<Box<T>>;

pub trait ContainerFactory {

    fn type_name(&self) -> &str;

    //fn create_process(&self, manifest: Value) -> JuizResult<Arc<Mutex<dyn Container>>>;

    fn create_container(&self, manifest: Value) -> JuizResult<Arc<Mutex<dyn Container>>>;
    
    fn profile_full(&self) -> JuizResult<Value>;
}