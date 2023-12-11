use std::{sync::{Arc, Mutex, atomic::AtomicBool, mpsc}, thread::Builder, time::Duration, ops::Deref};

use crate::{jvalue, Broker, CoreBroker, JuizResult, JuizError, Value, value::{obj_get_str, obj_get}, utils::juiz_lock, BrokerProxy};

use std::sync::atomic::Ordering::SeqCst;

#[allow(dead_code)]
pub struct LocalBroker {
    thread_builder: Option<Builder>,
    thread_handle: Option<tokio::task::JoinHandle<()>>,
    core_broker: Arc<Mutex<dyn BrokerProxy>>,
    end_flag: Arc<Mutex<AtomicBool>>,
    sender_receiver: Arc<Mutex<SenderReceiverPair>>,
}



pub struct SenderReceiverPair(pub mpsc::Sender<Value>, pub mpsc::Receiver<Value>);

impl LocalBroker {

    pub fn new(core_broker: Arc<Mutex<dyn BrokerProxy>>, sender_receiver: Arc<Mutex<SenderReceiverPair>> ) -> JuizResult<Arc<Mutex<dyn Broker>>>{
        Ok(Arc::new(Mutex::new(
            LocalBroker{
                core_broker: core_broker,
                thread_builder: None,
                thread_handle: None,
                sender_receiver,
                end_flag: Arc::new(Mutex::new(AtomicBool::from(false))),
            })))
    }
}

fn handle_function<F: Fn(Value)->JuizResult<()>>(core_broker: Arc<Mutex<dyn BrokerProxy>>, value: Value, send_function: F) -> JuizResult<()> {
    log::info!("Broker::handle_function() called");
    let function_name = obj_get_str(&value, "function_name")?;

    match function_name {
        "profile_full" => send_function(jvalue!({
                "function_name": function_name,
                "return": juiz_lock(&core_broker)?.profile_full()?
        })),
        "is_in_charge_for_process" => send_function(jvalue!({
            "function_name": function_name,
            "return": juiz_lock(&core_broker)?.is_in_charge_for_process(&obj_get_str(&value, "id")?.to_string())?
        })),
        "call_process" => send_function(jvalue!({
            "function_name": function_name,
            "return": juiz_lock(&core_broker)?.call_process(&obj_get_str(&value, "id")?.to_string(), obj_get(&value, "args")?.clone())?
        })),
        "execute_process" => send_function(jvalue!({
            "function_name": function_name,
            "return": juiz_lock(&core_broker)?.execute_process(&obj_get_str(&value, "id")?.to_string())?
        })),
        "connect_process_to" => send_function(jvalue!({
            "function_name": function_name,
            "return": juiz_lock(&core_broker)?.connect_process_to(&obj_get_str(&value, "source_process_id")?.to_string(),
                &obj_get_str(&value, "arg_name")?.to_string(),
                &obj_get_str(&value, "target_process_id")?.to_string(),
                obj_get(&value, "args")?.clone())?
        })),
        _ => {
            log::error!("Requested Function Name ({function_name:}) Not Supported.");
            send_function(jvalue!({
                "function_name": "RequestFunctionNameNotSupported"
            }))
        }
    }
}

impl Broker for LocalBroker {

    fn type_name(&self) -> &str {
        "local"
    }

    fn start(&mut self) -> JuizResult<()> {
        let end_flag = Arc::clone(&self.end_flag);

        log::trace!("LocalBroker::start() called");
        let sender_receiver = Arc::clone(&self.sender_receiver);
        let core_broker = self.core_broker.clone();
        let join_handle = tokio::task::spawn(async move
         {
                let timeout = Duration::new(0, 100*1000*1000);
                loop {
                    std::thread::sleep(Duration::new(0, 10*1000*1000));
                    println!("LocalBroker::routine() called");
                    match end_flag.lock() {
                        Err(_) => {continue},
                        Ok(f) => {
                            if f.load(SeqCst) {
                                break;
                            }
                        }
                    };
                    
                    match sender_receiver.lock() {
                        Err(_) => {continue},
                        Ok(sendr_recvr) => {
                            let SenderReceiverPair(sendr, recvr) = sendr_recvr.deref();
                            match recvr.recv_timeout(timeout) {
                                Err(_e) => {},
                                Ok(value) => {
                                    match handle_function(Arc::clone(&core_broker), value, |value| {
                                        sendr.send(value).map_err(|e| anyhow::Error::from(JuizError::BrokerSendError{error: e}))
                                    }) {
                                        Err(e) => {},
                                        Ok(()) => {}
                                    }
                                }
                            };
                        }
                    };
                }
                println!("LocalBroker::routine() end!!!");
            }
        );
        self.thread_handle = Some(join_handle);
        Ok(())
    }

    fn stop(&mut self) -> JuizResult<()> {
        log::debug!("LocalBroker::stop() called");
        match self.end_flag.lock() {
            Err(_) => {

            },
            Ok(f) => {
                f.swap(true, SeqCst);
            }
        };
        let _ = futures::executor::block_on(self.thread_handle.take().unwrap())?;
        log::debug!("LocalBroker stopped.");
        Ok(())
    }
}