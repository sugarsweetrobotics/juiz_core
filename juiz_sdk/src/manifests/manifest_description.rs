
use std::fmt::Display;

use crate::prelude::*;


#[derive(Debug, Clone)]
pub struct Description {
    pub text: String
}

impl Display for Description {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Description(\"{}\")", self.text))
    }
}

impl Description {
    pub fn new(text: &str) -> Self {
        Description{
            text: text.to_owned()
        }
    }

    pub fn to_str(self) -> String {
        self.text
    }

    pub fn as_str(&self) -> &str {
        self.text.as_str()
    }
}

impl Into<Value> for Description {
    fn into(self) -> Value {
        jvalue!(self.text)
    }
}

impl From<&str> for Description {
    fn from(value: &str) -> Self {
        Description::new(value)
    }
}