use std::{sync::{Arc, Mutex, atomic::AtomicBool}, thread::Builder, time::Duration};

use crate::{Broker, CoreBroker, JuizResult, JuizError};

use std::sync::atomic::Ordering::SeqCst;


#[allow(dead_code)]
pub struct LocalBroker {
    core_broker: Arc<Mutex<CoreBroker>>,
    thread_builder: Option<Builder>,
    thread_handle: Option<std::thread::JoinHandle<JuizResult<()>>>,
    end_flag: Option<Arc<AtomicBool>>
}

impl LocalBroker {

    pub fn new(core_broker: Arc<Mutex<CoreBroker>>) -> JuizResult<Arc<Mutex<dyn Broker>>>{
        Ok(Arc::new(Mutex::new(
            LocalBroker{
                thread_builder: None,
                core_broker,
                thread_handle: None,
                end_flag: None
            })))
    }
}

impl Broker for LocalBroker {

    fn type_name(&self) -> &str {
        "local"
    }

    fn start(&mut self) -> JuizResult<()> {
        log::trace!("LocalBroker::start() called");;
        let thread_builder = Builder::new().name("local_broker".to_string());
        let end_flag = Arc::new(AtomicBool::from(false));
        self.end_flag = Some(Arc::clone(&end_flag));
        let join_handle = thread_builder.spawn(move || -> JuizResult<()> {
            loop {
                std::thread::sleep(Duration::new(1, 0));
                println!("LocalBroker::routine() called");
                if end_flag.load(SeqCst) {
                    break;
                }
            }
            println!("LocalBroker::routine() end!!!");
            Ok(())
        }).unwrap_or_else(|e| panic!("{:?}", e));
        self.thread_handle = Some(join_handle);
        Ok(())
    }

    fn stop(&mut self) -> JuizResult<()> {
        log::debug!("LocalBroker::stop() called");
        self.end_flag.as_ref().unwrap().swap(true, SeqCst);
        match self.thread_handle.take().unwrap().join() {
            Err(_e) => return Err(anyhow::Error::from(JuizError::BrokerStopFailedError{type_name: "local".to_string()})),
            Ok(_) => {}
        };
        log::debug!("LocalBroker stopped.");
        Ok(())
    }
}