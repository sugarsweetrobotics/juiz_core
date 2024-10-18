
use std::sync::{Arc, Mutex};

use crate::prelude::*;

/// ContainerProcessを生成するためのFactoryクラスのtrait
/// 
pub trait ContainerProcessFactory : JuizObject {

    /// ContainerProcessを生成
    /// 
    fn create_container_process(&self, container: ContainerPtr, manifest: Value) -> JuizResult<ProcessPtr>;

    fn destroy_container_process(&mut self, p: ProcessPtr) -> JuizResult<Value>;
}

pub type ContainerProcessFactoryPtr = Arc<Mutex<dyn ContainerProcessFactory>>;