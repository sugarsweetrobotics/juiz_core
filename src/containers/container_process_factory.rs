


use std::sync::{Mutex, Arc};

use crate::{Process, Value, JuizResult, Container, JuizObject};


pub trait ContainerProcessFactory : JuizObject {
    fn create_container_process(&self, container: Arc<Mutex<dyn Container>>, manifest: Value) -> JuizResult<Arc<Mutex<dyn Process>>>;
}