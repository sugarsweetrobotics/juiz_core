use std::collections::HashMap;

use crate::{jvalue, connections::DestinationConnection, Value, JuizResult};




pub struct Outlet {
    destination_connections: HashMap<String, Box<dyn DestinationConnection>>,
}


impl Outlet {

    pub fn new() -> Outlet {

        Outlet{
            destination_connections: HashMap::new(),
        }
    }

    pub fn push(&self, output: Value) -> JuizResult<Value> {
        for (_name, dc) in self.destination_connections.iter() {
            let _ = dc.push(&output)?;
        }        
        return Ok(output);
    }

    pub fn profile_full(&self) -> JuizResult<Value> {
        Ok(jvalue!({
            "destination_connections": self.destination_connections.iter().map(| (_name, dc) |{dc.profile_full().unwrap()}).collect::<Vec<Value>>()
        }))
    }

    pub(crate) fn insert(&mut self, name: String, con: Box<crate::connections::DestinationConnectionImpl>) -> () {
        self.destination_connections.insert(name, con);
    }

    pub(crate) fn destination_connections(&self) -> JuizResult<Vec<&Box<dyn DestinationConnection>>> {
        let mut v: Vec<&Box<dyn DestinationConnection>> = Vec::new();
        for c in self.destination_connections.values() {
            v.push(c);
        }
        Ok(v)
    }

}