
use crate::prelude::*;
use super::{argument_manifest::{ArgumentManifest, ArgumentType}, manifest_description::Description};



pub struct ProcessManifest {
    pub type_name: String,
    pub description: Description,
    pub arguments: Vec<ArgumentManifest>,
}

impl ProcessManifest {
    pub fn new(type_name: &str) -> Self {
        ProcessManifest {
            type_name: type_name.to_owned(),
            description: "".into(), 
            arguments: Vec::new()
        }
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = description.into();
        self
    }

    pub fn add_arg(mut self, arg: ArgumentManifest) -> Self {
        self.arguments.push(arg);
        self
    }

    pub fn add_int_arg(self, name: &str, description: &str, default: i64) -> Self {
        self.add_arg((name, description, default).into())
    }

    pub fn add_image_arg(self, name: &str, description: &str) -> Self {
        self.add_arg(ArgumentManifest::new(ArgumentType::Image, name, description.into(), jvalue!({})).into())
    }

    pub fn add_string_arg(self, name: &str, description: &str, default: &str) -> Self {
        self.add_arg((name, description, default).into())
    }

}

fn arguments_to_object(args: Vec<ArgumentManifest>) -> Value {
    let mut v : serde_json::Map<String, Value> = serde_json::Map::new();
    for a in args.into_iter() {
        v.insert(a.name.clone(), a.into());
    }
    v.into()
}

impl Into<Value> for ProcessManifest {
    fn into(self) -> Value {
        jvalue!({
            "type_name": self.type_name,
            "description": self.description.to_str(),
            "arguments": arguments_to_object(self.arguments)
        })
    }
}