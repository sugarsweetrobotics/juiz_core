

use example_container::ExampleContainer;
use juiz_base::{env_logger, prelude::*};


fn manifest() -> ProcessManifest { 
    ProcessManifest::new("example_container_get")
        .description("Example(get)")
        .container(ExampleContainer::manifest())
}

fn get_function(container: &mut ContainerImpl<ExampleContainer>, _v: CapsuleMap) -> JuizResult<Capsule> {
    return Ok(jvalue!(container.value).into());
}


#[no_mangle]
pub unsafe extern "Rust" fn container_process_factory() -> JuizResult<ContainerProcessFactoryStruct> {
    env_logger::init();
    container_process_factory_create(manifest(), &get_function)
}
