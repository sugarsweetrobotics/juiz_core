
use juiz_core::prelude::*;


pub unsafe extern "Rust" fn manifest() -> Value { 
    ProcessManifest::new("decrement_process")
        .description("Example(decremnet_process)")
        .add_int_arg("arg1", "The output will be 'arg1 - 1'.", 1)
        .into()
}

fn decrement_process(args: CapsuleMap) -> JuizResult<Capsule> {
    log::trace!("increment_process({:?}) called", args);
    let i = args.get_int("arg1")?;
    return Ok(jvalue!(i-1).into());
}

#[no_mangle]
pub unsafe extern "Rust" fn process_factory() -> JuizResult<ProcessFactoryPtr> {
    env_logger::init();
    ProcessFactoryImpl::create(manifest(), decrement_process)
}
