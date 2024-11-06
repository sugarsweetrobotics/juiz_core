use example_container::ExampleContainer;
use juiz_sdk::prelude::*;

#[juiz_container_process(
    container_type = "example_container"
)]
fn increment_function(container: &mut ContainerImpl<ExampleContainer>, arg1: i64) -> JuizResult<Capsule> {
    container.value = container.value + arg1;
    return Ok(jvalue!(container.value).into());
}
