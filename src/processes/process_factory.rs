use crate::{JuizObject, JuizResult, ProcessPtr, Value};

pub trait ProcessFactory: JuizObject {
    fn create_process(&self, manifest: Value) -> JuizResult<ProcessPtr>;
}
