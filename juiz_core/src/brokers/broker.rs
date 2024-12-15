
use std::{fmt::Debug, time::Duration};

use crate::prelude::*;

pub trait Broker : JuizObject + 'static {

    fn start(&mut self) -> JuizResult<()>;

    fn wait_until_started(&mut self, timeout: Duration) -> JuizResult<()>;

    fn stop(&mut self) -> JuizResult<()>;
}