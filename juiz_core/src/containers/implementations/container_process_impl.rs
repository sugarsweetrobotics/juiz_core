use std::sync::Arc;

use anyhow::Context;
use juiz_sdk::anyhow::{self, anyhow};

use crate::connections::ConnectionFactoryImpl;
use crate::prelude::*;
use crate::processes::process_from_clousure;

// type ContainerFunctionTypePtr<T>= Arc<dyn Fn(&mut ContainerImpl<T>, CapsuleMap) -> JuizResult<Capsule>+'static>;

// pub type ContainerProcessPtr=Arc<RwLock<ContainerProcessImpl>>;

#[allow(dead_code)]
pub struct ContainerProcessImpl {
    core: ObjectCore,
    pub process: Box<dyn Process>,
    pub container: Option<ContainerPtr>,
    container_identifier: Identifier,
}


impl ContainerProcessImpl {

    pub fn new<'a, T: 'static> (manifest: ProcessManifest, container: ContainerPtr, function: Arc<dyn Fn(&mut ContainerImpl<T>, CapsuleMap) -> JuizResult<Capsule>+'static>) -> JuizResult<Self> {
        log::trace!("ContainerProcessImpl::new(manifest={:?}) called", manifest);
        //let manifest = check_process_manifest(manif)?;
        let container_clone = container.clone();
        let proc = process_from_clousure(
            Into::<Value>::into(manifest.clone()).try_into()?, 
            move |args| {
                match container_clone.lock_mut()?.downcast_mut::<ContainerImpl<T>>() {
                    Some(c) => {
                        (function)(c, args)
                    }
                    None => {
                        Err(anyhow!(JuizError::ContainerDowncastingError{identifier: "ContainerPtr".to_owned()}))
                    },
                }
            },
            Box::new(ConnectionFactoryImpl::new())
        )?;
        
        Ok(ContainerProcessImpl{
            core: ObjectCore::create(JuizObjectClass::ContainerProcess("ContainerProcessImpl"), 
            manifest.type_name.as_str(), manifest.name.unwrap()),
            container_identifier: container.identifier().clone(),
            container: Some(container),
            process: Box::new(proc),
        })
    }

    fn process(&self) -> JuizResult<&Box<dyn Process>> {
        Ok(&self.process)
    }

    fn process_mut(&mut self) -> JuizResult<&mut Box<dyn Process>> {
        Ok(&mut self.process)
    }
    
}

impl JuizObjectCoreHolder for ContainerProcessImpl {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}

impl JuizObject for ContainerProcessImpl {
    fn profile_full(&self) -> JuizResult<Value> {
        log::trace!("ContainerProcessImpl({})::profile_full() called", self.identifier());
        obj_merge(self.process().context("ContainerProcessImpl()::profile_full()")?.profile_full()?.try_into()?, &jvalue!({
            "container_identifier": self.container_identifier
        }))
    }
}

impl Process for ContainerProcessImpl {

    fn manifest(&self) -> &ProcessManifest {
        log::trace!("ContainerProcessImpl({})::manifest() called", self.identifier());
        self.process().context("ContainerProcessImpl::manifest()").unwrap().manifest()
    }

    fn call(&self, args: CapsuleMap) -> JuizResult<CapsulePtr> {
        log::trace!("ContainerProcessImpl({})::call() called", self.identifier());
        self.process().context("ContainerProcessImpl::call()")?.call(args)
    }

    fn is_updated(& self) -> JuizResult<bool> {
        self.process().context("ContainerProcessImpl::is_updated()")?.is_updated()
    }

    fn invoke<'b>(&self) ->  JuizResult<CapsulePtr> {
        self.process()?.invoke()
    }

    fn execute(&self) -> JuizResult<CapsulePtr> {
        self.process()?.execute()
    }

    fn push_by(&self, arg_name: &str, value: CapsulePtr) -> JuizResult<CapsulePtr> {
        self.process()?.push_by(arg_name, value)
    }

    fn get_output(&self) -> CapsulePtr {
        self.process().unwrap().get_output()
    }

    fn notify_connected_from<'b>(&'b mut self, source: ProcessPtr, connecting_arg: &str, connection_manifest: Value) -> JuizResult<Value> {
        self.process_mut()?.notify_connected_from(source, connecting_arg, connection_manifest)
    }

    fn try_connect_to(&mut self, target: ProcessPtr, connect_arg_to: &str, connection_manifest: Value) -> JuizResult<Value> {
        self.process_mut()?.try_connect_to(target, connect_arg_to, connection_manifest)
    }

    fn source_connections(&self) -> JuizResult<Vec<&Box<dyn SourceConnection>>> {
        self.process()?.source_connections()
    }

    fn destination_connections(&self) -> JuizResult<Vec<&Box<dyn DestinationConnection>>> {
        self.process()?.destination_connections()
    }
    
    fn bind(&mut self, arg_name: &str, value: CapsulePtr) -> JuizResult<CapsulePtr> {
        self.process_mut()?.bind(arg_name, value)
    }
    
    fn purge(&mut self) -> JuizResult<()> {
        log::trace!("ContainerProcessImpl({})::purge() called", self.identifier());
        log::trace!("ContainerProcessImpl({})::purge() exit", self.identifier());
        Ok(())
    }
}

unsafe impl Send for ContainerProcessImpl {
}

unsafe impl Sync for ContainerProcessImpl {
}