use juiz_sdk::{env_logger, factory::ContainerFactoryStruct, prelude::*};

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
pub unsafe extern "Rust" fn container_factory() -> JuizResult<ContainerFactoryStruct> {
    env_logger::init();
    Ok(juiz_sdk::container_factory(ExampleContainer::manifest(), create_example_container))
}


