
use std::{cell::RefCell, os::unix::process, rc::Rc};

use anyhow::Context;

use crate::prelude::*;
use super::manifest_description::Description;


#[derive(Debug, Clone)]
pub struct ContainerManifest {
    pub name: Option<String>,
    pub language: String,
    pub type_name: String,
    pub factory: String, 
    pub description: Description,
    pub parent_type_name: Option<String>,
    pub parent_name: Option<String>,
    pub processes: Vec<ProcessManifest>,
    pub args: Value,
}


impl ContainerManifest {

    pub fn build_instance_manifest(&self, mut partial_instance_manifest: ContainerManifest) -> JuizResult<Self> {
        partial_instance_manifest.type_name = self.type_name.clone();
        Ok(partial_instance_manifest
            .description(self.description.as_str())
        )
    }

    pub fn new(type_name: &str) -> Self {
        Self {
            name: None,
            language: "rust".to_owned(),
            type_name: type_name.to_owned(),
            factory: "component_factory".to_owned(),
            description: "".into(), 
            parent_type_name: None,
            parent_name: None,
            processes: Vec::new(),
            args: jvalue!({}),
        }
    }

    pub fn parent_container_manifest(&self) -> Self {
        Self {
            name: self.parent_name.clone(),
            language: "rust".to_owned(),
            type_name: self.parent_type_name.as_ref().unwrap().clone(),
            factory: "component_factory".to_owned(),
            description: "".into(),
            parent_type_name: None,
            parent_name: None,
            processes: Vec::new(),
            args: jvalue!({}),
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }

    pub fn language(mut self, language: &str) -> Self {
        self.language = language.to_owned();
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = description.into();
        self
    }

    pub fn parent_type_name(mut self, parent_type_name: &str) -> Self {
        self.parent_type_name = Some(parent_type_name.to_owned());
        self
    }

    pub fn parent_name(mut self, parent_name: &str) -> Self {
        self.parent_name = Some(parent_name.to_owned());
        self
    }

    pub fn add_process(mut self, process_manifest: ProcessManifest) -> Self {
       
        self.processes.push( 
            process_manifest
                .container_name(self.name.as_ref().map(|s|{s.clone()}))
                .container_type(Some(self.type_name.clone()))
        );
        self
    }

    pub fn factory(mut self, factory: &str) -> Self {
        self.factory = factory.to_owned();
        self
    }

}

impl TryFrom<Value> for ContainerManifest {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let desc = match obj_get_str(&value, "description") {
            Ok(v) => v,
            Err(_) => ""
        };
        let mut p = ContainerManifest::new(obj_get_str(&value, "type_name")?)
            .description(desc);
        match obj_get_str(&value, "name") {
            Ok(name) => {
                p = p.name(name);
            },
            Err(_) => {}
        }
        match obj_get_str(&value, "language") {
            Ok(lang) => {
                p = p.language(lang);
            },
            Err(_) => {}
        }
        match obj_get_str(&value, "parent_name") {
            Ok(name) => {
                p = p.parent_name(name);
            },
            Err(_) => {}
        }
        match obj_get_str(&value, "parent_type_name") {
            Ok(name) => {
                p = p.parent_type_name(name);
            },
            Err(_) => {}
        }
        match obj_get_array(&value, "processes") {
            Ok(process_manifest_values) => {
                for process_manifest_value in process_manifest_values.iter() {
                    let pp :ProcessManifest = process_manifest_value.clone().try_into().context("in loading ContainerManifest from Value")?;
                    p = p.add_process(pp);
                }
            },
            Err(_) => {}
        }

        Ok(p)
    }
}


impl Into<Value> for ContainerManifest {
    fn into(self) -> Value {
        let mut v = jvalue!({
            "type_name": self.type_name,
            "language": self.language,
            "description": self.description.to_str(),
        });
        let obj = v.as_object_mut().unwrap();
        if let Some(name) = self.name {
            obj.insert("name".to_owned(), name.into());
        }
        if let Some(parent_type_name) = self.parent_type_name {
            obj.insert("parent_type_name".to_owned(), parent_type_name.into());
        }
        if let Some(parent_name) = self.parent_name {
            obj.insert("parent_name".to_owned(), parent_name.into());
        }


        v   
    }
}
