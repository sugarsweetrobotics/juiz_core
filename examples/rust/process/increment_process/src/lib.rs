
use juiz_sdk::{env_logger, prelude::*};


pub unsafe extern "Rust" fn manifest() -> ProcessManifest { 
    ProcessManifest::new("increment_process")
        .description("Example(incremnet_process)")
        .add_int_arg("arg1", "The output will be 'arg1 + 1'.", 1)
}

fn increment_process(args: CapsuleMap) -> JuizResult<Capsule> {
    log::trace!("increment_process({:?}) called", args);
    let i = args.get_int("arg1")?;
    return Ok(jvalue!(i+1).into());
}

#[no_mangle]
pub unsafe extern "Rust" fn process_factory() -> JuizResult<ProcessFactoryStruct> {
    env_logger::init();
    Ok(juiz_sdk::process_factory(manifest(), increment_process))
}
