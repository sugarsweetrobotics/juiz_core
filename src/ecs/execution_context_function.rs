use crate::{core::system::System, prelude::*};

use super::execution_context_core::ExecutionContextState;

pub trait ExecutionContextFunction : Send + Sync + JuizObject {

    fn start(&mut self) -> JuizResult<Value>;

    fn stop(&mut self) -> JuizResult<Value>;

    fn get_state(&self) -> JuizResult<ExecutionContextState>;

    fn bind(&mut self, target_process: ProcessPtr) -> JuizResult<()>;
    
    fn unbind(&mut self, target_process_id: Identifier) -> JuizResult<()>;


    fn on_load(&mut self, _system: &mut System) -> () {
        
    }
}