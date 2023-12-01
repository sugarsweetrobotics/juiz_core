use std::{cell::RefCell, rc::Rc};

use crate::{process::Process, error::JuizError};







pub trait ProcessFactory {

    fn create_process(&mut self, name: String) -> Result<Rc<RefCell<dyn Process>>, JuizError>;
}