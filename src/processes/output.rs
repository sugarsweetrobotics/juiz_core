use crate::Value;




pub struct Output {
    pub value: Value,
}

impl Output {

    pub fn new(value: Value) -> Output {

        Output {
            value
        }
    }
}