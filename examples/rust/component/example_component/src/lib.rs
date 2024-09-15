
pub mod example_component {
    use juiz_core::env_logger;
    use juiz_core::prelude::*;

    #[no_mangle]
    pub unsafe extern "Rust" fn component_profile() -> Value {
        env_logger::init();
        return jvalue!({
            "type_name": "example_component",
            "containers": [
                {
                    "type_name": "example_component_container",
                    "factory": "example_component_container_factory",
                    "processes": [ 
                        {
                            "type_name": "example_component_container_get",
                            "factory": "example_component_container_get_factory",
                        },
                        {
                            "type_name": "example_component_container_increment",
                            "factory": "example_component_container_increment_factory"
                        }
                    ]
                }
            ]
        }); 
    }

    #[repr(Rust)]
    pub struct ExampleComponentContainer {
        pub value: i64
    }

    impl ExampleComponentContainer {

        pub fn manifest() -> Value {
            ContainerManifest::new("example_component_container").into()
        }
    }

    fn create_example_component_container(_manifest: Value) -> JuizResult<Box<ExampleComponentContainer>> {
        Ok(Box::new(ExampleComponentContainer{value: 0}))
    }

    #[no_mangle]
    pub unsafe extern "Rust" fn example_component_container_factory() -> JuizResult<ContainerFactoryPtr> {
        ContainerFactoryImpl::create(ExampleComponentContainer::manifest(), create_example_component_container)
    }


    fn example_component_container_get_function(container: &mut ContainerImpl<ExampleComponentContainer>, _v: CapsuleMap) -> JuizResult<Capsule> {
        Ok(jvalue!(container.value).into())
    }
    
    #[no_mangle]
    pub unsafe extern "Rust" fn example_component_container_get_factory() -> JuizResult<ContainerProcessFactoryPtr> {
        ContainerProcessFactoryImpl::create(
            ContainerProcessManifest::new(ExampleComponentContainer::manifest(), "example_component_container_get").into(),
            &example_component_container_get_function)
    }
    

    fn example_component_container_increment_function(container: &mut ContainerImpl<ExampleComponentContainer>, _v: CapsuleMap) -> JuizResult<Capsule> {
        container.value = container.value + 1;
        Ok(jvalue!(container.value).into())
    }
    
    #[no_mangle]
    pub unsafe extern "Rust" fn example_component_container_increment_factory() -> JuizResult<ContainerProcessFactoryPtr> {
        ContainerProcessFactoryImpl::create(
            ContainerProcessManifest::new(ExampleComponentContainer::manifest(), "example_component_container_increment").into(),
            &example_component_container_increment_function)
    }

    fn example_component_container_add_function(container: &mut ContainerImpl<ExampleComponentContainer>, v: CapsuleMap) -> JuizResult<Capsule> {
        let iv = v.get_int("arg1")?;
        container.value = container.value + iv;
        Ok(jvalue!(container.value).into())
    }
    
    #[no_mangle]
    pub unsafe extern "Rust" fn example_component_container_add_factory() -> JuizResult<ContainerProcessFactoryPtr> {
        ContainerProcessFactoryImpl::create(
            ContainerProcessManifest::new(
                ExampleComponentContainer::manifest(), 
                "example_component_container_increment")
                .add_int_arg("arg1", "This value waill be added to value", 1)
                .into(),
            &example_component_container_add_function)
    }

}