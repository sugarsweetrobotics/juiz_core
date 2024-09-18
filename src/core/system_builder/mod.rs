mod setup_plugins;
mod setup_objects;
mod cleanup_objects;
mod containers;
mod processes;
mod brokers;
mod ecs;
mod components;
mod connections;
mod subsystems;

mod http_broker;
mod ipc_broker;
mod local_broker;

pub(crate) use setup_plugins::setup_plugins;
pub(crate) use setup_objects::setup_objects;
pub(crate) use cleanup_objects::cleanup_objects;




