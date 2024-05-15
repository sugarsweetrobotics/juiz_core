use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::{connections::DestinationConnection, jvalue, utils::juiz_lock, JuizResult, Value};

use super::capsule::Capsule;


pub struct Outlet {
    destination_connections: HashMap<String, Box<dyn DestinationConnection>>,
    output_memo: Arc<Mutex<Capsule>>,
}


impl Outlet {

    pub fn new() -> Outlet {

        Outlet{
            destination_connections: HashMap::new(),
            output_memo: Arc::new(Mutex::new(Capsule::empty())),
        }
    }

    pub fn push(&self, output: Arc<Mutex<Capsule>>) -> JuizResult<Arc<Mutex<Capsule>>> {
        for (_name, dc) in self.destination_connections.iter() {
            let _ = dc.push(Arc::clone(&output))?;
        }
        return Ok(output);
    }

    pub fn profile_full(&self) -> JuizResult<Value> {
        Ok(jvalue!({
            "destination_connections": self.destination_connections.iter().map(| (_name, dc) | -> Value { dc.profile_full().unwrap() }).collect::<Vec<Value>>()
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

    pub(crate) fn memo(&self) -> Arc<Mutex<Capsule>> {
        self.output_memo.clone()//.borrow()
    }

    #[allow(dead_code)]
    pub(crate) fn memo_mut(&self) -> Arc<Mutex<Capsule>> {
        self.output_memo.clone()//borrow_mut()
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
        //self.output_memo = capsule;//.replace(capsule);
        juiz_lock(&self.output_memo)?.replace(capsule);
        return Ok(())
    }
}