


use std::sync::{Mutex, Arc};
use anyhow::Context;
use serde_json::Map;

use crate::identifier::{identifier_from_manifest, create_identifier_from_manifest};
use crate::object::{JuizObjectCoreHolder, ObjectCore, JuizObjectClass};

use crate::value::{obj_get_str, obj_get_obj, obj_merge_mut};
use crate::{jvalue, Identifier, JuizError, JuizObject, JuizResult, Process, Value};

use crate::utils::{check_manifest_before_call, check_process_manifest, juiz_lock};
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
            // source_connections: HashMap::new(),
            //destination_connections: HashMap::new(),
            //output_memo: RefCell::new(Output::new_from_value(jvalue!(null))),
            outlet: Outlet::new(),
            inlets: Self::create_inlets(&manifest)?,
        })
    }

    fn create_inlets(manifest: &Value) -> JuizResult<Vec<Inlet>> {
        let mut vec_inlet: Vec<Inlet> = Vec::new();
        for (k, v) in argument_manifest(&manifest)?.into_iter() {
            vec_inlet.push(Inlet::new(k.to_owned(), v.get("default").unwrap().clone()))
        }
        Ok(vec_inlet)
    }

    pub fn inlet(&self, name: &str) -> Option<&Inlet> {
        for inlet in self.inlets.iter() {
            if inlet.name() == name {
                return Some(inlet)
            }
        }
        None
    }
    
    pub fn inlet_mut(&mut self, name: &str) -> Option<&mut Inlet> {
        for inlet in self.inlets.iter_mut() {
            if inlet.name() == name {
                return Some(inlet)
            }
        }
        None
    }

    pub fn new(manif: Value, func: FunctionType) -> JuizResult<Self> {
        Self::new_with_class(JuizObjectClass::Process("ProcessImpl"), manif, func)
    }

    pub(crate) fn clousure_new_with_class_name(class_name: JuizObjectClass, manif: Value, func: Box<FunctionTrait>) -> JuizResult<Self> {
        log::trace!("ProcessImpl::new(manifest={}) called", manif);
        
        let manifest = check_process_manifest(manif)?;
        let type_name = obj_get_str(&manifest, "type_name")?;
        let object_name = obj_get_str(&manifest, "name")?;
        Ok(Self{
            core: ObjectCore::create(class_name,
            type_name, object_name),
            manifest: manifest.clone(), 
            function: func, 
            identifier: identifier_from_manifest("core", "core", "Process", &manifest)?,
            // source_connections: HashMap::new(),
            // destination_connections: HashMap::new(),
            // output_memo: RefCell::new(Output::new_from_value(jvalue!(null))),
            outlet: Outlet::new(),
            inlets: Self::create_inlets(&manifest)?,
        })
    }

    pub(crate) fn _clousure_new(manif: Value, func: Box<FunctionTrait>) -> JuizResult<Self> {
        ProcessImpl::clousure_new_with_class_name(JuizObjectClass::Process("ProcessImpl"), manif, func)
    }
    
    fn collect_values_exclude(&self, arg_name: &String, arg_value: Capsule) -> JuizResult<CapsuleMap>{
        log::trace!("ProcessImpl({:?}).collect_values_exclude({:?}) called.", &self.identifier, arg_name);
        let mut arg_map = CapsuleMap::new();
        arg_map.insert(arg_name.clone(), arg_value);
        
        for inlet in self.inlets.iter() {
            if inlet.name() == arg_name { continue; }
            arg_map.insert(inlet.name().clone(), inlet.collect_value()?);
        }
        Ok(arg_map)
    }

    /*
    fn to_arguments(&self, value: Value) -> JuizResult<Vec<Argument>> {
        let mut args: Vec<Argument> = Vec::new();
        let arg_map = get_hashmap(&value).context("ProcessImpl.to_arguments()")?;
        for (arg_name, _v) in argument_manifest(&self.manifest).context("check_arguments")? {
            match arg_map.get(arg_name) {
                None => return Err(anyhow::Error::from(JuizError::ArgumentMissingWhenCallingError{process_manifest: self.manifest.clone(), given_argument: value.clone(), missing_arg_name: arg_name.clone()})),
                Some(a) => {
                    args.push(Argument::new(arg_name, a.to_owned()));
                }
            };
        }
        Ok(args)
    }
    */

}

impl JuizObjectCoreHolder for ProcessImpl {
    fn core(&self) -> &crate::object::ObjectCore {
        &self.core
    }
}

impl JuizObject for ProcessImpl {


    fn profile_full(&self) -> JuizResult<Capsule> {
        /*
        let mut dc_profs = jvalue!({});
        let dc_map = get_hashmap_mut(&mut dc_profs)?;
        for (key, value) in self.destination_connections.iter() {
            dc_map.insert(key.to_owned(), value.profile_full()?);
        }
        **/

        let mut v = self.core.profile_full()?;
        obj_merge_mut(&mut v, &jvalue!({
            "inlets": self.inlets.iter().map(|inlet| { inlet.profile_full().unwrap().try_into().unwrap() }).collect::<Vec<Value>>(),
            "outlet": self.outlet.profile_full()?.as_value().ok_or(anyhow::Error::from(JuizError::CapsuleIsNotValueTypeError {  }))?,
//            "destination_connections": dc_profs,
            "arguments": self.manifest.get("arguments").unwrap(),
        }))?;
        Ok(v.into())
    }

}

impl Process for ProcessImpl {
    
    fn manifest(&self) -> &Value { 
        &self.manifest
    }

    fn call(&self, args: CapsuleMap) -> JuizResult<Capsule> {
        check_manifest_before_call(&(self.manifest), &args)?;
        (self.function)(args)
    }

    fn is_updated(&self) -> JuizResult<bool> {
        self.is_updated_exclude(&"".to_string())
    }

    fn is_updated_exclude(&self, arg_name: &String) -> JuizResult<bool> {
        if self.outlet.memo().is_empty() {
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

    fn invoke<'b>(&'b self) -> JuizResult<Capsule> {
        log::trace!("Processimpl({:?})::invoke() called", self.identifier());
        self.invoke_exclude(&"".to_string(), Capsule::from(jvalue!({})))
    }


    fn invoke_exclude<'b>(&self, arg_name: &String, value: Capsule) -> JuizResult<Capsule> {
        if !self.is_updated_exclude(arg_name)? {
            if self.outlet.memo().is_empty() {
                return Err(anyhow::Error::from(JuizError::ProcessOutputMemoIsNotInitializedError{id: self.identifier().clone()}));
            }
            //return Ok(Output::new_from_value(self.output_memo.borrow().get_value()?.clone()));
            println!("memo is requested");
            return Ok(self.outlet.memo().clone());
        }
        
        let result_value = self.call(self.collect_values_exclude(arg_name, Capsule::from(value))?)?;
        //self.output_memo.borrow_mut().get_value()?.clone_from(&result_value.get_value()?);
        //log::trace!("result_value = {:?}", result_value);
        self.outlet.set_value(result_value.clone())?;
        Ok(result_value)
    }

    fn execute(&self) -> JuizResult<Capsule> {
        self.outlet.push(self.invoke()?)
    }

    fn push_by(&self, arg_name: &String, value: &Capsule) -> JuizResult<Capsule> {
        self.outlet.push(self.invoke_exclude(arg_name, value.clone())?)
    }
    
    fn get_output(&self) -> Option<Capsule> {
        Some(self.outlet.memo().clone())
    }

    fn notify_connected_from(&mut self, source: Arc<Mutex<dyn Process>>, connecting_arg: &String, connection_manifest: Value) -> JuizResult<Capsule> {
        log::trace!("ProcessImpl(id={:?}).notify_connected_from(source=Process()) called", self.identifier());
        let id = self.identifier().clone();
        match self.inlet_mut(connecting_arg.as_str()) {
            None => { 
                log::error!("inlet_mut() in ProcessImpl::notify_connected_from() failed.");
                return Err(anyhow::Error::from(JuizError::ArgumentCanNotFoundByNameError{name: connecting_arg.clone()}));
            },
            Some(inlet) => {
                inlet.insert(
                    Box::new(SourceConnectionImpl::new(id, source, connection_manifest.clone(), connecting_arg.clone())?)
                )
            }
        }
        log::trace!("ProcessImpl(id={:?}).notify_connected_from(source=Process()) exit", self.identifier());
        Ok(connection_manifest.into())
    }

    fn try_connect_to(&mut self, destination: Arc<Mutex<dyn Process>>, arg_name: &String, connection_manifest: Value) -> JuizResult<Capsule> {
        log::info!("ProcessImpl(id={:?}).try_connect_to(destination=Process()) called", self.identifier());
        let destination_id = juiz_lock(&destination).context("ProcessImpl::try_connect_to()")?.identifier().clone();
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
        let mut v: Vec<&Box<dyn SourceConnection>> = Vec::new();
        for inlet in self.inlets.iter() {
            for sc in inlet.source_connections() {
                v.push(&sc);
            }
        }
        Ok(v)
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