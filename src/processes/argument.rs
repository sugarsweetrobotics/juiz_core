use crate::{jvalue, Value};




pub struct Argument {
    pub name: String,
    pub value: Value,
}


impl Argument {

    pub fn new(name: &str, value: Value) -> Self {
        Argument {
            name: name.to_owned(),
            value,
        }
    }
}