

use crate::Value;
use crate::error::JuizError;
use crate::process::*;
use crate::broker::*;
use crate::identifier::*;

#[allow(unused)]
struct ProcessProxy {
    broker: Box<dyn Broker>,
    id: Identifier
}


impl Process for ProcessProxy {
    fn identifier(&self) -> &Identifier {
        &self.id
    }

    fn call(&self, _args: crate::Value) -> Result<crate::Value, crate::error::JuizError> {
        // self.broker.call_process(&self.identifier(), args)
        todo!("To be implemented");

    }

    fn invoke(&self) -> Result<crate::Value, crate::error::JuizError> {
        todo!()
    }

    fn invoke_exclude<'b>(&self, _: &String, _: Value) -> Result<Value, JuizError> {
        todo!()
    }

    fn is_updated(&self) -> Result<bool, crate::error::JuizError> {
        todo!()
    }

    fn is_updated_exclude(& self, _caller_id: &Identifier) -> Result<bool, JuizError> {
        todo!()
    }

    fn execute(&self) -> Result<crate::Value, crate::error::JuizError> {
        todo!()
    }

    fn push_by(&self, _arg_name: &String, _value: &Value) -> Result<Value, JuizError> {
        todo!()
    }

    fn get_output(&self) -> Option<Value> {
        None
    }

    fn connected_from<'b>(&'b mut self, _source: std::sync::Arc<std::sync::Mutex<dyn Process>>, _connecting_arg: &String, _connection_manifest: Value) -> Result<Value, JuizError> {
        todo!()
    }

    fn connection_to(&mut self, _target: std::sync::Arc<std::sync::Mutex<dyn Process>>, _connect_arg_to: &String, _connection_manifest: Value) -> Result<Value, JuizError> {
        todo!()
    }


    
}



impl Drop for ProcessProxy {
    fn drop(&mut self) {
        todo!()
    }
}