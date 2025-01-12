use std::fmt::Display;

use mopa::mopafy;
use crate::{container_identifier::ContainerIdentifier, manifests::ContainerProfile, object::JuizObject, prelude::*, processes::ProcessPtr};

pub trait Container : Display + mopa::Any {

    fn identifier(&self) -> ContainerIdentifier;

    fn profile(&self) -> JuizResult<ContainerProfile>;
    
    // fn manifest(&self) -> &ContainerManifest;

    fn process(&self, name_or_id: &String) -> Option<ProcessPtr>;

    fn processes(&self) -> Vec<ProcessPtr>;

    fn register_process(&mut self, p: ProcessPtr) -> JuizResult<ProcessPtr>;

    fn purge_process(&mut self, name_or_id: &String) -> JuizResult<()>;

    fn clear(&mut self) -> JuizResult<()>;

}

mopafy!(Container);

