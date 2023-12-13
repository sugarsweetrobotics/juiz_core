use std::{sync::{Arc, Mutex, atomic::AtomicBool, mpsc, MutexGuard}, thread::Builder, time::Duration, ops::Deref};
use crate::{jvalue, Broker, JuizResult, JuizError, Value, value::{obj_get_str, obj_get, obj_merge}, utils::juiz_lock, BrokerProxy, JuizObject, object::{ObjectCore, JuizObjectClass, JuizObjectCoreHolder}, CoreBroker, Process};

use std::sync::atomic::Ordering::SeqCst;

#[allow(dead_code)]
pub struct LocalBroker {
    core: ObjectCore, 
    thread_builder: Option<Builder>,
    thread_handle: Option<tokio::task::JoinHandle<()>>,
    core_broker: Arc<Mutex<CoreBroker>>,
    end_flag: Arc<Mutex<AtomicBool>>,
    sender_receiver: Arc<Mutex<SenderReceiverPair>>,
}

pub struct SenderReceiverPair(pub mpsc::Sender<Value>, pub mpsc::Receiver<Value>);

impl LocalBroker {

    pub fn new(core_broker: Arc<Mutex<CoreBroker>>, sender_receiver: Arc<Mutex<SenderReceiverPair>> ) -> JuizResult<Arc<Mutex<dyn Broker>>>{
        let type_name = "local";
        let object_name = "local";
        Ok(Arc::new(Mutex::new(
            LocalBroker{
                core: ObjectCore::create(JuizObjectClass::Broker("LocalBroker"), type_name, object_name),
                core_broker: core_broker,
                thread_builder: None,
                thread_handle: None,
                sender_receiver,
                end_flag: Arc::new(Mutex::new(AtomicBool::from(false))),
            })))
    }
}

fn handle_function<F: Fn(Value)->JuizResult<()>>(core_broker: Arc<Mutex<CoreBroker>>, value: Value, send_function: F) -> JuizResult<()> {
    log::info!("Broker::handle_function() called");
    let class_name = obj_get_str(&value, "class_name")?;
    let function_name = obj_get_str(&value, "function_name")?;
    let args = obj_get(&value, "arguments")?;
    //let cb = juiz_lock(&core_broker)?;
    fn proc(core_broker: &Arc<Mutex<CoreBroker>>, value: &Value) -> JuizResult<Arc<Mutex<dyn Process>>> {
        juiz_lock(&core_broker)?.store().process(&obj_get_str(value, "id")?.to_string())
    }

    match function_name {
        "profile_full" => send_function(jvalue!({
                "function_name": function_name,
                "return": juiz_lock(&core_broker)?.profile_full()?
        })),
        /*
        "is_in_charge_for_process" => send_function(jvalue!({
            "function_name": function_name,
            "return": juiz_lock(&core_broker)?.is_in_charge_for_process(&obj_get_str(&value, "id")?.to_string())?
        })),*/
        "process_profile_full" => send_function(jvalue!({
            "function_name": function_name,
            "return": juiz_lock(&proc(&core_broker, &args)?)?.profile_full()?
        })),
        "process_call" => send_function(jvalue!({
            "function_name": function_name,
            "return": juiz_lock(&proc(&core_broker, &value)?)?.call(args.clone())?
        })),
        "process_execute" => send_function(jvalue!({
            "function_name": function_name,
            "return": juiz_lock(&proc(&core_broker, &value)?)?.execute()?
        })),
        /*
        "process_connect_to" => send_function(jvalue!({
            "function_name": function_name,
            "return": juiz_lock(&core_broker)?.process_connect_to(&obj_get_str(&value, "source_process_id")?.to_string(),
                &obj_get_str(&value, "arg_name")?.to_string(),
                &obj_get_str(&value, "target_process_id")?.to_string(),
                obj_get(&value, "args")?.clone())?
        })),*/
        _ => {
            log::error!("Requested Function Name ({function_name:}) Not Supported.");
            send_function(jvalue!({
                "function_name": "RequestFunctionNameNotSupported"
            }))
        }
    }
}

impl JuizObjectCoreHolder for LocalBroker {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}

impl JuizObject for LocalBroker {

    /* 
    fn profile_full(&self) -> JuizResult<Value> {
        obj_merge(self.core.profile_full()?, &jvalue!({
            "core_broker": juiz_lock(&self.core_broker)?.profile_full()?
        }))
    }
    */
}

impl Broker for LocalBroker {

    fn start(&mut self) -> JuizResult<()> {
        let end_flag = Arc::clone(&self.end_flag);

        log::trace!("LocalBroker::start() called");
        let sender_receiver = Arc::clone(&self.sender_receiver);
        let core_broker = self.core_broker.clone();
        let join_handle = tokio::task::spawn(async move
         {
                let timeout = Duration::new(0, 100*1000*1000);
                loop {
                    //log::trace!("LocalBroker::routine() called");
                    std::thread::sleep(Duration::new(0, 10*1000*1000));
                    match end_flag.lock() {
                        Err(e) => {
                            log::error!("Error({e:?}) in LocalBroker::routine()");
                            continue
                        },
                        Ok(f) => {
                            if f.load(SeqCst) {
                                log::debug!("Detect end_flag is raised in LocalBroker::routine()");
                                break;
                            }
                        }
                    };
                    match sender_receiver.lock() {
                        Err(e) => {
                            log::error!("Error({e:?}) in LocalBroker::routine()");
                            continue;
                        },
                        Ok(sendr_recvr) => {
                            let SenderReceiverPair(sendr, recvr) = sendr_recvr.deref();
                            match recvr.recv_timeout(timeout) {
                                Err(_e) => {
                                    continue;
                                },
                                Ok(value) => {
                                    match handle_function(core_broker.clone(), value, |value| {
                                        sendr.send(value).map_err(|e| anyhow::Error::from(JuizError::BrokerSendError{error: e}))
                                    }) {
                                        Err(e) => {log::error!("Error({e:?}) in LocalBroker::routine()")},
                                        Ok(()) => {}
                                    }
                                }
                            };
                        }
                    };
                }
                log::debug!("LocalBroker::routine() end!!!");
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