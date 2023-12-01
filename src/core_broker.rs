

use std::sync::Arc;
use std::sync::Mutex;

use crate::identifier::*;
use crate::process_rack_impl::ProcessRackImpl;
use crate::value::*;
use crate::error::*;
use crate::broker::*;
use crate::process::*;
use crate::process_rack::*;

use crate::manifest_checker::*;

#[allow(unused)]
pub struct CoreBroker {
    manifest: Value,
    process_rack: ProcessRackImpl
}

impl CoreBroker {

    pub fn new(manifest: Value) -> Result<CoreBroker, JuizError> {
        match check_corebroker_manifest(manifest) {
            Err(err) => return Err(err),
            Ok(manif) => return Ok(CoreBroker{manifest: manif, process_rack: ProcessRackImpl::new()})
        }
    }

    pub fn push_process(&mut self, p: Arc<Mutex<dyn Process>>) -> Result<(), JuizError> {
        self.process_rack.push(p)
    }
}

impl<'a> Broker for CoreBroker {

    fn is_in_charge_for_process(&mut self, id: &Identifier) -> bool {
        self.process_rack.process(id).is_some()
    }

    fn call_process(&mut self, id: &Identifier, args: Value) -> Result<Value, JuizError> {
        match self.process_rack.process(id) {
            None => return Err(JuizError::ProcessCanNotFoundError{}),
            Some(p) => {
                match p.try_lock() {
                    Err(_e) => Err(JuizError::CoreBrokerCanNotLockProcessMutexError{}),
                    Ok(proc) => proc.call(args)
                }
            }
        }
    }

    #[allow(unused)]
    fn connect_process_to(&mut self, source_process_id: &Identifier, arg_name: &String, target_process_id: &Identifier) -> Result<Value, JuizError> {
        todo!()
    }
}