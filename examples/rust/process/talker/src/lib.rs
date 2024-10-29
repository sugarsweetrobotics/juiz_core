
use juiz_sdk::{env_logger, prelude::*};


pub unsafe extern "Rust" fn manifest() -> ProcessManifest { 
    ProcessManifest::new("talker")
        .description("Example(talker)")
}

fn talker(args: CapsuleMap) -> JuizResult<Capsule> {
    log::trace!("talker({:?}) called", args);
    let string_value = "Hello World";
    println!("talker: {:}", string_value);
    return Ok(jvalue!(string_value).into());
}

#[no_mangle]
pub unsafe extern "Rust" fn process_factory() -> JuizResult<ProcessFactoryStruct> {
    env_logger::init();
    Ok(juiz_sdk::process_factory(manifest(), talker))
}
