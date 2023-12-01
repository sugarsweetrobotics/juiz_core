use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::source_connection::SourceConnection;


pub struct SourceConnectionRack {
    connection_map: HashMap<String, Arc<Mutex<dyn SourceConnection>>>
}

impl<'a> SourceConnectionRack {

    pub fn new() -> Self {
        SourceConnectionRack{connection_map: HashMap::new()}
    }

    pub fn append<SC : SourceConnection + std::clone::Clone + 'static>(&mut self, arg_name: &String,  connection: SC) -> &mut Self {
        self.connection_map.insert(arg_name.clone(), Arc::new(Mutex::new(connection.clone())));
        self
    }

    pub fn remove_connection(&mut self, arg_name: &String) -> &mut Self {
        self.connection_map.remove(arg_name);
        self
    }

    pub fn connection_map(&self) -> &HashMap<String, Arc<Mutex<dyn SourceConnection>>> {
        &self.connection_map
    }

    pub fn connection_map_mut(&mut self) -> &mut HashMap<String, Arc<Mutex<dyn SourceConnection>>> {
        &mut self.connection_map
    }

    pub fn connection_mut(&mut self, arg_name: &String) -> Option<&mut Arc<Mutex<dyn SourceConnection>>> {
        self.connection_map.get_mut(arg_name)
    }

    pub fn connection(&self, arg_name: &String) -> Option<&Arc<Mutex<dyn SourceConnection>>>  {
        self.connection_map.get(arg_name)
    }

    /*/
    pub fn collect_values_exclude(&self, arg_name: &String, arg_value: Value, caller_id: &Identifier) -> Result<Value, JuizError> {
        let mut args = Map::new();
        for (k, v) in self.connection_map.iter() {
            if k == arg_name {
                args.insert(arg_name.clone(), arg_value.clone());
            } else {
                match v.try_lock() {
                    Err(_try_lock_error) => return Err(JuizError::SourceConnectionCanNotBorrowMutableProcessReferenceError{}),
                    Ok(mut p) => {
                        match p.invoke_source() {
                            Err(e) => return Err(e),
                            Ok(value) => {
                                args.insert(k.clone(), value);
                            }
                        }
                    }
                }
            }
        }
        Ok(jvalue!(args))
    }
    */
    /*/
    pub fn is_updated(&self) -> Result<bool, JuizError> {
        for (_k, v) in (&self.connection_map).into_iter() {
            match v.try_lock() {
                Err(e) => return Err(JuizError::SourceConnectionCanNotBorrowMutableProcessReferenceError{}),
                Ok(p) => {
                    match p.is_source_updated() {
                        Err(e) => return Err(e),
                        Ok(f) => {
                            if f { return Ok(true); }
                        }
                    }
                }
            }
        }
        return Ok(false);
    }
    */
    /*
    pub fn is_updated_exclude(&self, arg_name: &String) -> Result<bool, JuizError> {
        for (k, v) in (&self.connection_map).into_iter() {
            if k == arg_name {
                continue;
            }
            match v.try_lock() {
                Err(_e) => return Err(JuizError::SourceConnectionCanNotBorrowMutableProcessReferenceError{}),
                Ok(p) => {
                    match p.is_source_updated() {
                        Err(e) => return Err(e),
                        Ok(f) => {
                            if f { return Ok(true); }
                        }
                    }
                }
            }
            
        }
        return Ok(false);
    }
    */
}


impl Drop for SourceConnectionRack {
    fn drop(&mut self) {
    //    for (k, v) in (&self.connection_map).into_iter() {
    //        v.borrow().drop();
    //    }
    }
}