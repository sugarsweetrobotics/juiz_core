


mod plugin;
mod python;
mod cpp;
mod rust;

pub(crate) use rust::ContainerFactoryImpl;

pub use plugin::{Plugin, JuizObjectPlugin, concat_dirname, plugin_name_to_file_name};
pub use rust::{ContainerStackFactoryImpl, ContainerProcessFactoryImpl};
//pub use rust::{ProcessFactoryImpl, ContainerFactoryImpl, ContainerProcessFactoryImpl};
pub(crate) use rust::RustPlugin;
//pub use rust::ContainerProcessConstructorType;

// use crate::{prelude::*, processes::FunctionType};
// pub(crate) fn create_process_factory(manifest: Value, function: FunctionType) -> JuizResult<impl ProcessFactory> {
//     ProcessFactoryImpl::new(manifest, function)
// }

pub use rust::container_stack_factory_create;