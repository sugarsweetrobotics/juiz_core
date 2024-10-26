
use crate::prelude::*;


#[derive(Debug, Clone)]
pub struct Description {
    pub text: String
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