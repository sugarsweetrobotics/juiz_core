
use crate::{ContainerPtr, Value, JuizResult, JuizObject};


pub type ContainerConstructFunction<T>=fn(Value) -> JuizResult<Box<T>>;

pub trait ContainerFactory : JuizObject {

    fn create_container(&self, manifest: Value) -> JuizResult<ContainerPtr>;
    
}