use crate::{JuizResult, JuizObject};

pub trait Broker : JuizObject {

    fn start(&mut self) -> JuizResult<()>;

    fn stop(&mut self) -> JuizResult<()>;
}