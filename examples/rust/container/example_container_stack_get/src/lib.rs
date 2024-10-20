

use example_container_stack::{ExampleContainer, ExampleContainerStack};
use juiz_core::{env_logger, prelude::*};


fn get_function(container: &mut ContainerImpl<ExampleContainerStack>, _v: CapsuleMap) -> JuizResult<Capsule> {
    return Ok(jvalue!({
        "name": container.name,
        "value": container.container.downcast_and_then(|c: &ContainerImpl<ExampleContainer>|{ c.value })?
    }).into());
}


#[no_mangle]
pub unsafe extern "Rust" fn container_process_factory() -> JuizResult<ContainerProcessFactoryPtr> {
    env_logger::init();
    let manifest = ContainerProcessManifest::new(ExampleContainerStack::manifest(), "example_container_stack_get")
        .description("Example(get)")
        .into();
    container_process_factory_create(manifest, &get_function)
}
