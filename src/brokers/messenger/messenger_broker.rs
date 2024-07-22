use std::{sync::{atomic::AtomicBool, mpsc, Arc, Mutex}, time::Duration};

use tokio::runtime;

use crate::{object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore}, value::{Capsule, CapsuleMap}, utils::juiz_lock, CapsulePtr, CoreBroker, JuizError, JuizObject, JuizResult, Value};
use crate::brokers::Broker;
use std::sync::atomic::Ordering::SeqCst;

use super::super::crud_broker::CRUDBroker;

#[allow(dead_code)]
pub struct MessengerBroker {
    core: ObjectCore, 
    thread_handle: Option<tokio::task::JoinHandle<()>>,
    //core_broker: Arc<Mutex<CoreBroker>>,
    end_flag: Arc<Mutex<AtomicBool>>,
    crud_broker: Arc<Mutex<CRUDBroker>>, 
    messenger: Arc<Mutex<dyn MessengerBrokerCore>>,
    tokio_runtime: Option<runtime::Runtime>,
}

pub type SenderType = dyn Fn(CapsuleMap) -> JuizResult<()>;
pub type ReceiverType = dyn Fn(Duration) -> JuizResult<Capsule>;

pub struct SendReceivePair(pub Box<SenderType>, pub Box<ReceiverType>);

pub trait MessengerBrokerCore : Send {
    fn receive_and_send(&self, timeout: Duration, func: Arc<Mutex<dyn Fn(CapsuleMap)->JuizResult<CapsulePtr >>>) -> JuizResult<Capsule>;
}

pub trait MessengerBrokerCoreFactory {
    fn create(&self, manifest: &Value) -> JuizResult<Arc<Mutex<dyn MessengerBrokerCore>>>;
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
                //tokio_runtime: Some(runtime::Builder::new_multi_thread().enable_all().build().unwrap()),
                tokio_runtime: Some(tokio::runtime::Builder::new_multi_thread().thread_name("messenger_broker").worker_threads(4).enable_all().build().unwrap()), 
           })))
    }
}

/*
fn to_param(map: &Map<String, Value>) -> JuizResult<HashMap<String, String>> {
    log::trace!("to_param called");
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
*/

fn extract_method_name(args: & CapsuleMap) -> JuizResult<&String> {
    let err = |name: &str | anyhow::Error::from(JuizError::CapsuleDoesNotIncludeParamError{ name: name.to_owned() });
    Ok(args.get_param("method_name").ok_or_else( || err("method_name") )?)
}
/*
fn extract_function_name(args: & CapsuleMap) -> JuizResult<String> {
    let err = |name: &str | anyhow::Error::from(JuizError::CapsuleDoesNotIncludeParamError{ name: name.to_owned() });
    Ok(args.get_param("function_name").ok_or_else( || err("function_name") )?.clone())
}

fn extract_create_parameter(args: CapsuleMap) -> Value {
    return args.into();
}
*/

fn handle_function(crud_broker: Arc<Mutex<CRUDBroker>>, args: CapsuleMap) -> JuizResult<CapsulePtr> {
    log::info!("MessengerBroker::handle_function() called");
    /*
    let method_name = obj_get_str(&value, "method_name")?;
    let class_name = obj_get_str(&value, "class_name")?;
    let function_name = obj_get_str(&value, "function_name")?;
    let args = obj_get(&value, "arguments")?;
    let params = to_param(obj_get_obj(&value, "params")?)?;
    */
    //  let (method_name, class_name, function_name, params) = extract_method_parameters(&args)?;
    //let function_name = extract_function_name(&args)?;
    match extract_method_name(&args)?.as_str() {
        "CREATE" => juiz_lock(&crud_broker)?.create_class(args),
        "READ" =>  juiz_lock(&crud_broker)?.read_class(args),
        "UPDATE" =>  juiz_lock(&crud_broker)?.update_class(args),
        _ => {
            Err(anyhow::Error::from(JuizError::CRUDBRokerCanNotFindMethodError{method_name: "".to_owned()}))
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
//        let join_handle = tokio::task::spawn(async move

        self.thread_handle = Some(self.tokio_runtime.as_mut().unwrap().spawn(
        async move {
                let timeout = Duration::new(0, 100*1000*1000);

                loop {
                    let crud = cb.clone();
                    let func: Arc<Mutex<dyn Fn(CapsuleMap)->JuizResult<CapsulePtr>>> = 
                        Arc::new(Mutex::new(move |value:CapsuleMap| -> JuizResult<CapsulePtr> {
                            handle_function(Arc::clone(&crud), value) 
                        }));
                        
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
                    //sender_receiver.get_mut()
                    //match juiz_lock(&sender_receiver) {
                    match sender_receiver.lock() {
                        Err(_) => {},
                        Ok(sndr_recvr) => {
                            // log::trace!("In MessengerBroker::routine(), calling sndr_recvr.receive_and_send() funciton.");
                            match sndr_recvr.receive_and_send(timeout, func) {
                                    Err(e) => {
                                        log::error!("Error. Core.receive_and_send failed. in MessengerBroker::routine(). Error is {}", e);
                                        //log::trace!("In MessengerBroker::routine(), sndr_recvr.receive_and_send() exit.");
                                    }, Ok(_) => {
                                        //log::trace!("In MessengerBroker::routine(), sndr_recvr.receive_and_send() exit.");
                                    }
                            }
                        }
                    }
                }
                log::debug!("LocalBroker::routine() end!!!");
            }
        ));
//        self.thread_handle = Some(join_handle);
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