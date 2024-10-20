

use example_container::ExampleContainer;
use juiz_core::{env_logger, prelude::*};


fn manifest() -> Value { 
    ContainerProcessManifest::new(ExampleContainer::manifest(), "example_container_get")
        .description("Example(get)")
        .into()
}

fn get_function(container: &mut ContainerImpl<ExampleContainer>, _v: CapsuleMap) -> JuizResult<Capsule> {
    return Ok(jvalue!(container.value).into());
}


#[no_mangle]
pub unsafe extern "Rust" fn container_process_factory() -> JuizResult<ContainerProcessFactoryPtr> {
    env_logger::init();
    container_process_factory_create(manifest(), &get_function)
}
