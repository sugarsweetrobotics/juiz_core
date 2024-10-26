use example_container::ExampleContainer;
use juiz_core::{env_logger, prelude::*};

fn manifest() -> ProcessManifest { 
    ProcessManifest::new("example_container_increment")
        .description("Example(get)")
        .add_int_arg("arg1", "test_argument", 1)
        .container(ExampleContainer::manifest())
}

fn increment_function(container: &mut ContainerImpl<ExampleContainer>, v: CapsuleMap) -> JuizResult<Capsule> {
    let i = v.get_int("arg1")?;
    container.value = container.value + i;
    return Ok(jvalue!(container.value).into());
}

#[no_mangle]
pub unsafe extern "Rust" fn container_process_factory() -> JuizResult<ContainerProcessFactoryPtr> {
    env_logger::init();
    container_process_factory_create(manifest(), &increment_function)
}
