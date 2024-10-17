

use example_container_stack::{ExampleContainer, ExampleContainerStack};
use juiz_core::{env_logger, prelude::*};


fn get_function(container: &mut ContainerImpl<ExampleContainerStack>, _v: CapsuleMap) -> JuizResult<Capsule> {
    let value = container.container.lock()?.downcast_ref::<ContainerImpl<ExampleContainer>>().unwrap().value;
    return Ok(jvalue!({
        "name": container.name,
        "value": value
    }).into());
}


#[no_mangle]
pub unsafe extern "Rust" fn container_process_factory() -> JuizResult<ContainerProcessFactoryPtr> {
    env_logger::init();
    let manifest = ContainerProcessManifest::new(ExampleContainerStack::manifest(), "example_container_stack_get")
        .description("Example(get)")
        .into();
    ContainerProcessFactoryImpl::create(manifest, &get_function)
}
