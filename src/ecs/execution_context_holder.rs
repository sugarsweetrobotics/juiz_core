use std::sync::{Mutex, Arc, RwLock, atomic::AtomicBool};



use tokio::runtime;

use crate::{jvalue, JuizResult, utils::{juiz_lock, sync_util::{juiz_borrow, juiz_borrow_mut}}, object::{ObjectCore, JuizObjectClass, JuizObjectCoreHolder}, JuizObject, Process, Value, value::obj_merge_mut, ecs::execution_context_core::ExecutionContextState};

use super::{execution_context::ExecutionContext, execution_context_core::ExecutionContextCore};

pub struct ExecutionContextHolder{
    object_core: ObjectCore,
    core: Arc<Mutex<ExecutionContextCore>>,
    execution_context: Arc<RwLock<dyn ExecutionContext>>,
    thread_handle: Option<tokio::task::JoinHandle<()>>,
    //tokio_runtime: &'static runtime::Runtime,
    tokio_runtime: runtime::Runtime,
    end_flag: Arc<Mutex<AtomicBool>>,
}

impl ExecutionContextHolder {

    pub fn new(type_name: &str, ec: Arc<RwLock<dyn ExecutionContext>>) -> JuizResult<Arc<Mutex<ExecutionContextHolder>>> {
        Ok(Arc::new(Mutex::new(
            ExecutionContextHolder { 
                object_core: ObjectCore::create(JuizObjectClass::ExecutionContext("ExecutionContext"), 
                    type_name, 
                    juiz_borrow(&ec)?.name()), 
                core: ExecutionContextCore::new(), 
                execution_context: ec.clone(),
                //tokio_runtime: runtime,
                tokio_runtime: runtime::Builder::new_multi_thread().thread_name("execution_context_holder").worker_threads(4).enable_all().build().unwrap(),
                //tokio_runtime: Some(runtime::Builder::new_current_thread().enable_all().build().unwrap()),
                thread_handle: None,
                end_flag: Arc::new(Mutex::new(AtomicBool::from(false))),
             }
        )))
    }

    pub fn start(&mut self) -> JuizResult<Value> {

        let type_name = self.type_name().to_string();
        log::trace!("ExecutionContextHolder::start(type_name={type_name}) called");
        {
            juiz_lock(&self.end_flag)?.swap(false, std::sync::atomic::Ordering::SeqCst);
        }        
        let core = self.core.clone();
        let mut ec = self.execution_context.clone();

        let end_flag = Arc::clone(&self.end_flag);


        //self.thread_handle = Some(self.tokio_runtime.spawn(
        self.thread_handle = Some(self.tokio_runtime.spawn_blocking(
//            async move {
            move || {

                {
                    match juiz_borrow_mut(&mut ec) {
                        Err(e) => {
                            log::error!("ExecutionContextHolder.routine() error: {e:?}");
                            return;
                        }, 
                        Ok(mut e) => {
                            match e.on_starting(&core) {
                                Ok(_) => {},
                                Err(e) => {
                                    log::error!("ExecutionContextHolder.routine() error: {e:?}");
                                    return;
                                }
                            }
                        }
                    }

                    match core.lock() {
                        Err(e) => {
                            log::error!("ExecutionContextHolder.routine() error: {e:?}");
                            return;
                        },
                        Ok(s) => {
                            s.state.store(ExecutionContextState::STARTED.to_i64(), std::sync::atomic::Ordering::SeqCst);
                        }
                    }
                }
                loop {
                    log::trace!("ExecutionContextHolder::routine() loop");
                    match end_flag.lock() {
                        Err(e) => {
                            log::error!("Error({e:?}) in ExecutionContextHodler::routine()");
                            continue
                        },
                        Ok(f) => {
                            match f.load(std::sync::atomic::Ordering::SeqCst) {
                                true => {
                                    log::debug!("Detect end_flag is raised in ExecutionContextHodler::routine()");
                                    break;
                                }
                                false => (),
                            }
                        }
                    };

                    {
                        match juiz_borrow(&ec) {
                            Err(e) => {
                                log::error!("ExecutionContextHolder.routine() error: {e:?}");
                                break;
                            }, 
                            Ok(e) => {
                                match e.execute(&core) {
                                    Err(e) => {
                                        log::error!("ExecutionContext.execute() failed in ExecutionContextHolder.routine() error: {e:?}");
                                        break;
                                    }, Ok(f) => {
                                        if !f {
                                            break;
                                        }
                                    }
                                }
                            }
                        };
                    }
                }
                {
                    match juiz_borrow_mut(&mut ec) {
                        Err(e) => {
                            log::error!("ExecutionContextHolder.routine() error: {e:?}");
                            return;
                        }, 
                        Ok(mut e) => {
                            match e.on_stopping(&core) {
                                Ok(_) => {},
                                Err(e) => {
                                    log::error!("ExecutionContextHolder.routine() error: {e:?}");
                                    return;
                                }
                            }
                        }
                    }
                    match core.lock() {
                        Err(e) => {
                            log::error!("ExecutionContextHolder.routine() error: {e:?}");
                            return;
                        },
                        Ok(c) => {
                            c.state.store(ExecutionContextState::STOPPED.to_i64(), std::sync::atomic::Ordering::SeqCst);
                        }
                    }
                }
            }
        ));
        log::trace!("ExecutionContextHolder::start(type_name={type_name}) exit");
        Ok(jvalue!({}))
    }
        
    pub fn start_(&mut self) -> JuizResult<Value> {

        todo!()
        /*let result = juiz_lock(&self.execution_context)?.on_starting(self.core.clone())?;
        match self.state.lock() {
            Err(e) => return Err(anyhow::Error::from(JuizError::ExecutionContextCanNotLockStateError{})),
            Ok(s) => {
                s.store(ExecutionContextState::STARTED.to_i64(), std::sync::atomic::Ordering::SeqCst);
            }
        }
        return Ok(jvalue!(ExecutionContextState::STARTED.to_string()));*/
    }

    pub fn stop(&mut self) -> JuizResult<Value> {
        log::info!("ExecutionContextHolder::stop() called");
        if self.thread_handle.is_some() {
            juiz_lock(&self.end_flag)?.swap(true, std::sync::atomic::Ordering::SeqCst);
            let _ = futures::executor::block_on(self.thread_handle.take().unwrap())?;
            self.thread_handle = None;
        }
        Ok(jvalue!({}))
    }

    pub fn stop_(&mut self) -> JuizResult<Value> {
        todo!()
        //let result = juiz_lock(&self.execution_context)?.on_stopping(Arc::clone(&self.core))?;
        /*
        match self.state.lock() {
            Err(e) => return Err(anyhow::Error::from(JuizError::ExecutionContextCanNotLockStateError{})),
            Ok(s) => {
                s.store(ExecutionContextState::STOPPED.to_i64(), std::sync::atomic::Ordering::SeqCst);
            }
        }
        return Ok(jvalue!(ExecutionContextState::STOPPED.to_string()));
        */
    }

    pub fn bind(&mut self, target_process: Arc<Mutex<dyn Process>>) -> JuizResult<()> {
        juiz_lock(&self.core)?.bind(target_process)
    }

    pub fn get_state(&self) -> JuizResult<ExecutionContextState> {
        let s = juiz_lock(&self.core)?.state.load(std::sync::atomic::Ordering::SeqCst);
        Ok(ExecutionContextState::from(s))
    }
}

impl JuizObjectCoreHolder for ExecutionContextHolder {
    fn core(&self) -> &ObjectCore {
        &self.object_core
    }
}

impl JuizObject for ExecutionContextHolder {

    fn profile_full(&self) -> JuizResult<Value> {
        log::trace!("ExecutionContextHolder()::profile_full() called");
        let mut v = self.object_core.profile_full()?;
        let ecv = juiz_borrow(&self.execution_context)?.profile()?;
        obj_merge_mut(&mut v, &ecv)?;

        let cv = juiz_lock(&self.core)?.profile()?;
        obj_merge_mut(&mut v, &cv)?;
        Ok(v)
    }
}
