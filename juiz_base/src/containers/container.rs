use std::fmt::Display;

use mopa::mopafy;
use crate::{object::JuizObject, prelude::*, processes::ProcessPtr};

pub trait Container : Display + mopa::Any + JuizObject{
    
    fn manifest(&self) -> &ContainerManifest;

    fn process(&self, name_or_id: &String) -> Option<ProcessPtr>;

    fn processes(&self) -> Vec<ProcessPtr>;

    fn register_process(&mut self, p: ProcessPtr) -> JuizResult<ProcessPtr>;

    fn purge_process(&mut self, name_or_id: &String) -> JuizResult<()>;

    fn clear(&mut self) -> JuizResult<()>;
}

mopafy!(Container);

