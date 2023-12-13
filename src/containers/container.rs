use std::fmt::Display;

use mopa::mopafy;

use crate::{Value, JuizObject};

pub trait Container : Display + mopa::Any + JuizObject{
    
    fn manifest(&self) -> &Value;
}

mopafy!(Container);

