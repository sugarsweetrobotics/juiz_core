

pub mod example_container {

    use std::sync::{Mutex, Arc};

    use juiz_core::{jvalue, JuizResult, Value, ContainerFactory, create_container_factory};
    
    
    #[no_mangle]
    pub unsafe extern "Rust" fn manifest() -> Value { 
    
        return jvalue!({
            "type_name": "example_container",
        }); 
    }


    #[allow(dead_code)]
    #[repr(Rust)]
    pub struct ExampleContainer {
        pub value: i64
    }

    pub fn create_example_container(_manifest: Value) -> JuizResult<Box<ExampleContainer>> {
        println!("create_example_container({})", _manifest);
        Ok(Box::new(ExampleContainer{value: 0}))
    }


    #[no_mangle]
    pub unsafe extern "Rust" fn container_factory() -> JuizResult<Arc<Mutex<dyn ContainerFactory>>> {
        env_logger::init();
        create_container_factory(manifest(), create_example_container)
    }

}
