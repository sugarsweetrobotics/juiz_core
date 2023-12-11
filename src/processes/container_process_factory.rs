


use std::sync::{Mutex, Arc};

use crate::{ContainerProcess, Value, JuizResult, Container};


pub trait ContainerProcessFactory {

    fn type_name(&self) -> &str;

    fn create_container_process(&self, container: Arc<Mutex<dyn Container>>, manifest: Value) -> JuizResult<Arc<Mutex<dyn ContainerProcess>>>;

    fn profile_full(&self) -> JuizResult<Value>;
}