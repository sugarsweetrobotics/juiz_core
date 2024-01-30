use std::io::Bytes;

use crate::{Value, JuizResult, JuizError};
use opencv::core::Mat;
use serde::de::value::BytesDeserializer;


enum OutputCore {
    Value(Value),
    Mat(Mat)
}

pub struct Output {
    core: OutputCore,
}

impl From<Value> for Output {
    fn from(value: Value) -> Self {
        Self::new_from_value( value )
    }
}

impl Output {

    pub fn new_from_value(value: Value) -> Self {
        Self{core: OutputCore::Value(value)}
    }

    pub fn get_value(&self) -> JuizResult<Value> {
        match &self.core {
            OutputCore::Value(v) => return Ok(v.clone()),
            _ => return Err(anyhow::Error::from(JuizError::OutputDoesNotContainValueTypeError{}))
        }
    }

    pub fn set_value(&mut self, value: Value) -> JuizResult<()> {
        self.core = OutputCore::Value(value);
        Ok(())
    }
}