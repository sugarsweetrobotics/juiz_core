
use juiz_sdk::{env_logger, prelude::*};


pub unsafe extern "Rust" fn manifest() -> ProcessManifest { 
    ProcessManifest::new("listener")
        .description("Example(listener)")
        .add_string_arg("arg1", "listener input string", "")
}

fn listener(args: CapsuleMap) -> JuizResult<Capsule> {
    log::trace!("listener({:?}) called", args);
    let string_value = args.get("arg1")?.lock_as_str(|string_value| {
        println!("listener: {:}", string_value);
    });
    return Ok(jvalue!("Hello World").into());
}

#[no_mangle]
pub unsafe extern "Rust" fn process_factory() -> JuizResult<ProcessFactoryStruct> {
    env_logger::init();
    Ok(juiz_sdk::process_factory(manifest(), listener))
}
