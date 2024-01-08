
pub mod process;
pub mod process_proxy;
pub mod process_impl;
pub mod process_factory;
pub mod process_factory_impl;
pub mod process_factory_wrapper;
pub mod argument;
pub mod output;

pub mod inlet;
pub mod outlet;

pub use process::{Process, ProcessFunction};
pub use process_factory::ProcessFactory;
pub use process_factory_impl::create_process_factory;
pub use process_factory_wrapper::ProcessFactoryWrapper;

use crate::{Value, JuizResult, JuizError};

pub use self::argument::Argument;
pub use self::output::Output;


pub fn arg<'t>(args: &'t Vec<Argument>, name: &str) -> JuizResult<&'t Value> {
    for a in args.iter() {
        if a.name == name {
            return Ok(&a.value);
        }
    }
    Err(anyhow::Error::from(JuizError::ArgumentCanNotFoundByNameError{name: name.to_owned()}))
}