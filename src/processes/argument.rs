
use crate::{Value, processes::Output};




pub struct Argument {
    pub value: Value,
}


impl Argument {
}

impl From<Value> for Argument {
    fn from(value: Value) -> Self {
        Argument {
            value
        }
    }
}


impl From<Output> for Argument {
    fn from(_value: Output) -> Self {
        todo!("ここにOutputからArgumentへの変換を記述")
    }
}