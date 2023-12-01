
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Mutex, Arc};

use crate::identifier::Identifier;
use crate::process::{Process, ProcessFunction, Connectable};
use crate::value::*;
use crate::error::JuizError;
use crate::manifest_checker::{check_manifest_before_call, check_process_manifest};
use crate::source_connection_impl::*;
use crate::source_connection::*;
use crate::destination_connection_impl::*;
use serde_json::Map;

pub struct ProcessImpl {
    manifest: Value,
    // default_manifest: Value, 
    function: ProcessFunction,
    fullpath: Identifier,
    // source_connections: SourceConnectionRack,
    source_connections: HashMap<String, Box<dyn SourceConnection>>,
    destination_connections: DestinationConnectionRack,
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

impl ProcessImpl {

    pub fn new(name: &str, manif: Value, func: ProcessFunction) -> Result<Self, JuizError> {
        match check_process_manifest(manif) {
            Ok(manif) => {
                let fullpath = name.to_string();
                Ok(ProcessImpl{manifest: manif, function: func, fullpath: fullpath,
                    //source_connections: SourceConnectionRack::new(),
                    source_connections: HashMap::new(),
                    destination_connections: DestinationConnectionRack::new(),
                    output_memo: RefCell::new(jvalue!(null)) })
            },
            Err(e) => {
                Err(e)
            }
        }
    }

    pub fn source_connection(&mut self, name: &String) -> Option<&Box<dyn SourceConnection>> {
        self.source_connections.get(name)
    }


    fn collect_values_exclude(&self, arg_name: &String, arg_value: Value) -> Result<Value, JuizError>{
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
}

impl Process for ProcessImpl {
    
    fn call(&self, args: Value) -> Result<Value, JuizError> {
        check_manifest_before_call(&(self.manifest), &args)?;
        Ok((self.function)(args)?)
    }

    fn identifier(&self) -> &Identifier {
        &self.fullpath
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

    fn invoke<'b>(&'b self) -> Result<crate::Value, crate::error::JuizError> {
        self.invoke_exclude(&"".to_string(), jvalue!({}))
    }


    fn invoke_exclude<'b>(&self, arg_name: &String, value: Value) -> Result<Value, JuizError> {
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
        Ok(self.destination_connections.push(self.invoke()?)?)
    }

    fn push_by(&self, arg_name: &String, value: &Value) -> Result<Value, JuizError> {
        Ok(self.destination_connections.push(self.invoke_exclude(arg_name, value.clone())?)?)
    }
    
    fn get_output(&self) -> Option<Value> {
        if self.output_memo.borrow().is_null() {
            return None
        }
        Some(self.output_memo.borrow().clone())
    }

    fn connected_from(&mut self, source: Arc<Mutex<dyn Process>>, connecting_arg: &String, connection_manifest: Value) -> Result<Value, JuizError> {
        self.source_connections.insert(connecting_arg.clone(), 
            Box::new(SourceConnectionImpl::new(self.identifier().clone(), source, connection_manifest.clone(), connecting_arg.clone())?)
        );
        Ok(connection_manifest)
    }

    fn connection_to(&mut self, target: Arc<Mutex<dyn Process>>, arg_name: &String, connection_manifest: Value) -> Result<Value, JuizError> {
        self.destination_connections.append(arg_name, DestinationConnectionImpl::new(&self.identifier(), target, connection_manifest.clone(), arg_name.clone())?)?;
        Ok(connection_manifest)
    }
}

impl Connectable for ProcessImpl {

    fn is_connected_from(&self) -> bool {
        todo!()
    }
}

impl Drop for ProcessImpl {
    fn drop(&mut self) {
        //self.source_connections.drop();
    }
}