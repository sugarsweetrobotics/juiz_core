use std::sync::{Arc, Mutex};

use juiz_core::{anyhow::anyhow, env_logger, prelude::*};
pub use example_container::ExampleContainer;


#[repr(Rust)]
pub struct ExampleContainerStack {
    pub name: String,
    pub container: ContainerPtr,
}

impl ExampleContainerStack {

    pub fn manifest() -> Value {
        ContainerManifest::new("example_container_stack").parent("example_container").into()
    }

}

fn create_example_container_ex(container_ptr: ContainerPtr, manifest: Value) -> JuizResult<Box<ExampleContainerStack>> {
    let my_name = match manifest.as_object().unwrap().get("name") {
        None => Err(anyhow!(JuizError::InvalidValueError{ message: "Argument profile does not include 'name'.".to_owned()})),
        Some(v) => {
            Ok(v.as_str().unwrap())
        },
    }?;
    Ok(Box::new(ExampleContainerStack{name: my_name.to_owned(), container: container_ptr}))
}

#[no_mangle]
pub unsafe extern "Rust" fn container_factory() -> JuizResult<Arc<Mutex<dyn ContainerFactory>>> {
    env_logger::init();
    let manifest = ExampleContainerStack::manifest();
    ContainerStackFactoryImpl::create(manifest, create_example_container_ex)
}


