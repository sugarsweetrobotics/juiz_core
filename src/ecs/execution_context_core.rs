

use std::sync::{Mutex, Arc, atomic::AtomicI64};



use crate::{jvalue, JuizResult, Process, utils::juiz_lock, Value, Identifier};


pub enum ExecutionContextState {
    STARTED = 1,
    STOPPED = 2,
    UNKNOWN = 99,
}

impl ExecutionContextState {

    pub fn to_i64(&self) -> i64 {
        match *self {
            ExecutionContextState::STARTED => {1},
            ExecutionContextState::STOPPED => {2},
            ExecutionContextState::UNKNOWN => {99}
        }
    }

    pub fn from(i: i64) -> Self {
        match i {
            1 => ExecutionContextState::STARTED,
            2 => ExecutionContextState::STOPPED,
            _ => ExecutionContextState::UNKNOWN,
        }
    }
}

impl ToString for ExecutionContextState {
    fn to_string(&self) -> String {
        match *self {
            ExecutionContextState::STARTED => {"STARTED".to_owned()},
            ExecutionContextState::STOPPED => {"STOPPED".to_owned()},
            ExecutionContextState::UNKNOWN => {"UNKNOWN".to_owned()}
        }
    }
}

pub struct ExecutionContextCore {
    target_processes: Vec<Arc<Mutex<dyn Process>>>,
    pub state: AtomicI64,
}

impl ExecutionContextCore {

    pub fn new() -> Arc<Mutex<ExecutionContextCore>> {
        Arc::new(Mutex::new(ExecutionContextCore{
            target_processes: Vec::new(),
            state: AtomicI64::new(ExecutionContextState::STOPPED.to_i64()),
        }))
    }

    pub fn bind(&mut self, target_process: Arc<Mutex<dyn Process>>) -> JuizResult<()> {
        self.target_processes.push(target_process);
        Ok(())
    }

    pub fn svc(&self) -> JuizResult<Value> {
        for tp in self.target_processes.iter() {
            let _ = juiz_lock(tp)?.execute()?;
        }
        Ok(jvalue!({}))
    }

    pub fn profile(&self) -> JuizResult<Value> {
        let id_list = self.target_processes.iter().map(|tp| -> Identifier {
            match juiz_lock(tp) {
                Ok(p) => p.identifier().clone(),
                Err(_e) => "Error {e:?}".to_owned()
            }
        });
        Ok(jvalue!({
            "targets": id_list.collect::<Vec<String>>(),
            "state": ExecutionContextState::from(self.state.load(std::sync::atomic::Ordering::SeqCst)).to_string()
        }))
    }
}

