use std::sync::Arc;

use crate::{containers::{container_lock, container_lock_mut}, jvalue, object::{JuizObjectClass, JuizObjectCoreHolder, ObjectCore}, value::{Capsule, CapsuleMap}, processes::process_impl::ProcessImpl, utils::check_process_manifest, value::{obj_get_str, obj_merge}, CapsulePtr, ContainerPtr, Identifier, JuizError, JuizObject, JuizResult, Process, ProcessPtr, Value};

use super::container_impl::ContainerImpl;
//use crate::containers::container_process_impl::JuizObjectClass::ContainerProcess;



//pub type ContainerProcessFunction<T>=dyn Fn (&mut Box<T>, Value) -> JuizResult<Value> + 'static;
pub type ContainerFunctionTrait<T>=dyn Fn(&mut ContainerImpl<T>, CapsuleMap) -> JuizResult<Capsule> + 'static;
//pub type ContainerFunctionType<T>=fn (&mut ContainerImpl<T>, CapsuleMap) -> JuizResult<Capsule>;
pub type ContainerFunctionType<T>= Arc<ContainerFunctionTrait<T>>;

#[allow(dead_code)]
pub struct ContainerProcessImpl<T: 'static> {
    core: ObjectCore,
    process: ProcessImpl,
    pub container: ContainerPtr,
    container_identifier: Identifier,
    function: ContainerFunctionType<T>,
}

impl<T: 'static> ContainerProcessImpl<T> {

    pub fn new<'a> (manif: Value, container: ContainerPtr, function: ContainerFunctionType<T>) -> JuizResult<Self> {
        log::trace!("ContainerProcessImpl::new(manifest={}) called", manif);
        //let identifier = create_identifier_from_manifest("ContainerProcess", &manif)?;
        let manifest = check_process_manifest(manif)?;
        let container_clone = Arc::clone(&container);
        let container_identifier = container_lock(&container)?.identifier().clone();
        let f  = function.clone();
        let proc = ProcessImpl::clousure_new_with_class_name(JuizObjectClass::ContainerProcess("ProcessImpl"), manifest.clone(), Box::new(move |args| {
            let mut locked_container = container_lock_mut(&container)?;
            match locked_container.downcast_mut::<ContainerImpl<T>>() {
                None => Err(anyhow::Error::from(JuizError::ContainerDowncastingError{identifier: locked_container.identifier().clone()})),
                Some(container_impl) => {
                    Ok((f)(container_impl, args)?)
                }
            }
            
        }))?;
        
        let type_name = obj_get_str(&manifest, "type_name")?;
        let object_name = obj_get_str(&manifest, "name")?;
        // let f2 = function.clone();
        Ok(  
            (
                move || ContainerProcessImpl::<T>{
                    core: ObjectCore::create(JuizObjectClass::ContainerProcess("ContainerProcessImpl"), 
                        type_name, object_name),
                    container_identifier,
                    container: container_clone,
                    process: proc,
                    function,
                }
            )()
        )
    }

    
}

impl<T: 'static> JuizObjectCoreHolder for ContainerProcessImpl<T> {
    fn core(&self) -> &ObjectCore {
        &self.core
    }
}

impl<T: 'static> JuizObject for ContainerProcessImpl<T> {
    fn profile_full(&self) -> JuizResult<Value> {
        obj_merge(self.process.profile_full()?.try_into()?, &jvalue!({
            "container_identifier": self.container_identifier
        }))
    }
}

impl<T: 'static> Process for ContainerProcessImpl<T> {


    fn manifest(&self) -> &crate::Value {
        self.process.manifest()
    }

    fn call(&self, args: CapsuleMap) -> crate::JuizResult<CapsulePtr> {
        self.process.call(args)
    }

    fn is_updated(& self) -> crate::JuizResult<bool> {
        self.process.is_updated()
    }

    fn is_updated_exclude(& self, caller_id: &str) -> crate::JuizResult<bool> {
        self.process.is_updated_exclude(caller_id)
    }


    fn invoke<'b>(&self) -> crate::JuizResult<CapsulePtr> {
        self.process.invoke()
    }

    fn invoke_exclude<'b>(&self, arg_name: &str, value: CapsulePtr) -> JuizResult<CapsulePtr> {
        self.process.invoke_exclude(arg_name, value)
    }

    fn execute(&self) -> JuizResult<CapsulePtr> {
        self.process.execute()
    }

    fn push_by(&self, arg_name: &str, value: CapsulePtr) -> JuizResult<CapsulePtr> {
        self.process.push_by(arg_name, value)
    }

    fn get_output(&self) -> CapsulePtr {
        self.process.get_output()
    }

    fn notify_connected_from<'b>(&'b mut self, source: ProcessPtr, connecting_arg: &str, connection_manifest: Value) -> JuizResult<Value> {
        self.process.notify_connected_from(source, connecting_arg, connection_manifest)
    }

    fn try_connect_to(&mut self, target: ProcessPtr, connect_arg_to: &str, connection_manifest: Value) -> JuizResult<Value> {
        self.process.try_connect_to(target, connect_arg_to, connection_manifest)
    }

    fn source_connections(&self) -> JuizResult<Vec<&Box<dyn crate::connections::SourceConnection>>> {
        self.process.source_connections()
    }

    fn destination_connections(&self) -> JuizResult<Vec<&Box<dyn crate::connections::DestinationConnection>>> {
        self.process.destination_connections()
    }
    
    fn bind(&mut self, arg_name: &str, value: CapsulePtr) -> JuizResult<CapsulePtr> {
        self.process.bind(arg_name, value)
    }
}




unsafe impl<T: 'static> Send for ContainerProcessImpl<T> {
}

unsafe impl<T: 'static> Sync for ContainerProcessImpl<T> {
}