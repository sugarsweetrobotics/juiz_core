use crate::{jvalue, Value};
use super::manifest_description::Description;



pub struct ContainerManifest {
    pub type_name: String,
    pub description: Description,
}

impl ContainerManifest {
    pub fn new(type_name: &str) -> Self {
        ContainerManifest {
            type_name: type_name.to_owned(),
            description: "".into(), 
        }
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = description.into();
        self
    }


}


impl Into<Value> for ContainerManifest {
    fn into(self) -> Value {
        jvalue!({
            "type_name": self.type_name,
            "description": self.description.to_str(),
        })
    }
}