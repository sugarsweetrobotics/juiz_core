
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use serde_json::Map;

use crate::value::obj_get_str;
use crate::{Value, jvalue, Process, ProcessFunction, Identifier, JuizError, JuizResult};

use crate::manifest_checker::{check_manifest_before_call, check_process_manifest};
use crate::connection::{SourceConnection, SourceConnectionImpl, DestinationConnection, DestinationConnectionImpl};

pub struct ProcessImpl {
    manifest: Value,
    function: ProcessFunction,
    identifier: Identifier,
    source_connections: HashMap<String, Box<dyn SourceConnection>>,
    destination_connections: HashMap<String, Box<dyn DestinationConnection>>,
    output_memo: RefCell<Value>,
}


pub fn argument_manifest(process_manifest: &Value) -> Result<Map<String, Value>, JuizError>{
    match process_manifest.get("arguments") {
        None => return Err(JuizError::ArgumentMissingError{}),
        Some(v) => {
            match v.as_object() {
                None => return Err(JuizError::ArgumentIsNotObjectError{}),
                Some(v_obj) => Ok(v_obj.clone())
            }
        }
    }
}

fn identifier_from_manifest(manifest: &Value) -> Identifier {
    obj_get_str(manifest, "name").unwrap().to_string()
}

impl ProcessImpl {

    pub fn new(manif: Value, func: ProcessFunction) -> Result<Self, JuizError> {
        log::trace!("ProcessImpl::new(manifest={}) called", manif);
        
        let manifest = check_process_manifest(manif)?;
        Ok(ProcessImpl{
            manifest: manifest.clone(), 
            function: func, 
            identifier: identifier_from_manifest(&manifest),
            source_connections: HashMap::new(),
            destination_connections: HashMap::new(),
            output_memo: RefCell::new(jvalue!(null)) })
    }

    pub fn source_connection(&mut self, name: &String) -> Option<&Box<dyn SourceConnection>> {
        self.source_connections.get(name)
    }

    fn collect_values_exclude(&self, arg_name: &String, arg_value: Value) -> Result<Value, JuizError>{
        log::trace!("ProcessImpl({:?}).collect_values_exclude({:?}) called.", self.identifier(), arg_name);
        let mut value_map: Map<String, Value> = Map::new();
        value_map.insert(arg_name.clone(), arg_value.clone());
        
        for (k, v) in argument_manifest(&self.manifest)?.into_iter() {
            if k == *arg_name { continue; }
            match self.source_connections.get(&k) {
                None => { value_map.insert(k, v.get("default").unwrap().clone()); }
                Some(con) => {
                    value_map.insert(k, con.pull()?);                        
                }
            }
        }
        Ok(Value::from(value_map))
    }

    fn push_to_destinations(&self, output: Value) -> Result<Value, JuizError> {
        for (_k, v) in self.destination_connections.iter() {
            match v.push(&output) {
                Err(e) => return Err(e),
                Ok(_) => {}
            }
        }
        return Ok(output)
    }

}

impl Process for ProcessImpl {
    
    fn manifest(&self) -> &Value { 
        &self.manifest
    }

    fn call(&self, args: Value) -> Result<Value, JuizError> {
        check_manifest_before_call(&(self.manifest), &args)?;
        Ok((self.function)(args)?)
    }

    fn identifier(&self) -> &Identifier {
        &self.identifier
    }

    fn is_updated(&self) -> Result<bool, JuizError> {
        self.is_updated_exclude(&"".to_string())
    }

    fn is_updated_exclude(&self, arg_name: &String) -> Result<bool, JuizError> {
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
                return Err(JuizError::ProcessOutputMemoIsNotInitializedError{});
            }
            return Ok(self.output_memo.borrow().clone());
        }
        
        let result_value = self.call(self.collect_values_exclude(arg_name, value)?)?;
        self.output_memo.borrow_mut().clone_from(&result_value);
        Ok(result_value)
    }

    fn execute(&self) -> Result<Value, JuizError> {
        Ok(self.push_to_destinations(self.invoke()?)?)
    }

    fn push_by(&self, arg_name: &String, value: &Value) -> Result<Value, JuizError> {
        Ok(self.push_to_destinations(self.invoke_exclude(arg_name, value.clone())?)?)
    }
    
    fn get_output(&self) -> Option<Value> {
        if self.output_memo.borrow().is_null() {
            return None
        }
        Some(self.output_memo.borrow().clone())
    }

    fn connected_from(&mut self, source: Arc<Mutex<dyn Process>>, connecting_arg: &String, connection_manifest: Value) -> JuizResult<Value> {
        println!("ProcessImpl(id={:?}).connected_from(source=Process()) called", self.identifier());
        self.source_connections.insert(connecting_arg.clone(), 
            Box::new(SourceConnectionImpl::new(self.identifier().clone(), source, connection_manifest.clone(), connecting_arg.clone())?)
        );
        Ok(connection_manifest)
    }

    fn connection_to(&mut self, destination: Arc<Mutex<dyn Process>>, arg_name: &String, connection_manifest: Value) -> JuizResult<Value> {
        println!("ProcessImpl(id={:?}).connect_to(destination=Process()) called", self.identifier());
        self.destination_connections.insert(
            arg_name.clone(), 
            Box::new(DestinationConnectionImpl::new(
                &self.identifier(), 
                destination, 
                connection_manifest.clone(), 
                arg_name.clone())?));
        Ok(connection_manifest)
    }
}

/*
impl Connectable for ProcessImpl {

    fn is_connected_from(&self) -> bool {
        todo!()
    }
}*/

impl Drop for ProcessImpl {
    fn drop(&mut self) {
        //self.source_connections.drop();
    }
}