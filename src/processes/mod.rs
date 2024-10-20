
mod process;
mod process_ptr;
mod process_factory;
mod implementations;

pub use process::{Process, FunctionTrait,  FunctionType};
pub use process_ptr::ProcessPtr;
pub use process_factory::{ProcessFactory, ProcessFactoryPtr};
pub(crate) use implementations::{
    process_from_clousure_new_with_class_name,
    ProcessFactoryWrapper,
};

pub use implementations::{
    ProcessProxy,
    process_new,
};

use crate::{plugin::ProcessFactoryImpl, prelude::*};


pub fn process_factory_create(manifest: Value, function: FunctionType) -> JuizResult<ProcessFactoryPtr> {
    Ok(ProcessFactoryPtr::new(ProcessFactoryImpl::new(manifest, function)?))
}