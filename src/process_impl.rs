
use std::cell::{RefCell, Cell};
use std::collections::{LinkedList, HashMap};
use std::rc::Rc;

use crate::identifier::Identifier;
use crate::process::{Process, ProcessFunction, Connectable};
use crate::value::*;
use crate::error::JuizError;
use crate::manifest_checker::{check_manifest_before_call, check_process_manifest};
use crate::connection::*;
use serde_json::Map;

#[derive(Debug)]
pub struct ProcessImpl<'a> {
    manifest: Value,
    function: ProcessFunction,
    fullpath: Identifier,
    source_connections: SourceConnectionRack<'a>,
    //destination_connections: ConnectionRack,
    output_memo: Option<Value>,
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

impl<'a> ProcessImpl<'a> {

    pub fn new(manif: Value, func: ProcessFunction) -> Result<Self, JuizError> {
        match check_process_manifest(manif) {
            Ok(manif) => {
                let fullpath = manif.get("name").unwrap().as_str().unwrap().to_string();
                Ok(ProcessImpl{manifest: manif, function: func, fullpath: fullpath, source_connections: SourceConnectionRack::new(),
                    //destination_connections: ConnectionRack::new(),
                    output_memo: None })
            },
            Err(e) => {
                Err(e)
            }
        }
    }

    pub fn source_connection(&'a mut self, name: &String) -> Option<&'a mut SourceConnection> {
        self.source_connections.connection_mut(name)
    }
}

impl<'a> Process for ProcessImpl<'a>  {
    
    fn call(&self, args: Value) -> Result<Value, JuizError> {
        match check_manifest_before_call(&(self.manifest), &args) {
            Ok(_) => match (self.function)(args) {
                Ok(v) => Ok(v),
                Err(e) => Err(e) // 内部の知らんエラー
            },
            Err(e) => Err(e) // マニフェスト合わないエラー
        }
    }

    fn identifier(&self) -> Identifier {
        self.fullpath.clone()
    }

    fn is_updated(&self) -> Result<bool, JuizError> {
        if self.output_memo.is_none() {
            return Ok(true)
        }   
        Ok(self.source_connections.is_updated())
        /*
        match argument_manifest(&self.manifest) {
            Err(err) => return Err(err),
            Ok(args) => {
                let mut updated_flag : bool = false;
                for (name, _) in args.into_iter() {
                    match self.source_connections.connection(&name) {
                        None => {},
                        Some(con) => {
                            updated_flag = updated_flag || con.is_source_updated();
                        }
                    }
                }
                Ok(updated_flag)
            }
        }
        */
    }

    

    
    fn invoke<'b>(&'b mut self) -> Result<crate::Value, crate::error::JuizError> {
        match self.is_updated() {
            Err(e) => return Err(e),
            Ok(flag) => {
                if !flag {
                    match &self.output_memo {
                        None => return Err(JuizError::ProcessOutputMemoIsNotInitializedError{}),
                        Some(memo) => {
                            return Ok(memo.clone());
                        }
                    }
                }
            }
        }
        match self.source_connections.collect_values() {
            Err(e) => return Err(e),
            Ok(mut value) => {
                match argument_manifest(&self.manifest) {
                    Err(e) => return Err(e),
                    Ok(manif_map) => {
                        let value_map = value.as_object_mut().unwrap();
                        for (k, v) in manif_map.into_iter() {
                            if !value_map.contains_key(&k) {

                                value_map.insert(k, v.get("default").unwrap().clone());
                            }
                        }
                        match self.call(jvalue!(value_map)) {
                            Err(err) => return Err(err),
                            Ok(result_value) => {
                                self.output_memo = Some(result_value.clone());
                                return Ok(result_value);
                            }
                        }
                    }
                }
            }
        }
    }

}


impl<'a> Connectable<'a> for ProcessImpl<'a> {

    fn connected_from<'b>(&'b mut self, source: &'a Rc<RefCell<&'a mut dyn Process>>, connecting_arg: &String) -> () {
        self.source_connections.append(connecting_arg, 
            SourceConnection::new(&self.identifier(), source)
        );
        //self
    }

    fn is_connected_from(&self) -> bool {
        todo!()
    }
}