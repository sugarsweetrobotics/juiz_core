use crate::JuizResult;

pub trait Broker {

    fn type_name(&self) -> &str;

    fn start(&mut self) -> JuizResult<()>;

    fn stop(&mut self) -> JuizResult<()>;
}