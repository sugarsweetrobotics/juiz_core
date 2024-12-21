use std::fmt::Display;

use crate::{connections::destination_connection, prelude::{Connection, Identifier}, result::JuizError};



pub struct ConnectionIdentifier {
    pub source_identifier: Identifier,
    pub destination_identifier: Identifier,
    pub arg_name: String,
}

impl Display for ConnectionIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("ConnectionID(src={}, arg={}, dst={})", self.source_identifier, self.arg_name, self.destination_identifier))
    }
}

impl ConnectionIdentifier {
    pub fn new(source_identifier: Identifier, arg_name: &str, destination_identifier: Identifier) -> Self {
        Self {
            source_identifier,
            arg_name: arg_name.to_owned(),
            destination_identifier,
        }
    }
}

impl TryFrom<String> for ConnectionIdentifier {
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let tokens = value[13..].split('|').collect::<Vec<&str>>();
        if tokens.len() != 3 {
            log::error!("TryFrom<String>(for ConnectionIdentifier)::try_from({value}) failed. The number of split tokens by '|' is not three but {}", tokens.len());
            return Err(anyhow::Error::from(JuizError::InvalidConnectionIdentifierError{identifier: value}));
        }
        Ok(ConnectionIdentifier{
            source_identifier: tokens[0].to_owned(),
            arg_name: tokens[1].to_owned(),
            destination_identifier: tokens[2].to_owned(),
        })
    }
    type Error = anyhow::Error;
}

impl Into<String> for ConnectionIdentifier {
    fn into(self) -> String {
        format!("connection://{}|{}|{}", self.source_identifier, self.arg_name, self.destination_identifier)
    }
}