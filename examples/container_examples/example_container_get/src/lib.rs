
pub mod example_container_get {

    use std::sync::{Arc, Mutex};

    use example_container::example_container::ExampleContainer;
    use juiz_core::{containers::{container_impl::ContainerImpl, create_container_process_factory}, jvalue, processes::capsule::{Capsule, CapsuleMap}, ContainerProcessFactory, JuizResult, Value};

    
    #[no_mangle]
    pub unsafe extern "Rust" fn _manifest() -> Value { 
        return jvalue!({
            "container_type_name": "example_container",
            "type_name": "example_container_get",
            "arguments" : {
            }, 
        }); 
    }


    fn get_function(container: &mut ContainerImpl<ExampleContainer>, _v: CapsuleMap) -> JuizResult<Capsule> {
        return Ok(jvalue!(container.value).into());
    }


    #[no_mangle]
    pub unsafe extern "Rust" fn container_process_factory() -> JuizResult<Arc<Mutex<dyn ContainerProcessFactory>>> {
        env_logger::init();
        create_container_process_factory::<ExampleContainer>(_manifest(), &get_function )
    }

}