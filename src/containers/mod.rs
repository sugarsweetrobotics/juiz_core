
mod container;
mod container_ptr;
mod container_process;

mod container_factory;
mod container_process_factory;
mod implementations;

use crate::{plugin::ContainerProcessFactoryImpl, prelude::*};

pub use container::Container;
pub use container_ptr::ContainerPtr;
pub use container_factory::{ContainerFactory, ContainerFactoryPtr, ContainerConstructFunction};
pub use container_process_factory::{ContainerProcessFactory, ContainerProcessFactoryPtr};

pub(crate) use implementations::{
    ContainerProcessImpl,
    ContainerFunctionType,
    ContainerFunctionTypePtr,
    ContainerFactoryWrapper, 
    ContainerProcessFactoryWrapper
};

pub use implementations::{
    ContainerImpl,
    ContainerProxy,
};

use crate::plugin::ContainerFactoryImpl;

pub fn container_factory_create<S: 'static>(manifest: Value, constructor: ContainerConstructFunction<S>) -> JuizResult<ContainerFactoryPtr> {
    Ok(ContainerFactoryPtr::new(ContainerFactoryImpl::new(manifest, constructor)?))
}

pub fn container_process_factory_create<S: 'static>(manifest: Value, constructor: &'static ContainerFunctionType<S>) -> JuizResult<ContainerProcessFactoryPtr> {
    Ok(ContainerProcessFactoryPtr::new(ContainerProcessFactoryImpl::new(manifest, constructor)?))
}