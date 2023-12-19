
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use anyhow::Context;
use serde_json::Map;


use crate::identifier::{identifier_from_manifest, create_identifier_from_manifest};
use crate::object::{JuizObjectCoreHolder, ObjectCore, JuizObjectClass};
use crate::utils::manifest_util::get_hashmap_mut;
use crate::value::{obj_get_str, obj_get_obj, obj_merge_mut};
use crate::{Value, jvalue, Process, Identifier, JuizError, JuizResult, JuizObject};

use crate::utils::{check_manifest_before_call, check_process_manifest, juiz_lock};
use crate::connections::{SourceConnection, SourceConnectionImpl, DestinationConnection, DestinationConnectionImpl};

pub struct ProcessImpl {
    core: ObjectCore,
    manifest: Value,
    function: Box<dyn Fn(Value) -> JuizResult<Value>>,
    identifier: Identifier,
    source_connections: HashMap<String, Box<dyn SourceConnection>>,
    destination_connections: HashMap<String, Box<dyn DestinationConnection>>,
    output_memo: RefCell<Value>,
}


pub fn argument_manifest(process_manifest: &Value) -> JuizResult<&Map<String, Value>>{
    obj_get_obj(process_manifest, "arguments")
}


impl ProcessImpl {

    pub fn new_with_class(class_name: JuizObjectClass, manif: Value, func: fn(Value) -> JuizResult<Value>) -> JuizResult<Self> {
        log::trace!("ProcessImpl::new(manifest={}) called", manif);
        let manifest = check_process_manifest(manif)?;
        let type_name = obj_get_str(&manifest, "type_name")?;
        let object_name = obj_get_str(&manifest, "name")?;
        Ok(ProcessImpl{
            core: ObjectCore::create(class_name, 
                type_name,
                object_name,
            ),
            manifest: manifest.clone(), 
            function: Box::new(func), 
            identifier: create_identifier_from_manifest("Process", &manifest)?,
            source_connections: HashMap::new(),
            destination_connections: HashMap::new(),
            output_memo: RefCell::new(jvalue!(null)) })
    }


    pub fn new(manif: Value, func: fn(Value) -> JuizResult<Value>) -> JuizResult<Self> {
        ProcessImpl::new_with_class(JuizObjectClass::Process("ProcessImpl"), manif, func)
    }

    pub(crate) fn clousure_new_with_class_name(class_name: JuizObjectClass, manif: Value, func: Box<impl Fn(Value) -> JuizResult<Value> + 'static>) -> JuizResult<Self> {
        log::trace!("ProcessImpl::new(manifest={}) called", manif);
        
        let manifest = check_process_manifest(manif)?;
        let type_name = obj_get_str(&manifest, "type_name")?;
        let object_name = obj_get_str(&manifest, "name")?;
        Ok(ProcessImpl{
            core: ObjectCore::create(class_name,
            type_name, object_name),
            manifest: manifest.clone(), 
            function: func, 
            identifier: identifier_from_manifest("core", "core", "Process", &manifest)?,
            source_connections: HashMap::new(),
            destination_connections: HashMap::new(),
            output_memo: RefCell::new(jvalue!(null)) })
    }

    pub(crate) fn _clousure_new(manif: Value, func: Box<impl Fn(Value) -> JuizResult<Value> + 'static>) -> JuizResult<Self> {
        ProcessImpl::clousure_new_with_class_name(JuizObjectClass::Process("ProcessImpl"), manif, func)
    }

    pub fn source_connection(&mut self, name: &String) -> Option<&Box<dyn SourceConnection>> {
        self.source_connections.get(name)
    }

    fn collect_values_exclude(&self, arg_name: &String, arg_value: Value) -> JuizResult<Value>{
        log::trace!("ProcessImpl({:?}).collect_values_exclude({:?}) called.", &self.identifier, arg_name);
        let mut value_map: Map<String, Value> = Map::new();
        value_map.insert(arg_name.clone(), arg_value.clone());
        
        for (k, v) in argument_manifest(&self.manifest)?.into_iter() {
            if k == arg_name { continue; }
            match self.source_connections.get(k) {
                None => { value_map.insert(k.clone(), v.get("default").unwrap().clone()); }
                Some(con) => {
                    value_map.insert(k.clone(), con.pull()?);                        
                }
            }
        }
        Ok(Value::from(value_map))
    }

    fn push_to_destinations(&self, output: Value) -> JuizResult<Value> {
        for (_k, v) in self.destination_connections.iter() {
            let _ = v.push(&output)?;
        }
        return Ok(output)
    }

    

}

impl JuizObjectCoreHolder for ProcessImpl {
    fn core(&self) -> &crate::object::ObjectCore {
        &self.core
    }
}

impl JuizObject for ProcessImpl {


    fn profile_full(&self) -> JuizResult<Value> {
        let mut sc_profs = jvalue!({});
        let sc_map = get_hashmap_mut(&mut sc_profs)?;
        for (key, value) in self.source_connections.iter() {
            sc_map.insert(key.to_owned(), value.profile_full()?);
        }

        let mut dc_profs = jvalue!({});
        let dc_map = get_hashmap_mut(&mut dc_profs)?;
        for (key, value) in self.destination_connections.iter() {
            dc_map.insert(key.to_owned(), value.profile_full()?);
        }

        let mut v = self.core.profile_full()?;
        obj_merge_mut(&mut v, &jvalue!({
            "source_connections": sc_profs,
            "destination_connections": dc_profs,
            "arguments": self.manifest.get("arguments").unwrap(),
        }))?;
        Ok(v)
    }
}

impl Process for ProcessImpl {
    
    fn manifest(&self) -> &Value { 
        &self.manifest
    }

    fn call(&self, args: Value) -> JuizResult<Value> {
        check_manifest_before_call(&(self.manifest), &args)?;
        Ok((self.function)(args)?)
    }

    fn is_updated(&self) -> JuizResult<bool> {
        self.is_updated_exclude(&"".to_string())
    }

    fn is_updated_exclude(&self, arg_name: &String) -> JuizResult<bool> {
        if self.output_memo.borrow().is_null() {
            return Ok(true)
        }
        for (k, p) in self.source_connections.iter() {
            if k == arg_name { continue; }
            if p.is_source_updated()? {
                return Ok(true);
            } 
        }
        Ok(false)
    }

    fn invoke<'b>(&'b self) -> JuizResult<Value> {
        log::trace!("Processimpl({:?})::invoke() called", self.identifier());
        self.invoke_exclude(&"".to_string(), jvalue!({}))
    }


    fn invoke_exclude<'b>(&self, arg_name: &String, value: Value) -> JuizResult<Value> {
        if !self.is_updated_exclude(arg_name)? {
            if self.output_memo.borrow().is_null() {
                return Err(anyhow::Error::from(JuizError::ProcessOutputMemoIsNotInitializedError{id: self.identifier().clone()}));
            }
            return Ok(self.output_memo.borrow().clone());
        }
        
        let result_value = self.call(self.collect_values_exclude(arg_name, value)?)?;
        self.output_memo.borrow_mut().clone_from(&result_value);
        Ok(result_value)
    }

    fn execute(&self) -> JuizResult<Value> {
        self.push_to_destinations(self.invoke()?)
    }

    fn push_by(&self, arg_name: &String, value: &Value) -> JuizResult<Value> {
        self.push_to_destinations(self.invoke_exclude(arg_name, value.clone())?)
    }
    
    fn get_output(&self) -> Option<Value> {
        if self.output_memo.borrow().is_null() {
            return None
        }
        Some(self.output_memo.borrow().clone())
    }

    fn notify_connected_from(&mut self, source: Arc<Mutex<dyn Process>>, connecting_arg: &String, connection_manifest: Value) -> JuizResult<Value> {
        log::info!("ProcessImpl(id={:?}).notify_connected_from(source=Process()) called", self.identifier());
        self.source_connections.insert(connecting_arg.clone(), 
            Box::new(SourceConnectionImpl::new(self.identifier().clone(), source, connection_manifest.clone(), connecting_arg.clone())?)
        );
        Ok(connection_manifest)
    }

    fn try_connect_to(&mut self, destination: Arc<Mutex<dyn Process>>, arg_name: &String, connection_manifest: Value) -> JuizResult<Value> {
        log::info!("ProcessImpl(id={:?}).try_connect_to(destination=Process()) called", self.identifier());
        let destination_id = juiz_lock(&destination).context("ProcessImpl::try_connect_to()")?.identifier().clone();
        self.destination_connections.insert(
            arg_name.clone(), 
            Box::new(DestinationConnectionImpl::new(
                &self.identifier(), 
                &destination_id,
                destination, 
                connection_manifest.clone(), 
                arg_name.clone())?));
        Ok(connection_manifest)
    }

    
    fn source_connections(&self) -> JuizResult<Vec<&Box<dyn SourceConnection>>> {
        let mut v: Vec<&Box<dyn SourceConnection>> = Vec::new();
        for c in self.source_connections.values() {
            v.push(c);
        }
        Ok(v)
    }

    fn destination_connections(&self) -> JuizResult<Vec<&Box<dyn DestinationConnection>>> {
        let mut v: Vec<&Box<dyn DestinationConnection>> = Vec::new();
        for c in self.destination_connections.values() {
            v.push(c);
        }
        Ok(v)
    }
}

impl Drop for ProcessImpl {
    fn drop(&mut self) {
        //self.source_connections.drop();
    }
}

unsafe impl Send for ProcessImpl {

}