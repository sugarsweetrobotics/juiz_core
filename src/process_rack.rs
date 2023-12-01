
use std::sync::{Arc, Mutex};

use crate::identifier::Identifier;
use crate::process::*;

pub trait ProcessRack {

    fn process(&mut self, id: &Identifier) -> Option<&Arc<Mutex<dyn Process>>> ;
}