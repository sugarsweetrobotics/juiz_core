
mod process;
mod process_factory;
mod implementations;

pub use process::{Process, ProcessPtr, FunctionTrait,  FunctionType};
pub use process_factory::{ProcessFactory, ProcessFactoryPtr};
pub use implementations::{
    process_from_clousure_new_with_class_name,
    process_new,
    ProcessProxy,
    ProcessFactoryWrapper,
};