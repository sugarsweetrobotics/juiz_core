
use crate::value::{obj_get_array, obj_get_str, Value};

use super::{ContainerManifest, Description, ProcessManifest};




pub struct ComponentManifest {
    pub type_name: String,
    pub description: Description,
    pub language: String,
    pub containers: Vec<ContainerManifest>,
    pub processes: Vec<ProcessManifest>,
}

impl ComponentManifest {



    pub fn new(type_name: &str) -> Self {
        Self {
            type_name: type_name.to_owned(),
            description: Description::new(""),
            language: "rust".to_owned(),
            containers: Vec::new(),
            processes: Vec::new(),
        }
    }

    pub fn language(mut self, language: &str) -> Self {
        self.language = language.to_owned();
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = description.into();
        self
    }


    pub fn add_container(mut self, container: ContainerManifest) -> Self {
        self.containers.push(container);
        self
    }

    pub fn add_process(mut self, process: ProcessManifest) -> Self {
        self.processes.push(process);
        self
    }
}

impl TryFrom<Value> for ComponentManifest {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let desc = match obj_get_str(&value, "description") {
            Ok(v) => v,
            Err(_) => ""
        };
        let mut p = ComponentManifest::new(obj_get_str(&value, "type_name")?).description(desc);
        match obj_get_str(&value, "language") {
            Ok(lang) => {
                p = p.language(lang);
            },
            Err(_) => {}
        }
        match obj_get_array(&value, "containers") {
            Ok(value_array) => {
                for v in value_array.iter() {
                    p = p.add_container(v.clone().try_into()?);
                }
            }
            Err(_) => {}
        }
        match obj_get_array(&value, "processes") {
            Ok(value_array) => {
                for v in value_array.iter() {
                    p = p.add_process(v.clone().try_into()?);
                }
            }
            Err(_) => {}
        }
        Ok(p)
    }
}

impl Into<Value> for ComponentManifest {
    fn into(self) -> Value {
        todo!()
    }
}