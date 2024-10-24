use std::collections::HashMap;

use crate::prelude::*;
use crate::connections::DestinationConnection;



pub struct Outlet {
    name: String, 
    destination_connections: HashMap<String, Box<dyn DestinationConnection>>,
    output_memo: CapsulePtr,
    use_memo: bool,
}


impl Outlet {

    pub fn new(name: &str, use_memo: bool) -> Outlet {
        log::trace!("Outlet()::new(use_memo='{}') called", use_memo);
        Outlet{
            name: name.to_owned(),
            destination_connections: HashMap::new(),
            output_memo: Capsule::empty().into(),
            use_memo: use_memo,
        }
    }

    // pub fn use_memo(&self) -> bool {
    //     self.use_memo
    // }
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

    pub(crate) fn memo(&self) -> CapsulePtr {
        self.output_memo.clone()//.borrow()
    }

    #[allow(dead_code)]
    pub(crate) fn memo_mut(&self) -> CapsulePtr {
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

    /// 出力バッファー (memo) にデータを書き込む
    pub(crate) fn set_value(&self, capsule: CapsulePtr) -> CapsulePtr {
        log::trace!("Outlet({})::set_value() called", self.name);
        if self.use_memo {
            self.output_memo.replace(capsule);
        } else {
            log::trace!("Outlet({})::set_value() called but 'use_memo' property is set to false so value is spoiled.", self.name);
            return capsule;
        }
        self.memo()
    }

    /// 出力を出力接続 (DestinationConnection) に投げる
    /// 
    pub fn push(&self, output: CapsulePtr) -> JuizResult<CapsulePtr> {
        log::trace!("Outlet({})::push() called", self.name);
        for (_name, dc) in self.destination_connections.iter() {
            let _ = dc.push(output.clone())?;
        }
        return Ok(output);
    }

}