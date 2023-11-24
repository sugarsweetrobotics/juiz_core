

use crate::identifier::Identifier;
use crate::process::*;

pub trait ProcessRack {

    
    fn process(&mut self, _id: &Identifier) -> Option<&mut Box<dyn Process>> {
        None
    }
}