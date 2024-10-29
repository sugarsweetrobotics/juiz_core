pub mod connection;

pub mod destination_connection;
pub mod source_connection;

pub use connection::{Connection,  ConnectionType, ConnectionCore};
pub use source_connection::SourceConnection;
pub use destination_connection::DestinationConnection;