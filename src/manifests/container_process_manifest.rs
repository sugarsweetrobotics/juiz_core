
use crate::{prelude::*, value::obj_get_str};
use super::{argument_manifest::{ArgumentManifest, ArgumentType}, manifest_description::Description};



pub struct ContainerProcessManifest {
    pub type_name: String,
    pub container_manifest: Value,
    pub description: Description,
    pub arguments: Vec<ArgumentManifest>,
}

impl ContainerProcessManifest {
    pub fn new(container_manifest: Value, type_name: &str) -> Self {
        ContainerProcessManifest {
            type_name: type_name.to_owned(),
            container_manifest,
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

impl Into<Value> for ContainerProcessManifest {
    fn into(self) -> Value {
        jvalue!({
            "type_name": self.type_name,
            "container_type_name": obj_get_str(&self.container_manifest, "type_name").unwrap(),
            "description": self.description.to_str(),
            "arguments": arguments_to_object(self.arguments)
        })
    }
}