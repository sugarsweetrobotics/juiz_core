use std::sync::{Mutex, Arc};



use crate::{JuizResult, Value};

use super::execution_context_core::ExecutionContextCore;

pub trait ECServiceFunction : Fn()->JuizResult<()> + Send + Sync {

}

pub trait ExecutionContext : Send + Sync {

    fn name(&self) -> &str;

    fn type_name(&self) -> &str;

    fn on_starting(&mut self, _core: &Arc<Mutex<ExecutionContextCore>>) -> JuizResult<()> {
        Ok(())
    }

    fn on_stopping(&mut self, _core: &Arc<Mutex<ExecutionContextCore>>) -> JuizResult<()> {
        Ok(())
    }

    fn profile(&self) -> JuizResult<Value>;

    /// 周期的に呼ばれる関数。自身をSTOPしたいならfalseを返すこと。
    fn execute(&self, core: &Arc<Mutex<ExecutionContextCore>>) -> JuizResult<bool>;
}
