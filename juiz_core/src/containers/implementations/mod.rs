

mod container_factory_wrapper;
mod container_process_impl;
mod container_process_factory_wrapper;
mod container_process_ptr;
mod container_proxy;
mod container_factory_impl;
mod container_stack_factory;
mod container_process_factory_impl;


use std::sync::Arc;

use container_factory_impl::{ContainerConstructor, ContainerFactoryImpl};
pub use juiz_sdk::containers::ContainerImpl;
pub use container_factory_wrapper::ContainerFactoryWrapper;
pub use container_process_impl::ContainerProcessImpl;
pub use container_process_factory_impl::ContainerProcessFactoryImpl;
pub use container_process_factory_wrapper::ContainerProcessFactoryWrapper;
pub use container_proxy::ContainerProxy;


use juiz_sdk::prelude::*;
pub use container_process_factory_impl::bind_container_function;

use super::{ContainerFactoryPtr, ContainerProcessFactoryPtr};
pub use container_process_factory_impl::BindedContainerFunctionType;

pub fn container_factory_create(manifest: ContainerManifest, constructor: Arc<ContainerConstructor>) -> JuizResult<ContainerFactoryPtr> {
    Ok(ContainerFactoryPtr::new(ContainerFactoryImpl::new(manifest, constructor)?))
}

pub fn container_process_factory_create(manifest: ProcessManifest, constructor: Arc<dyn Fn(ContainerPtr, CapsuleMap)->JuizResult<Capsule>+'static>) -> JuizResult<ContainerProcessFactoryPtr> {
    Ok(ContainerProcessFactoryPtr::new(ContainerProcessFactoryImpl::new_t(manifest, constructor)?))
}
