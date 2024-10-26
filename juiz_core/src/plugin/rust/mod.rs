mod rust_plugin;
mod container_factory_impl;
mod container_process_factory_impl;
mod container_stack_factory;

use container_stack_factory::ContainerStackConstructFunction;
pub(crate) use rust_plugin::RustPlugin;
pub use container_factory_impl::ContainerFactoryImpl;
pub use container_stack_factory::ContainerStackFactoryImpl;
pub use container_process_factory_impl::{ContainerProcessFactoryImpl, bind_container_function, BindedContainerFunctionType};

use crate::{prelude::*, prelude::ContainerManifest};


pub fn container_stack_factory_create<S: 'static>(manifest: ContainerManifest, constructor: ContainerStackConstructFunction<S>) -> JuizResult<ContainerFactoryPtr> {
    Ok(ContainerFactoryPtr::new(ContainerStackFactoryImpl::new(manifest, constructor)?))
}