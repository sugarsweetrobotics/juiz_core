
use std::fmt::Display;

use crate::prelude::*;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ArgumentType {
    Bool, 
    Int,
    Float,
    String,
    Array,
    Object,
    Image,
}

impl ArgumentType {

    pub fn as_str(&self) -> &'static str {
        match self {
            ArgumentType::Bool => "bool", 
            ArgumentType::Int => "int",
            ArgumentType::Float => "float",
            ArgumentType::String => "string",
            ArgumentType::Array => "array",
            ArgumentType::Object => "object",
            ArgumentType::Image => "image",
        }
    }
}

impl Display for ArgumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl TryFrom<&str> for ArgumentType {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "bool" => Ok(ArgumentType::Bool),
            "int" => Ok(ArgumentType::Int),
            "float" => Ok(ArgumentType::Float),
            "string" => Ok(ArgumentType::String),
            "array" => Ok(ArgumentType::Array),
            "object" => Ok(ArgumentType::Object),
            "image" => Ok(ArgumentType::Image),
            _ => Err(anyhow!(JuizError::ProcessManifestInvalidError{message: "Argument type is invalid in ArgumentManifest in ProcessManifest.".to_owned()}))
        }
    }
}


pub(super) fn type_check(arg_type: &ArgumentType, value: &Value) -> JuizResult<()> {
    fn ret_err() -> JuizResult<()> {
        Err(anyhow!(JuizError::ArguemntTypeIsInvalidError {}))
    }
    // println!("type_check({arg_type:?}, {value:?}) called");
    match arg_type {
        ArgumentType::Bool => if value.is_boolean() { Ok(()) } else {ret_err() },
        ArgumentType::Int => if value.is_i64() {Ok(())} else { ret_err() },
        ArgumentType::Float => if value.is_f64() {Ok(())} else { ret_err() },
        ArgumentType::String => if value.is_string() {Ok(())} else { ret_err() },
        ArgumentType::Array => if value.is_array() {Ok(())} else { ret_err() },
        ArgumentType::Object => if value.is_object() {Ok(())} else { ret_err() },
        ArgumentType::Image => if value.is_null() || value.is_object() {Ok(())} else { ret_err() },
    }
}