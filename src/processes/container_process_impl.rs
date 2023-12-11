use std::sync::{Arc, Mutex};

use crate::{Process, Identifier, Value, JuizResult, Container, utils::{juiz_lock, check_process_manifest, manifest_util::get_hashmap_mut}, JuizError, jvalue, value::obj_get_str};

use super::{container::ContainerProcess, container_impl::ContainerImpl, process_impl::ProcessImpl};



pub type ContainerProcessFunction<T>=dyn Fn (&mut Box<T>, Value) -> JuizResult<Value> + 'static;


fn identifier_from_manifest(manifest: &Value) -> Identifier {
    match obj_get_str(manifest, "identifier") {
        Err(_) => obj_get_str(manifest, "name").unwrap().to_string(),
        Ok(id) => id.to_string()
    }
}


#[allow(dead_code)]
pub struct ContainerProcessImpl<T: 'static> {
    identifier: Identifier,
    process: ProcessImpl,
    pub container: Arc<Mutex<dyn Container>>,
    container_identifier: Identifier,
    function: fn (&mut Box<T>, Value) -> JuizResult<Value>
}

impl<T: 'static> ContainerProcessImpl<T> {

    pub fn new<'a> (manif: Value, container: Arc<Mutex<dyn Container>>, function: fn (&mut Box<T>, Value) -> JuizResult<Value>) -> JuizResult<Self> {
        log::trace!("ContainerProcessImpl::new(manifest={}) called", manif);
        let identifier = identifier_from_manifest(&manif);
        let manifest = check_process_manifest(manif)?;
        let container_clone = Arc::clone(&container);
        let container_identifier = juiz_lock(&container)?.identifier().clone();
        let proc = ProcessImpl::clousure_new(manifest, Box::new(move |args: Value| {
            let mut locked_container = juiz_lock(&container)?;
            match locked_container.downcast_mut::<ContainerImpl<T>>() {
                None => Err(anyhow::Error::from(JuizError::ContainerDowncastingError{identifier: locked_container.identifier().clone()})),
                Some(container_impl) => {
                    Ok((function)(&mut container_impl.t, args)?)
                }
            }
            
        }))?;
        Ok(  
            (
                move || ContainerProcessImpl::<T>{
                    identifier,
                    container_identifier,
                    container: container_clone,
                    process: proc,
                    function,
                }
            )()
        )
        /*
        let proc = ProcessImpl::clousure_new(manifest, Box::new(func))?;
        Ok((move ||
            ContainerProcessImpl::<T>{
                container: container_clone,
                process: proc,
                function
            })())
            */
    }

    
}

impl<T: 'static> Process for ContainerProcessImpl<T> {
    fn identifier(&self) -> &crate::Identifier {
        &self.identifier
        // self.process.identifier()
    }


    fn profile_full(&self) -> JuizResult<Value> {
        let mut prof = self.process.profile_full()?;
        let p_hash = get_hashmap_mut(&mut prof)?;
        p_hash.insert("container_identifier".to_string(), jvalue!(self.container_identifier));
        Ok(prof)
    }

    fn manifest(&self) -> &crate::Value {
        self.process.manifest()
    }

    fn call(&self, args: crate::Value) -> crate::JuizResult<crate::Value> {
        self.process.call(args)
    }

    fn is_updated(& self) -> crate::JuizResult<bool> {
        self.process.is_updated()
    }

    fn is_updated_exclude(& self, caller_id: &crate::Identifier) -> crate::JuizResult<bool> {
        self.process.is_updated_exclude(caller_id)
    }


    fn invoke<'b>(&self) -> crate::JuizResult<crate::Value> {
        self.process.invoke()
    }

    fn invoke_exclude<'b>(&self, arg_name: &String, value: crate::Value) -> crate::JuizResult<crate::Value> {
        self.process.invoke_exclude(arg_name, value)
    }

    fn execute(&self) -> crate::JuizResult<crate::Value> {
        self.process.execute()
    }

    fn push_by(&self, arg_name: &String, value: &crate::Value) -> crate::JuizResult<crate::Value> {
        self.process.push_by(arg_name, value)
    }

    fn get_output(&self) -> Option<crate::Value> {
        self.process.get_output()
    }

    fn connected_from<'b>(&'b mut self, source: Arc<Mutex<dyn Process>>, connecting_arg: &String, connection_manifest: crate::Value) -> crate::JuizResult<crate::Value> {
        self.process.connected_from(source, connecting_arg, connection_manifest)
    }

    fn connection_to(&mut self, target: Arc<Mutex<dyn Process>>, connect_arg_to: &String, connection_manifest: crate::Value) -> crate::JuizResult<crate::Value> {
        self.process.connection_to(target, connect_arg_to, connection_manifest)
    }
}

impl<T: 'static> ContainerProcess for ContainerProcessImpl<T> {
    
}