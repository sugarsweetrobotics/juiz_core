
use crate::prelude::*;
use super::manifest_description::Description;
use anyhow::anyhow;


#[derive(Clone, Debug)]
pub enum ArgumentType {
    Int,
    Float,
    String,
    Object,
    Image,
}

impl ArgumentType {

    pub fn as_str(&self) -> &'static str {
        match self {
            ArgumentType::Int => "int",
            ArgumentType::Float => "float",
            ArgumentType::String => "string",
            ArgumentType::Object => "object",
            ArgumentType::Image => "image",
        }
    }
}

impl TryFrom<&str> for ArgumentType {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "int" => Ok(ArgumentType::Int),
            "float" => Ok(ArgumentType::Float),
            "string" => Ok(ArgumentType::String),
            "object" => Ok(ArgumentType::Object),
            "image" => Ok(ArgumentType::Image),
            _ => Err(anyhow!(JuizError::ProcessManifestInvalidError{message: "Argument type is invalid in ArgumentManifest in ProcessManifest.".to_owned()}))
        }
    }
}

#[derive(Clone, Debug)]
pub struct ArgumentManifest {
    pub type_name: ArgumentType,
    pub name: String,
    pub description: Description,
    pub default: Value
}

fn type_check(arg_type: &ArgumentType, value: &Value) -> JuizResult<()> {
    fn ret_err() -> JuizResult<()> {
        Err(anyhow!(JuizError::ArguemntTypeIsInvalidError {}))
    }

    match arg_type {
        ArgumentType::Int => if value.is_i64() {Ok(())} else { ret_err() },
        ArgumentType::Float => if value.is_f64() {Ok(())} else { ret_err() },
        ArgumentType::String => if value.is_string() {Ok(())} else { ret_err() },
        ArgumentType::Object => if value.is_object() {Ok(())} else { ret_err() },
        ArgumentType::Image => if value.is_object() {Ok(())} else { ret_err() },
    }
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

    pub fn new_with_check(type_name: ArgumentType, name: &str, description: Description, default: Value) -> JuizResult<Self> {
        type_check(&type_name, &default)?;
        Ok(ArgumentManifest {
            type_name,
            name: name.to_owned(),
            description,
            default
        })
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = description.into();
        self
    }

    pub fn new_int(name: &str, default: i64) -> Self {
        Self::new(ArgumentType::Int, name, "".into(), default.into())
    }

    pub fn new_float(name: &str, default: f64) -> Self {
        Self::new(ArgumentType::Float, name, "".into(), default.into())
    }

    pub fn new_string(name: &str, default: &str) -> Self {
        Self::new(ArgumentType::String, name, "".into(), default.into())
    }

    pub fn new_object(name: &str, default: Value) -> Self {
        Self::new(ArgumentType::Object, name, "".into(), default)
    }

    pub fn new_image(name: &str) -> Self {
        Self::new(ArgumentType::String, name, "".into(), jvalue!({}))
    }
    
}

impl Into<Value> for ArgumentManifest {
    fn into(self) -> Value {
        jvalue!({
            "name": self.name,
            "type": self.type_name.as_str(),
            "description": self.description.to_str(),
            "default": self.default,
        })
    }
}

/// ```
/// use juiz_core::prelude::*;
/// fn main() -> JuizResult<()> {
/// let arg_value = jvalue!({
///   "type": "int",
///   "name": "arg0",
///   "description": "int arg",
///   "default": 1
/// });
/// let arg: ArgumentManifest = arg_value.try_into()?;
/// 
/// 
/// Ok(())}
/// ```
impl TryFrom<Value> for ArgumentManifest {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let name = obj_get_str(&value, "name")?;
        let description = obj_get_str(&value, "description").or::<JuizError>(Ok("")).unwrap();
        let type_name: ArgumentType = obj_get_str(&value, "type")?.try_into()?;
        let default_value = obj_get(&value, "default")?;
        ArgumentManifest::new_with_check(type_name, name, description.into(), default_value.clone())
    }
}
