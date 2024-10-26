

use std::sync::{Arc, Mutex};
use futures::Future;

use crate::core::CoreBrokerPtr;
use crate::prelude::*;
use crate::brokers::{Broker, CRUDBroker};

use tokio::runtime;

pub struct CRUDBrokerHolder<F, Fut> where F: Fn(Value, Arc<Mutex<CRUDBroker>>) -> Fut + Send + Sync + Copy + 'static, Fut: Future<Output=()>+ Send + 'static {
    core: ObjectCore,
    crud_broker: Arc<Mutex<CRUDBroker>>,
    thread_handle: Option<tokio::task::JoinHandle<()>>,
    tokio_runtime: runtime::Runtime,
    on_start_function: F,
    manifest: Value,
}

impl<F, Fut> CRUDBrokerHolder<F, Fut> where F: Fn(Value, Arc<Mutex<CRUDBroker>>) -> Fut + Send + Sync + Copy + 'static, Fut: Future<Output=()>+ Send + 'static {
    pub fn new(impl_class_name: &'static str, type_name: &'static str, core_broker: CoreBrokerPtr, on_start_function: F, manifest: Value) -> JuizResult<Self> {
        let object_name = obj_get_str(&manifest, "name")?;
        Ok(CRUDBrokerHolder{
            core: ObjectCore::create(JuizObjectClass::Broker(impl_class_name), type_name, object_name), 
            crud_broker: Arc::new(Mutex::new(CRUDBroker::new(core_broker)?)),
            thread_handle: None,
            tokio_runtime: runtime::Builder::new_multi_thread().enable_all().build().unwrap(),
            on_start_function,
            manifest,
        })
    }
}

impl<F, Fut> JuizObjectCoreHolder for CRUDBrokerHolder<F, Fut>  where F: Fn(Value, Arc<Mutex<CRUDBroker>>) -> Fut + Send + Sync + Copy + 'static, Fut: Future<Output=()>+ Send + 'static {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}
impl<F, Fut> JuizObject for CRUDBrokerHolder<F, Fut>  where F: Fn(Value, Arc<Mutex<CRUDBroker>>) -> Fut + Send + Sync + Copy + 'static, Fut: Future<Output=()>+ Send + 'static {}


impl<F, Fut> Broker for CRUDBrokerHolder<F, Fut>  where F: Fn(Value, Arc<Mutex<CRUDBroker>>) -> Fut + Send + Sync + Copy + 'static, Fut: Future<Output=()>+ Send + 'static {

    fn start(&mut self) -> JuizResult<()> {
        let type_name = self.type_name().to_string();
        log::trace!("CRUDBrokerHolder::start(type_name={type_name}) called");
        
        let crud = self.crud_broker.clone();
        let manifest = self.manifest.clone();
        let on_start = self.on_start_function;//.take().unwrap();
        self.thread_handle = Some(self.tokio_runtime.spawn(
            async move  {
                on_start(manifest, crud).await;
            }
        ));
        log::trace!("CRUDBrokerHolder::start(type_name={type_name}) exit");
        Ok(())
    }

    fn stop(&mut self) -> JuizResult<()> {
        let type_name = self.type_name().to_string();
        log::trace!("CRUDBrokerHolder::stop(type_name={type_name}) called");
        self.thread_handle.take().unwrap().abort();
        log::trace!("CRUDBrokerHolder::stop(type_name={type_name}) exit");
        Ok(())
    }
}
