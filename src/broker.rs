
use std::sync::{Arc, Mutex};
use crate::{Identifier, Value, Process, JuizResult, Container, ContainerProcess};

pub trait Broker {

    fn is_in_charge_for_process(&self, _id: &Identifier) -> bool {
        false
    }

    fn process(&self, id: &Identifier) -> JuizResult<Arc<Mutex<dyn Process>>>;

    fn call_process(&self, _id: &Identifier, _args: Value) -> JuizResult<Value>;

    fn execute_process(&self, _id: &Identifier) -> JuizResult<Value>;

    fn connect_process_to(&mut self, source_process_id: &Identifier, arg_name: &String, target_process_id: &Identifier, manifest: Value) -> JuizResult<Value>;

    fn create_process(&mut self, manifest: Value) -> JuizResult<Arc<Mutex<dyn Process>>>;

    fn create_container(&mut self, manifest: Value) -> JuizResult<Arc<Mutex<dyn Container>>>;

    fn create_container_process(&mut self, container: Arc<Mutex<dyn Container>>, manifest: Value) -> JuizResult<Arc<Mutex<dyn ContainerProcess>>>;

    fn profile_full(&self) -> JuizResult<Value>;

}