use std::{sync::{atomic::AtomicBool, Arc, Mutex, atomic::Ordering::SeqCst}, time::Duration};
use crate::{utils::juiz_lock, Value, value::{obj_get_str, obj_get_f64}, JuizResult};
use crate::ecs::{ExecutionContext, ExecutionContextCore, ExecutionContextFactory};


pub struct TimerEC {
    thread_handle: Option<tokio::task::JoinHandle<()>>,
    end_flag: Arc<Mutex<AtomicBool>>,
    rate: f64,
    name: String
}

impl TimerEC {
    pub fn new(name: &str, rate: f64) -> Arc<Mutex<TimerEC>> {
        Arc::new(Mutex::new(TimerEC{
            thread_handle: None,
            end_flag: Arc::new(Mutex::new(AtomicBool::from(false))),
            rate,
            name: name.to_string(),
        }))
    }
}

impl ExecutionContext for TimerEC {
    fn on_starting(&mut self, svc: Arc<Mutex<ExecutionContextCore>>) -> crate::JuizResult<()> {
        let rate_sec: u64 = self.rate.floor() as u64;
        let rate_nsec: u32 = ((self.rate - self.rate.floor()) * (1000_000_000.0)) as u32;
        let timeout = Duration::new(rate_sec, rate_nsec);

        juiz_lock(&self.end_flag)?.swap(false, SeqCst);
        let end_flag = Arc::clone(&self.end_flag);
        //svc_.
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

    fn on_stopping(&mut self, _core: Arc<Mutex<ExecutionContextCore>>) -> crate::JuizResult<()> {
        log::debug!("TimerEC::on_stopping() called");
        juiz_lock(&self.end_flag)?.swap(true, SeqCst);
        let _ = futures::executor::block_on(self.thread_handle.take().unwrap())?;
        log::debug!("TimerEC stopped.");
        Ok(())
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn type_name(&self) -> &str {
        "TimerEC"
    }

    fn profile(&self) ->  JuizResult<Value> {
        todo!()
    }
}

struct TimerECFactory {

}

impl ExecutionContextFactory for TimerECFactory {
    fn type_name(&self) -> &str {
        "TimerEC"
    }

    fn create(&self, manifest: Value) -> crate::JuizResult<Arc<Mutex<dyn ExecutionContext>>> {
        let name = obj_get_str(&manifest, "name")?;
        let rate = obj_get_f64(&manifest, "rate")?;
        Ok(
            TimerEC::new(name, rate)
        )
    }
}