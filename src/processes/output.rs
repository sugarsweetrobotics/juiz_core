

use crate::Value;
use opencv::core::Mat;


#[derive(Clone, Debug)]
pub enum Output {
    Empty(()),
    Value(Value),
    Mat(Mat)
}

/*
pub struct Output {
    core: OutputCore,
    value_buffer: RefCell<OutputCore>,
}



fn mat_to_value(mat: &Mat) -> Value {
    todo!()
}
*/
impl From<Value> for Output {
    fn from(value: Value) -> Self {
        Self::Value( value )
    }
}

impl From<Mat> for Output {
    fn from(value: Mat) -> Self {
        Self::Mat( value )
    }
}


impl Output {

    pub fn is_empty(&self) -> bool {
        match self {
            Output::Empty(_) => return true, 
            _ => return false
        }
    }

    pub fn is_value(&self) -> bool {
        match self {
            Output::Value(_) => return true, 
            _ => return false
        }
    }

    pub fn as_value(&self) -> Option<&Value> {
        match self {
            Output::Value(v) => Some(v),
            _ => None
        }
    }

    pub fn is_mat(&self) -> bool {
        match self {
            Output::Mat(_) => return true, 
            _ => return false
        }
    }

    pub fn as_mat(&self) -> Option<&Mat> {
        match self {
            Output::Mat(v) => Some(v),
            _ => None
        }
    }
}