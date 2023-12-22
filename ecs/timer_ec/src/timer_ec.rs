use std::{sync::{Arc, Mutex, RwLock}, time::Duration};

use juiz_core::{jvalue, ecs::{ExecutionContext, ExecutionContextCore, ExecutionContextFactory}, JuizResult, value::{obj_get_str, obj_get_f64}, Value, JuizError};

pub struct TimerEC {
    //thread_handle: Option<tokio::task::JoinHandle<()>>,
    //end_flag: Arc<Mutex<AtomicBool>>,
    rate: f64,
    name: String,
    timeout: Duration, 
}

impl TimerEC {
    pub fn new(name: &str, rate: f64) -> Arc<RwLock<TimerEC>> {
        let rate_sec: u64 = rate.floor() as u64;
        let rate_nsec: u32 = ((rate - rate.floor()) * (1000_000_000.0)) as u32;
        let timeout = Duration::new(rate_sec, rate_nsec);
        Arc::new( RwLock::new(TimerEC{
            //thread_handle: None,
            //end_flag: Arc::new(Mutex::new(AtomicBool::from(false))),
            rate,
            name: name.to_string(),
            timeout,
        }))
    }
}

impl ExecutionContext for TimerEC {

    /* fn on_starting_(&mut self, svc: Arc<Mutex<ExecutionContextCore>>) -> JuizResult<()> {
        let rate_sec: u64 = self.rate.floor() as u64;
        let rate_nsec: u32 = ((self.rate - self.rate.floor()) * (1000_000_000.0)) as u32;
        let timeout = Duration::new(rate_sec, rate_nsec);

        juiz_lock(&self.end_flag)?.swap(false, SeqCst);
        let end_flag = Arc::clone(&self.end_flag);
        log::trace!("TimerEC::start() called");
        //let core = self.core.clone();
        let join_handle = tokio::task::spawn(async move {
            loop {
                std::thread::sleep(timeout);
                match end_flag.lock() {
                    Err(e) => {
                        log::error!("Error({e:?}) in LocalBroker::routine()");
                        continue
                    },
                    Ok(f) => {
                        match f.load(SeqCst) {
                            true => {
                                log::debug!("Detect end_flag is raised in TimerEC::routine()");
                                break;
                            }
                            false => (),
                        }
                    }
                };
                
                match svc.lock() {
                    Err(e) => {log::error!("Error({e:?}) in Locking ECServiceFunction")},
                    Ok(svc_func) => { let _ = svc_func.svc().map_err(|e| -> () {log::error!("Error({e:?}) in Service function in ExecutionContext."); }); }
                }
                
            }
            log::debug!("TimerEC::routine() end!!!");
        });
        self.thread_handle = Some(join_handle);
        Ok(())

    }

    fn on_stopping(&mut self, _core: Arc<Mutex<ExecutionContextCore>>) -> JuizResult<()> {
        log::debug!("TimerEC::on_stopping() called");
        juiz_lock(&self.end_flag)?.swap(true, SeqCst);
        let _ = futures::executor::block_on(self.thread_handle.take().unwrap())?;
        log::debug!("TimerEC stopped.");
        Ok(())
    }*/

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn type_name(&self) -> &str {
        "TimerEC"
    }

    fn profile(&self) -> JuizResult<Value> {
        Ok(jvalue!({
            "rate": self.rate,
        }))
    }

    fn execute(&self, core: &Arc<Mutex<ExecutionContextCore>>) -> JuizResult<bool> {
        log::trace!("TimerExecutionContext.execute called");
        std::thread::sleep(self.timeout);
        match core.lock() {
            Err(e) => {
                log::error!("Error({e:?}) in Locking ECServiceFunction");
                return Err(anyhow::Error::from(JuizError::ExecutionContextCanNotLockStateError{}));
            },
            Ok(svc_func) => { 
                let _ = svc_func.svc().map_err(|e| -> () {log::error!("Error({e:?}) in Service function in ExecutionContext."); }); 
            }
        }
        return Ok(true);
    }
}

struct TimerECFactory {

}

impl ExecutionContextFactory for TimerECFactory {
    fn type_name(&self) -> &str {
        "TimerEC"
    }

    fn create(&self, manifest: Value) -> JuizResult<Arc<RwLock<dyn ExecutionContext>>> {
        let name = obj_get_str(&manifest, "name")?;
        let rate = obj_get_f64(&manifest, "rate")?;
        Ok(
            TimerEC::new(name, rate)
        )
    }
}

#[no_mangle]
pub unsafe extern "Rust" fn execution_context_factory() -> JuizResult<Arc<Mutex<dyn ExecutionContextFactory>>> {
    env_logger::init();
    Ok(Arc::new(Mutex::new(TimerECFactory{})))
}
