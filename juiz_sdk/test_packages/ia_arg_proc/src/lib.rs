use juiz_sdk::prelude::*;

#[juiz_process]
fn _arg_proc(arg1: Vec<i64>) -> JuizResult<Capsule> {
    log::trace!("process({arg1:?}) called");
    return Ok(jvalue!("OK!").into());
}