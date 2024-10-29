use juiz_sdk::result::JuizResult;
use crate::prelude::*;




pub trait ConnectionFactory {

    fn create_source_connection(&self, owner_identifier: Identifier, source_process: ProcessPtr, manifest: Value, arg_name: String) -> JuizResult<Box<dyn SourceConnection + 'static>>;

    fn create_destination_connection(&self, owner_identifier: &Identifier, destination_process_id: &Identifier, dest_process: ProcessPtr, connection_manifest: Value, arg_name: String) -> JuizResult<Box<dyn DestinationConnection+'static>>;
}