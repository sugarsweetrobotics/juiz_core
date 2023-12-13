use std::{sync::{Arc, Mutex, atomic::AtomicBool, mpsc, MutexGuard}, thread::Builder, time::Duration, ops::Deref, collections::HashMap};
use serde_json::Map;

use crate::{jvalue, Broker, JuizResult, JuizError, Value, value::{obj_get_str, obj_get, obj_merge, obj_get_obj, obj_get_hashmap}, utils::juiz_lock, BrokerProxy, JuizObject, object::{ObjectCore, JuizObjectClass, JuizObjectCoreHolder}, CoreBroker, Process};

use std::sync::atomic::Ordering::SeqCst;

use super::crud_broker::CRUDBroker;

#[allow(dead_code)]
pub struct MessengerBroker {
    core: ObjectCore, 
    thread_handle: Option<tokio::task::JoinHandle<()>>,
    //core_broker: Arc<Mutex<CoreBroker>>,
    end_flag: Arc<Mutex<AtomicBool>>,
    crud_broker: Arc<Mutex<CRUDBroker>>, 
    messenger: Arc<Mutex<dyn MessengerBrokerCore>>,
}

pub type SenderType = dyn Fn(Value) -> JuizResult<()>;
pub type ReceiverType = dyn Fn(Duration) -> JuizResult<Value>;

pub struct SendReceivePair(pub Box<SenderType>, pub Box<ReceiverType>);

pub trait MessengerBrokerCore : Send {
    fn receive_and_send(&self, timeout: Duration, func: Arc<Mutex<dyn Fn(Value)->JuizResult<Value>>>) -> JuizResult<Value>;
}

pub trait MessengerBrokerCoreFactory {
    fn create(&self) -> JuizResult<Arc<Mutex<dyn MessengerBrokerCore>>>;
}

pub struct SenderReceiverPair(pub mpsc::Sender<Value>, pub mpsc::Receiver<Value>);

impl MessengerBroker {

    pub fn new<'a>(impl_class_name: &'static str, type_name: &'a str, object_name: &'a str, core_broker: Arc<Mutex<CoreBroker>>, messenger: Arc<Mutex<dyn MessengerBrokerCore>> ) -> JuizResult<Arc<Mutex<dyn Broker>>>{
        Ok(Arc::new(Mutex::new(
            MessengerBroker{
                core: ObjectCore::create(JuizObjectClass::Broker(impl_class_name), type_name, object_name),
                thread_handle: None,
                messenger,
                crud_broker: CRUDBroker::new(core_broker.clone())?,
                end_flag: Arc::new(Mutex::new(AtomicBool::from(false))),
            })))
    }
}

fn to_param(map: &Map<String, Value>) -> JuizResult<HashMap<String, String>> {
    let mut ret_map: HashMap<String, String> = HashMap::with_capacity(map.len());
    for (k, v) in map.iter() {
        match v.as_str() {
            None => return Err(anyhow::Error::from(JuizError::CRUDBrokerParameterIsInvalidTypeError{})),
            Some(str_v) => {
                ret_map.insert(k.clone(), str_v.to_string());
            }
        };
    }
    Ok(ret_map)
}

fn handle_function(crud_broker: Arc<Mutex<CRUDBroker>>, value: Value) -> JuizResult<Value> {
    log::info!("Broker::handle_function() called");
    let method_name = obj_get_str(&value, "method_name")?;
    let class_name = obj_get_str(&value, "class_name")?;
    let function_name = obj_get_str(&value, "function_name")?;
    let args = obj_get(&value, "arguments")?;
    let params = to_param(obj_get_obj(&value, "params")?)?;
    //let cb = juiz_lock(&core_broker)?;
    /*
    fn proc(core_broker: &Arc<Mutex<CoreBroker>>, value: &Value) -> JuizResult<Arc<Mutex<dyn Process>>> {
        juiz_lock(&core_broker)?.store().process(&obj_get_str(value, "id")?.to_string())
    }
    */
    
    match method_name {
        //"CREATE" => juiz_lock(&crud_broker)?.create_class(class_name, function_name, args, params),
        "READ" =>  juiz_lock(&crud_broker)?.read_class(class_name, function_name, params),
        "UPDATE" =>  juiz_lock(&crud_broker)?.update_class(class_name, function_name, args.clone(), params),
        _ => {
            Err(anyhow::Error::from(JuizError::CRUDBRokerCanNotFindMethodError{method_name: method_name.to_string()}))
        }
    }
}

impl JuizObjectCoreHolder for MessengerBroker {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}

impl JuizObject for MessengerBroker {
}

impl Broker for MessengerBroker {

    fn start(&mut self) -> JuizResult<()> {
        let end_flag = Arc::clone(&self.end_flag);
        log::trace!("LocalBroker::start() called");
        let sender_receiver = self.messenger.clone();

        let cb = self.crud_broker.clone();
        let join_handle = tokio::task::spawn(async move
         {
                let timeout = Duration::new(0, 100*1000*1000);

                loop {
                    let crud = cb.clone();
                    let func: Arc<Mutex<dyn Fn(Value)->JuizResult<Value>>> = Arc::new(Mutex::new(move |value: Value| -> JuizResult<Value> { handle_function(Arc::clone(&crud), value) }));
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
                    match juiz_lock(&sender_receiver) {
                        Err(_) => {},
                        Ok(sndr_recvr) => {
                            
                            let response = sndr_recvr.receive_and_send(
                                timeout, func);
                        }
                    }
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