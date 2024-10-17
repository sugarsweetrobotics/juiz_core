
use crate::prelude::*;
use super::manifest_description::Description;



pub struct ContainerManifest {
    pub type_name: String,
    pub description: Description,
    pub parent_type_name: Option<String>,
}

impl ContainerManifest {
    pub fn new(type_name: &str) -> Self {
        ContainerManifest {
            type_name: type_name.to_owned(),
            description: "".into(), 
            parent_type_name: None,
        }
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = description.into();
        self
    }

    pub fn parent(mut self, parent_type_name: &str) -> Self {
        self.parent_type_name = Some(parent_type_name.to_owned());
        self
    }

}


impl Into<Value> for ContainerManifest {
    fn into(self) -> Value {
        match self.parent_type_name {
            Some(ptn) => {
                jvalue!({
                    "type_name": self.type_name,
                    "description": self.description.to_str(),
                    "parent_type_name": ptn.as_str(),
                })
            }
            None => {
                jvalue!({
                    "type_name": self.type_name,
                    "description": self.description.to_str(),
                })
            }
        }        
    }
}