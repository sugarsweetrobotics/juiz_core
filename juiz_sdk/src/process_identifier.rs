use std::fmt::Display;

use anyhow::anyhow;

use crate::result::JuizError;

#[derive(Debug)]
pub struct ProcessIdentifier {
    pub broker_name: String, 
    pub broker_type_name: String,
    pub name: String,
    pub type_name: String,
    pub class_name: String,
    pub container_name: Option<String>,
}

impl ProcessIdentifier {

    pub fn new_process_id(broker_name: String, broker_type_name: String, type_name: String, name: String) -> Self {
        Self {
            class_name: "process".to_owned(),
            broker_name,
            broker_type_name,
            name,
            type_name,
            container_name: None,
        }
    }

    pub fn new_container_process_id(broker_name: String, broker_type_name: String, type_name: String, container_name: String, name: String) -> Self {
        Self {
            class_name: "container_process".to_owned(),
            broker_name,
            broker_type_name,
            name,
            type_name: format!("{}:{}", type_name, container_name),
            container_name: Some(container_name)
        }
    }
}

impl Display for ProcessIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("ProcessIdentifier({}://{}/{}/{}::{}", self.broker_type_name, self.broker_name, self.class_name, self.name, self.type_name))
    }
}

impl TryFrom<String> for ProcessIdentifier {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match regex::Regex::new(r"^(.+?)://(.+?)/(.+?)/(.+?)::(.+?)$") {
            Ok(re) => {
                match re.captures(value.as_str()) {
                    Some(caps) => {
                        let class_name = caps[3].to_owned();
                        let type_name = caps[5].to_owned();
                        let name = caps[4].to_owned();
                        let broker_name = caps[2].to_owned();
                        let broker_type_name = caps[1].to_owned();
                        match class_name.as_str() {
                            "process" => {
                                Ok(Self::new_process_id(broker_name, broker_type_name, type_name, name))
                            },
                            "container_process" => {
                                let tokens = type_name.split(":").collect::<Vec<&str>>();
                                if tokens.len() != 2 {

                                }
                                let container_name = tokens[1].to_owned();
                                let container_process_type_name = tokens[0].to_owned();
                                Ok(Self::new_container_process_id(broker_name, broker_type_name, container_process_type_name, container_name, name))
                            },
                            _ => {
                                Err(anyhow!(JuizError::InvalidIdentifierError { message: format!("Invalid Class Name {class_name}") }))
                            }
                        }
                    },
                    None => {
                        log::error!("TryFrom<String> for ProcessIdentifier error. Invalid Identifier ({value}).");
                        return Err(anyhow!(JuizError::InvalidIdentifierError{message: value}))
                    },
                }
            }
            Err(e) => Err(anyhow!(e))
        }    
    }
}

impl Into<String> for ProcessIdentifier {
    fn into(self) -> String {
        format!("{}://{}/{}/{}::{}", self.broker_type_name, self.broker_name, self.class_name, self.name, self.type_name)
    }
}