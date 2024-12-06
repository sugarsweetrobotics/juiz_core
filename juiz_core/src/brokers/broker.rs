
use std::fmt::Debug;

use crate::prelude::*;

pub trait Broker : JuizObject + 'static {

    fn start(&mut self) -> JuizResult<()>;

    fn stop(&mut self) -> JuizResult<()>;
}