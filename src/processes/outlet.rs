use std::{collections::HashMap, cell::RefCell};

use crate::{jvalue, connections::DestinationConnection, Value, JuizResult};

use super::capsule::Capsule;


pub struct Outlet {
    destination_connections: HashMap<String, Box<dyn DestinationConnection>>,
    output_memo: RefCell<Capsule>,
}


impl Outlet {

    pub fn new() -> Outlet {

        Outlet{
            destination_connections: HashMap::new(),
            output_memo: RefCell::new(Capsule::empty()),
        }
    }

    pub fn push(&self, output: Capsule) -> JuizResult<Capsule> {
        for (_name, dc) in self.destination_connections.iter() {
            let _ = dc.push(&output)?;
        }        
        return Ok(output);
    }

    pub fn profile_full(&self) -> JuizResult<Capsule> {
        Ok(jvalue!({
            "destination_connections": self.destination_connections.iter().map(| (_name, dc) | -> Value {dc.profile_full().unwrap().try_into().unwrap()}).collect::<Vec<Value>>()
        }).into())
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

    pub(crate) fn memo(&self) -> std::cell::Ref<Capsule> {
        self.output_memo.borrow()
    }

    #[allow(dead_code)]
    pub(crate) fn memo_mut(&self) -> std::cell::RefMut<Capsule> {
        self.output_memo.borrow_mut()
    }
    /*
    pub(crate) fn is_buffer_empty(&self) -> JuizResult<bool> {
        Ok(self.output_memo.borrow().get_value()?.is_null())
    }

    pub(crate) fn get_value(&self) -> JuizResult<&Value> {
        self.output_memo.borrow().get_value()
    }

    pub(crate) fn get_value_mut(&mut self) -> JuizResult<&mut Value> {
        self.output_memo.borrow().get_value_mut()
    }
    */

    pub(crate) fn set_value(&self, capsule: Capsule) -> JuizResult<()> {
        self.output_memo.replace(capsule);
        return Ok(())
    }
}