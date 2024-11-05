use juiz_sdk::prelude::*;

#[juiz_process]
fn talker() -> JuizResult<Capsule> {
    log::trace!("talker() called");
    let string_value = "Hello World";
    println!("talker: {:}", string_value);
    return Ok(jvalue!(string_value).into());
}