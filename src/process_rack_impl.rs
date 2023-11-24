

use crate::identifier::*;
use crate::process_rack::ProcessRack;
use crate::process::*;

pub struct ProcessRackImpl {
    processes: Vec<Box<dyn Process>>
}

impl ProcessRackImpl {
    
    pub fn new() -> Self {
        ProcessRackImpl{processes: Vec::new()}
    }

    pub fn push(&mut self, proc: Box<dyn Process>) -> &mut Self {
        self.processes.push(proc);
        self
    }
}

impl ProcessRack for ProcessRackImpl {

    fn process(&mut self, id: &Identifier) -> Option<&mut Box<dyn Process>> {
        for p in self.processes.iter_mut() {
            if &p.identifier() == id {
                return Some(p);
            }
        }
        None
    }

}