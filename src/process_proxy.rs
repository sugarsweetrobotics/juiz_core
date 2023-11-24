

use crate::process::*;
use crate::broker::*;
use crate::identifier::*;

struct ProcessProxy {
    broker: Box<dyn Broker>,
    id: Identifier
}


impl Process for ProcessProxy {
    fn identifier(&self) -> Identifier {
        self.id.clone()
    }

    fn call(&self, args: crate::Value) -> Result<crate::Value, crate::error::JuizError> {
        // self.broker.call_process(&self.identifier(), args)
        todo!("To be implemented");

    }

    fn invoke(&mut self) -> Result<crate::Value, crate::error::JuizError> {
        todo!()
    }

    fn is_updated(&self) -> Result<bool, crate::error::JuizError> {
        todo!()
    }

    
}

