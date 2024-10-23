use juiz_core::{env_logger, prelude::*};

#[repr(Rust)]
pub struct ExampleContainer {
    pub value: i64
}

impl ExampleContainer {
    pub fn manifest() -> ContainerManifest {
        ContainerManifest::new("example_container")
    }
}

fn create_example_container(_manifest: ContainerManifest) -> JuizResult<Box<ExampleContainer>> {
    Ok(Box::new(ExampleContainer{value: 0}))
}

#[no_mangle]
pub unsafe extern "Rust" fn container_factory() -> JuizResult<ContainerFactoryPtr> {
    env_logger::init();
    container_factory_create(ExampleContainer::manifest(), create_example_container)
}


