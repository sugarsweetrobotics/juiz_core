/// inlet.rs
/// 
/// 
/// 
use crate::{connections::SourceConnection, jvalue, CapsulePtr, Identifier, JuizResult, Value};

pub struct Inlet {
    name: String,
    source_connections: Vec<Box<dyn SourceConnection>>,
    default_value: CapsulePtr,
    buffer: CapsulePtr,
}


impl Inlet {

    pub fn new(name: &str, default_value: Value) -> Inlet {
        Inlet{ 
            name: name.to_owned(), 
            default_value: default_value.into(),
            source_connections: Vec::new(),
            buffer: CapsulePtr::new(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn source_connection_by_identifier(&mut self, identifier: &Identifier) -> Option<&Box<dyn SourceConnection>> {
        for c in self.source_connections.iter() {
            if c.identifier() == identifier {
                return Some(c)
            }
        }
        return None
    }

    pub fn source_connections(&self) -> &Vec<Box<dyn SourceConnection>> {
        return &self.source_connections
    }

    pub fn source_connections_mut(&mut self) -> &mut Vec<Box<dyn SourceConnection>> {
        return &mut self.source_connections
    }

    pub fn profile_full(&self) -> JuizResult<Value> {
        Ok(jvalue!({
            "name": self.name,
            "source_connections": self.source_connections.iter().map(|sc| -> Value {
                sc.profile_full().unwrap_or_else(|e| { jvalue!(format!("Error. SourceConnection::profile_full() failed. Error {e:}")) })
            }).collect::<Vec<Value>>()
        }).into())
    }

    pub fn is_updated(&self) -> JuizResult<bool> {
        //self.source_connections.iter().find_map(|sc| { if sc.is_source_updated() })
        for sc in self.source_connections.iter() {
            if sc.is_source_updated()? {
                return Ok(true);
            } 
        }
        Ok(false)
    }
   
    // データを収集。pullする。あとからの接続を優先
    pub fn collect_value(&self) -> CapsulePtr {
        for sc in self.source_connections.iter() {
            match sc.pull() {
                Err(_) => {},
                Ok(output) => {
                    return output.clone();
                }
            }
        }
        match self.buffer.is_empty() {
            Err(e) => {
                return self.default_value.clone();
            },
            Ok(v) => {
                if v {
                    return self.default_value.clone();
                }
            }
        } 
        return self.buffer.clone();
    }

    pub(crate) fn insert(&mut self, con: Box<crate::connections::SourceConnectionImpl>) {
        self.source_connections.push(con);
    }

    pub fn bind(&mut self, value: CapsulePtr) -> JuizResult<CapsulePtr> {
        //self.buffer = value;
        self.buffer.replace(value);
        Ok(self.buffer.clone())
    }
}