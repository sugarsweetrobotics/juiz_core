use std::{collections::HashMap, cell::RefCell};

use crate::{jvalue, connections::DestinationConnection, Value, JuizResult};

use super::Output;


pub struct Outlet {
    destination_connections: HashMap<String, Box<dyn DestinationConnection>>,
    output_memo: RefCell<Output>,
}


impl Outlet {

    pub fn new() -> Outlet {

        Outlet{
            destination_connections: HashMap::new(),
            output_memo: RefCell::new(Output::new_from_value(jvalue!(null))),
        }
    }

    pub fn push(&self, output: Output) -> JuizResult<Output> {
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

    pub(crate) fn is_buffer_empty(&self) -> JuizResult<bool> {
        Ok(self.output_memo.borrow().get_value()?.is_null())
    }

    pub(crate) fn get_value(&self) -> JuizResult<Value> {
        self.output_memo.borrow().get_value()
    }


    pub(crate) fn set_value(&self, value: Value) -> JuizResult<()> {
        self.output_memo.borrow_mut().set_value(value)
    }
}