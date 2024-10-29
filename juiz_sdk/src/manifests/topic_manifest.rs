use crate::prelude::*;

use anyhow::anyhow;


#[derive(Clone, Debug)]
pub struct TopicManifest {
    pub name: String
}

impl TopicManifest {
    pub fn new(name: &str) -> Self {
        TopicManifest{name: name.to_owned()}
    }
}

impl TryFrom<Value> for TopicManifest {
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value.as_str() {
            Some(v_str) => {
                Ok(TopicManifest{name: v_str.to_owned()})
            },
            None => Err(anyhow!(JuizError::TopicManifestInvalidError{message: "Topic manifest can not convert to Value.".to_owned()})),
        }
    }
    
    type Error = anyhow::Error;
}

impl Into<Value> for TopicManifest {
    fn into(self) -> Value {
        self.name.into()
    }
}