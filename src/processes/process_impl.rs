





use anyhow::Context;
use serde_json::Map;

use crate::identifier::{identifier_from_manifest, create_identifier_from_manifest};
use crate::object::{JuizObjectCoreHolder, ObjectCore, JuizObjectClass};

use crate::processes::proc_lock;
use crate::value::{obj_get_str, obj_get_obj, obj_merge_mut};
use crate::{jvalue, CapsulePtr, Identifier, JuizError, JuizObject, JuizResult, Process, ProcessPtr, Value};

use crate::utils::{check_manifest_before_call, check_process_manifest};
use crate::connections::{SourceConnection, SourceConnectionImpl, DestinationConnection, DestinationConnectionImpl};

use super::capsule::{Capsule, CapsuleMap};
use super::inlet::Inlet;
use super::outlet::Outlet;

pub type FunctionType = fn(CapsuleMap) -> JuizResult<Capsule>;
pub type FunctionTrait = dyn Fn(CapsuleMap) -> JuizResult<Capsule>;

pub struct ProcessImpl {
    core: ObjectCore,
    manifest: Value,
    function: Box<FunctionTrait>,
    identifier: Identifier,
    //output_memo: RefCell<Output>,
    outlet: Outlet,
    inlets: Vec<Inlet>,
}


pub fn argument_manifest(process_manifest: &Value) -> JuizResult<&Map<String, Value>>{
    obj_get_obj(process_manifest, "arguments")
}

impl ProcessImpl {

    pub fn new_with_class(class_name: JuizObjectClass, manif: Value, func: FunctionType) -> JuizResult<Self> {
        log::trace!("ProcessImpl::new(manifest={}) called", manif);
        let manifest = check_process_manifest(manif)?;
        let type_name = obj_get_str(&manifest, "type_name")?;
        let object_name = obj_get_str(&manifest, "name")?;

        Ok(Self{
            core: ObjectCore::create(class_name, 
                type_name,
                object_name,
            ),
            manifest: manifest.clone(), 
            function: Box::new(func), 
            identifier: create_identifier_from_manifest("Process", &manifest)?,
            outlet: Outlet::new(),
            inlets: Self::create_inlets(&manifest)?,
        })
    }

    fn create_inlets(manifest: &Value) -> JuizResult<Vec<Inlet>> {
        Ok(argument_manifest(&manifest)?.iter().map( |(k, v)| {
            Inlet::new(k.to_owned(), v.get("default").unwrap().clone())
        }).collect::<Vec<Inlet>>())
    }

    pub fn inlet(&self, name: &str) -> JuizResult<&Inlet> {
        self.inlets.iter().find(|inlet| { (*inlet).name() == name }).ok_or_else(|| { anyhow::Error::from(JuizError::CanNotFindError { target: format!("Process::Inlet({name})") }) } )
    }
    
    pub fn inlet_mut(&mut self, name: &str) -> JuizResult<&mut Inlet> {
        self.inlets.iter_mut().find(|inlet| { (*inlet).name() == name }).ok_or_else(|| { anyhow::Error::from(JuizError::CanNotFindError { target: format!("Process::Inlet({name})") }) } )
    }

    pub fn new(manif: Value, func: FunctionType) -> JuizResult<Self> {
        Self::new_with_class(JuizObjectClass::Process("ProcessImpl"), manif, func)
    }

    pub(crate) fn clousure_new_with_class_name(class_name: JuizObjectClass, manif: Value, func: Box<FunctionTrait>) -> JuizResult<Self> {
        log::trace!("ProcessImpl::new(manifest={}) called", manif);
        let manifest = check_process_manifest(manif)?;
        Ok(Self{
            core: ObjectCore::create(class_name, obj_get_str(&manifest, "type_name")?, obj_get_str(&manifest, "name")?),
            function: func, 
            identifier: identifier_from_manifest("core", "core", "Process", &manifest)?,
            outlet: Outlet::new(),
            inlets: Self::create_inlets(&manifest)?,
            manifest: manifest, 
        })
    }

    pub(crate) fn _clousure_new(manif: Value, func: Box<FunctionTrait>) -> JuizResult<Self> {
        ProcessImpl::clousure_new_with_class_name(JuizObjectClass::Process("ProcessImpl"), manif, func)
    }
    
    fn collect_values_exclude(&self, arg_name: &String, arg_value: CapsulePtr) -> JuizResult<CapsuleMap>{
        log::trace!("ProcessImpl({:?}).collect_values_exclude({:?}) called.", &self.identifier, arg_name);
        let mut arg_map = CapsuleMap::new();
        arg_map.insert(arg_name.clone(), arg_value);
        
        for inlet in self.inlets.iter() {
            if inlet.name() == arg_name { continue; }
            arg_map.insert(inlet.name().clone(), inlet.collect_value()?);
        }
        Ok(arg_map)
    }

}

impl JuizObjectCoreHolder for ProcessImpl {
    fn core(&self) -> &crate::object::ObjectCore {
        &self.core
    }
}

impl JuizObject for ProcessImpl {


    fn profile_full(&self) -> JuizResult<Value> {
        let mut v = self.core.profile_full()?;
        obj_merge_mut(&mut v, &jvalue!({
            "inlets": self.inlets.iter().map(|inlet| { inlet.profile_full().unwrap() }).collect::<Vec<Value>>(),
            "outlet": self.outlet.profile_full()?,
            "arguments": self.manifest.get("arguments").unwrap(),
        }))?;
        Ok(v.into())
    }

}

impl Process for ProcessImpl {
    
    fn manifest(&self) -> &Value { 
        &self.manifest
    }

    fn call(&self, args: CapsuleMap) -> JuizResult<CapsulePtr> {
        log::trace!("ProcessImpl::call() called");
        check_manifest_before_call(&(self.manifest), &args)?;
        Ok( (self.function)(args)?.into() )
    }

    fn is_updated(&self) -> JuizResult<bool> {
        self.is_updated_exclude(&"".to_string())
    }

    fn is_updated_exclude(&self, arg_name: &String) -> JuizResult<bool> {
        if self.outlet.memo().is_empty()? {
            return Ok(true)
        }
        for inlet in self.inlets.iter() {
            if inlet.name() == arg_name { continue; }
            if inlet.is_updated()? {
                return Ok(true)
            }
        }
        Ok(false)
    }

    fn invoke<'b>(&'b self) -> JuizResult<CapsulePtr> {
        log::trace!("Processimpl({:?})::invoke() called", self.identifier());
        self.invoke_exclude(&"".to_string(), Capsule::from(jvalue!({})).into())
    }


    fn invoke_exclude<'b>(&self, arg_name: &String, value: CapsulePtr) -> JuizResult<CapsulePtr> {
        if !self.is_updated_exclude(arg_name)? {
            if self.outlet.memo().is_empty()? { // .is_empty() {
                return Err(anyhow::Error::from(JuizError::ProcessOutputMemoIsNotInitializedError{id: self.identifier().clone()}));
            }
            return Ok(self.outlet.memo().clone());
        }
        Ok(self.outlet.set_value(self.call(self.collect_values_exclude(arg_name, value)?)?))
    }

    fn execute(&self) -> JuizResult<CapsulePtr> {
        self.outlet.push(self.invoke()?)
    }

    fn push_by(&self, arg_name: &String, value: CapsulePtr) -> JuizResult<CapsulePtr> {
        self.outlet.push(self.invoke_exclude(arg_name, value.clone())?)
    }
    
    fn get_output(&self) -> CapsulePtr {
        self.outlet.memo().clone()
    }

    fn notify_connected_from(&mut self, source: ProcessPtr, connecting_arg: &String, connection_manifest: Value) -> JuizResult<Value> {
        log::trace!("ProcessImpl(id={:?}).notify_connected_from(source=Process()) called", self.identifier());
        let id = self.identifier().clone();
        self.inlet_mut(connecting_arg.as_str())?.insert(
                    Box::new(SourceConnectionImpl::new(id, source, connection_manifest.clone(), connecting_arg.clone())?));
        log::trace!("ProcessImpl(id={:?}).notify_connected_from(source=Process()) exit", self.identifier());
        Ok(connection_manifest.into())
    }

    fn try_connect_to(&mut self, destination: ProcessPtr, arg_name: &String, connection_manifest: Value) -> JuizResult<Value> {
        log::info!("ProcessImpl(id={:?}).try_connect_to(destination=Process()) called", self.identifier());
        let destination_id = proc_lock(&destination).context("ProcessImpl::try_connect_to()")?.identifier().clone();
        self.outlet.insert(
            arg_name.clone(), 
            Box::new(DestinationConnectionImpl::new(
                &self.identifier(), 
                &destination_id,
                destination, 
                connection_manifest.clone(), 
                arg_name.clone())?));
        log::info!("ProcessImpl(id={:?}).try_connect_to(destination=Process()) exit", self.identifier());
        Ok(connection_manifest.into())
    }

    
    fn source_connections(&self) -> JuizResult<Vec<&Box<dyn SourceConnection>>> {
        Ok(self.inlets.iter().map(|inlet| { inlet.source_connections() } ).flatten().collect::<Vec<&Box<dyn SourceConnection>>>())
    }
    

    fn destination_connections(&self) -> JuizResult<Vec<&Box<dyn DestinationConnection>>> {
        self.outlet.destination_connections()
    }
}

impl Drop for ProcessImpl {
    fn drop(&mut self) {
        //self.source_connections.drop();
    }
}

unsafe impl Send for ProcessImpl {

}

unsafe impl Sync for ProcessImpl {

}