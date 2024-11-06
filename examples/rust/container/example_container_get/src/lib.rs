
use juiz_sdk::prelude::*;
use example_container::ExampleContainer;

#[juiz_container_process(container_type = "example_container")]
fn example_container_get(container: &mut ContainerImpl<ExampleContainer>) -> JuizResult<Capsule> {
    return Ok(jvalue!(container.value).into());
}

