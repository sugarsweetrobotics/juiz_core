
use crate::prelude::*;
use super::manifest_description::Description;



#[derive(Clone, Debug)]
pub enum ArgumentType {
    Int,
    String,
    Image,
}

#[derive(Clone, Debug)]
pub struct ArgumentManifest {
    pub type_name: ArgumentType,
    pub name: String,
    pub description: Description,
    pub default: Value
}

impl ArgumentManifest {

    pub fn new(type_name: ArgumentType, name: &str, description: Description, default: Value) -> Self {
        ArgumentManifest {
            type_name,
            name: name.to_owned(),
            description,
            default
        }
    }

    pub fn new_int(name: &str, description: &str, default: i64) -> Self {
        Self::new(ArgumentType::Int, name, description.into(), default.into())
    }

    pub fn new_string(name: &str, description: &str, default: &str) -> Self {
        Self::new(ArgumentType::Int, name, description.into(), default.into())
    }
    
}

impl Into<Value> for ArgumentManifest {
    fn into(self) -> Value {
        jvalue!({
            "name": self.name,
            "description": self.description.to_str(),
            "default": self.default,
        })
    }
}

impl From<(&str, &str, i64)> for ArgumentManifest {
    fn from(value:(&str, &str, i64)) -> Self {
        ArgumentManifest::new_int(value.0, value.1, value.2)
    }
}

impl From<(&str, &str, &str)> for ArgumentManifest {
    fn from(value:(&str, &str, &str)) -> Self {
        ArgumentManifest::new_string(value.0, value.1, value.2)
    }
}