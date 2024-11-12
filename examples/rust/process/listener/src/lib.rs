
use juiz_sdk::prelude::*;

#[juiz_process(
    arguments = {
        default = {
            arg1 = "Hello, Juiz!"
        }
        arg1 = {
            default = "Hello, Juiz!"
        }
    }
)]
fn listener(arg1: String) -> JuizResult<Capsule> {
    log::trace!("listener({:}) called", arg1);
    println!("listener: {:}", arg1);
    return Ok(jvalue!("Hello World").into());
}