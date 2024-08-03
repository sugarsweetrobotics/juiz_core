use std::sync::Arc;

use crate::{containers::{container_lock, container_lock_mut}, jvalue, object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore}, value::{Capsule, CapsuleMap}, processes::process_impl::ProcessImpl, utils::check_process_manifest, value::{obj_get_str, obj_merge}, CapsulePtr, ContainerPtr, Identifier, JuizError, JuizObject, JuizResult, Process, ProcessPtr, Value};

use super::container_impl::ContainerImpl;
//use crate::containers::container_process_impl::JuizObjectClass::ContainerProcess;



//pub type ContainerProcessFunction<T>=dyn Fn (&mut Box<T>, Value) -> JuizResult<Value> + 'static;
pub type ContainerFunctionTrait<T>=dyn Fn(&mut ContainerImpl<T>, CapsuleMap) -> JuizResult<Capsule> + 'static;
//pub type ContainerFunctionType<T>=fn (&mut ContainerImpl<T>, CapsuleMap) -> JuizResult<Capsule>;
pub type ContainerFunctionType<T>= Arc<ContainerFunctionTrait<T>>;

#[allow(dead_code)]
pub struct ContainerProcessImpl {
    core: ObjectCore,
    process: Option<ProcessImpl>,
    pub container: Option<ContainerPtr>,
    container_identifier: Identifier,
    //function: ContainerFunctionType<T>,
}

impl ContainerProcessImpl {

    pub fn new<'a, T: 'static> (manif: Value, container: ContainerPtr, function: ContainerFunctionType<T>) -> JuizResult<Self> {
        log::trace!("ContainerProcessImpl::new(manifest={}) called", manif);
        //let identifier = create_identifier_from_manifest("ContainerProcess", &manif)?;
        let manifest = check_process_manifest(manif)?;
        let container_clone = Arc::clone(&container);
        let container_identifier = container_lock(&container)?.identifier().clone();
        //let f  = function.clone();
        let proc = ProcessImpl::clousure_new_with_class_name(JuizObjectClass::ContainerProcess("ProcessImpl"), manifest.clone(), Box::new(move |args| {
            let mut locked_container = container_lock_mut(&container)?;
            match locked_container.downcast_mut::<ContainerImpl<T>>() {
                None => Err(anyhow::Error::from(JuizError::ContainerDowncastingError{identifier: locked_container.identifier().clone()})),
                Some(container_impl) => {
                    Ok((function)(container_impl, args)?)
                }
            }
        }))?;
        
        let type_name = obj_get_str(&manifest, "type_name")?;
        let object_name = obj_get_str(&manifest, "name")?;
        // let f2 = function.clone();
        Ok(  
            (
                move || ContainerProcessImpl{
                    core: ObjectCore::create(JuizObjectClass::ContainerProcess("ContainerProcessImpl"), 
                        type_name, object_name),
                    container_identifier,
                    container: Some(container_clone),
                    process: Some(proc),
                }
            )()
        )
    }

    fn process(&self) -> JuizResult<&ProcessImpl> {
        match &self.process {
            Some(p) => Ok(p),
            None => Err(anyhow::Error::from(JuizError::ObjectCanNotFoundByIdError { id: "ContainerProcessImpl::process() failed".to_owned() }))
        }
    }

    fn process_mut(&mut self) -> JuizResult<&mut ProcessImpl> {
        match &mut self.process {
            Some(p) => Ok(p),
            None => Err(anyhow::Error::from(JuizError::ObjectCanNotFoundByIdError { id: "ContainerProcessImpl::process() failed".to_owned() }))
        }
    }
    
}

impl JuizObjectCoreHolder for ContainerProcessImpl {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}

impl JuizObject for ContainerProcessImpl {
    fn profile_full(&self) -> JuizResult<Value> {
        obj_merge(self.process()?.profile_full()?.try_into()?, &jvalue!({
            "container_identifier": self.container_identifier
        }))
    }
}

impl Process for ContainerProcessImpl {

    fn manifest(&self) -> &crate::Value {
        self.process().unwrap().manifest()
    }

    fn call(&self, args: CapsuleMap) -> crate::JuizResult<CapsulePtr> {
        self.process()?.call(args)
    }

    fn is_updated(& self) -> crate::JuizResult<bool> {
        self.process()?.is_updated()
    }

    fn is_updated_exclude(& self, caller_id: &str) -> crate::JuizResult<bool> {
        self.process()?.is_updated_exclude(caller_id)
    }


    fn invoke<'b>(&self) -> crate::JuizResult<CapsulePtr> {
        self.process()?.invoke()
    }

    fn invoke_exclude<'b>(&self, arg_name: &str, value: CapsulePtr) -> JuizResult<CapsulePtr> {
        self.process()?.invoke_exclude(arg_name, value)
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

    fn source_connections(&self) -> JuizResult<Vec<&Box<dyn crate::connections::SourceConnection>>> {
        self.process()?.source_connections()
    }

    fn destination_connections(&self) -> JuizResult<Vec<&Box<dyn crate::connections::DestinationConnection>>> {
        self.process()?.destination_connections()
    }
    
    fn bind(&mut self, arg_name: &str, value: CapsulePtr) -> JuizResult<CapsulePtr> {
        self.process_mut()?.bind(arg_name, value)
    }
    
    fn purge(&mut self) -> JuizResult<()> {
        log::trace!("ContainerProcessImpl({})::purge() called", self.identifier());
        self.container.take();
        self.process.take();
        log::trace!("ContainerProcessImpl({})::purge() exit", self.identifier());
        Ok(())
    }
}




unsafe impl Send for ContainerProcessImpl {
}

unsafe impl Sync for ContainerProcessImpl {
}

impl Drop for ContainerProcessImpl{
    fn drop(&mut self) {
        log::trace!("ContainerProcessImpl({})::drop() called", self.identifier());
        self.container.take();
        log::trace!("ContainerProcessImpl({})::drop()e exit", self.identifier());
    }
}