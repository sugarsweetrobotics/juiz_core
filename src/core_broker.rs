



use crate::identifier::*;
use crate::process_rack_impl::ProcessRackImpl;
use crate::value::*;
use crate::error::*;
use crate::broker::*;
use crate::process::*;
use crate::process_rack::*;

pub struct CoreBroker {
    manifest: Value,
    process_rack: ProcessRackImpl
}

impl CoreBroker {

    pub fn new(manifest: Value) -> CoreBroker {
        CoreBroker{manifest: manifest, process_rack: ProcessRackImpl::new()}
    }

    pub fn push_process(&mut self, p: Box<dyn Process>) -> &mut Self {
        self.process_rack.push(p);
        self
    }
}

impl<'a> Broker for CoreBroker {

    fn is_in_charge_for_process(&mut self, id: &Identifier) -> bool {
        self.process_rack.process(id).is_some()
    }

    fn call_process(&mut self, id: &Identifier, args: Value) -> Result<Value, JuizError> {
        match self.process_rack.process(id) {
            None => return Err(JuizError::ProcessCanNotFoundError{}),
            Some(p) => return p.call(args)
        }
    }

    fn connect_process_to(&mut self, source_process_id: &Identifier, arg_name: &String, target_process_id: &Identifier) -> Result<Value, JuizError> {
        todo!()
    }
}