
mod process;
mod process_ptr;
mod process_factory;
mod implementations;

pub use process::{Process, ProcessBodyFunctionTrait,  ProcessBodyFunctionType};
pub use process_ptr::ProcessPtr;
pub use process_factory::{ProcessFactory, ProcessFactoryPtr};
pub(crate) use implementations::{
    process_from_clousure_new_with_class_name,
    process_from_clousure,
    ProcessFactoryWrapper,
    ProcessFactoryImpl,
};

pub use implementations::{
    ProcessProxy,
    process_new,
};

use crate::prelude::*;


pub fn process_factory_create(manifest: ProcessManifest, function: ProcessBodyFunctionType) -> JuizResult<ProcessFactoryPtr> {
    Ok(ProcessFactoryPtr::new(ProcessFactoryImpl::new(manifest, function)?))
}

pub fn process_factory_create_from_trait(manifest: ProcessManifest, function: impl Fn(CapsuleMap) -> JuizResult<Capsule> +'static ) -> JuizResult<ProcessFactoryPtr> {
    Ok(ProcessFactoryPtr::new(ProcessFactoryImpl::new_from_clousure(manifest, function)?))
}