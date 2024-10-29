use super::{ConnectionFactory, DestinationConnectionImpl, SourceConnectionImpl};
use crate::prelude::*;




pub struct ConnectionFactoryImpl {

}


impl ConnectionFactoryImpl {

    pub fn new() -> Self {
        Self{}
    }
}

impl ConnectionFactory for ConnectionFactoryImpl {


    fn create_source_connection(&self, owner_identifier: Identifier, source_process: ProcessPtr, manifest: Value, arg_name: String) -> JuizResult<Box<dyn SourceConnection + 'static>> {
        Ok(Box::new(SourceConnectionImpl::new(owner_identifier, source_process, manifest, arg_name)?))
    }

    fn create_destination_connection(&self, owner_identifier: &Identifier, destination_process_id: &Identifier, dest_process: ProcessPtr, connection_manifest: Value, arg_name: String) -> JuizResult<Box<dyn DestinationConnection + 'static>> {
        Ok(Box::new(DestinationConnectionImpl::new(owner_identifier, destination_process_id, dest_process, connection_manifest, arg_name)?))
    }
}