use juiz_sdk::connections::ConnectionManifest;
use crate::prelude::*;




pub trait ConnectionFactory {

    fn create_source_connection(&self, source_process: ProcessPtr, connection_manifest: ConnectionManifest) -> Box<dyn SourceConnection>;

    fn create_destination_connection(&self, destination_process: ProcessPtr, connection_manifest: ConnectionManifest) -> Box<dyn DestinationConnection>;
}