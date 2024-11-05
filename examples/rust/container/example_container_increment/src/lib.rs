use example_container::ExampleContainer;
use juiz_sdk::prelude::*;
use juiz_sdk::env_logger;

fn manifest() -> ProcessManifest { 
    ProcessManifest::new("example_container_increment")
        .description("Example(get)")
        .add_int_arg("arg1", "test_argument", 5)
        .container(ExampleContainer::manifest())
}

fn increment_function(container: &mut ContainerImpl<ExampleContainer>, v: CapsuleMap) -> JuizResult<Capsule> {
    let i = v.get_int("arg1")?;
    container.value = container.value + i;
    return Ok(jvalue!(container.value).into());
}

#[no_mangle]
pub unsafe extern "Rust" fn container_process_factory() -> JuizResult<ContainerProcessFactoryStruct> {
    env_logger::init();
    Ok(juiz_sdk::container_process_factory(manifest(), increment_function))
}
