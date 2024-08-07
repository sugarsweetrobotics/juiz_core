
use std::sync::{Arc, Mutex};

use crate::prelude::*;
use crate::JuizObject;

/// ContainerProcessを生成するためのFactoryクラスのtrait
/// 
pub trait ContainerProcessFactory : JuizObject {

    /// ContainerProcessを生成
    /// 
    fn create_container_process(&self, container: ContainerPtr, manifest: Value) -> JuizResult<ContainerProcessPtr>;

    fn destroy_container_process(&mut self, p: ContainerProcessPtr) -> JuizResult<Value>;
}

pub type ContainerProcessFactoryPtr = Arc<Mutex<dyn ContainerProcessFactory>>;