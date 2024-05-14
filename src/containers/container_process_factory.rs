
use crate::{ContainerPtr, JuizObject, JuizResult, ProcessPtr, Value};


pub trait ContainerProcessFactory : JuizObject {
    fn create_container_process(&self, container: ContainerPtr, manifest: Value) -> JuizResult<ProcessPtr>;
}