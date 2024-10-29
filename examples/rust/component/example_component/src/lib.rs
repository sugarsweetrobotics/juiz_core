
pub mod example_component {
    use juiz_base::env_logger;
    use juiz_base::prelude::*;

    #[no_mangle]
    pub unsafe extern "Rust" fn component_manifest() -> ComponentManifest {
        env_logger::init();
        ComponentManifest::new("example_component")
          .add_container(ContainerManifest::new("example_component_container")
            .factory("example_component_container_factory")
            .add_process(ProcessManifest::new("example_component_container_get")
              .factory("example_component_container_get_factory"))
            .add_process(ProcessManifest::new("example_component_container_increment")
              .factory("example_component_container_increment_factory"))
          ).add_process(ProcessManifest::new("increment_process")
            .factory("increment_process_factory"))
    }

    #[repr(Rust)]
    pub struct ExampleComponentContainer {
        pub value: i64
    }

    impl ExampleComponentContainer {

        pub fn manifest() -> ContainerManifest {
            ContainerManifest::new("example_component_container")
        }
    }

    fn create_example_component_container(_manifest: ContainerManifest) -> JuizResult<Box<ExampleComponentContainer>> {
        Ok(Box::new(ExampleComponentContainer{value: 0}))
    }

    #[no_mangle]
    pub unsafe extern "Rust" fn example_component_container_factory() -> JuizResult<ContainerFactoryStruct> {
        Ok(juiz_base::container_factory(ExampleComponentContainer::manifest(), create_example_component_container))
    }

    fn increment_process(args: CapsuleMap) -> JuizResult<Capsule> {
        log::trace!("increment_process({:?}) called", args);
        let i = args.get_int("arg1")?;
        return Ok(jvalue!(i+1).into());
    }
    
    #[no_mangle]
    pub unsafe extern "Rust" fn increment_process_factory() -> JuizResult<ProcessFactoryStruct> {
        // env_logger::init();
        let manif = ProcessManifest::new("increment_process")
            .description("Example(incremnet_process)")
            .add_int_arg("arg1", "The output will be 'arg1 + 1'.", 1)
            .into();
        Ok(juiz_base::process_factory(manif, increment_process))
    }
    

    fn example_component_container_get_function(container: &mut ContainerImpl<ExampleComponentContainer>, _v: CapsuleMap) -> JuizResult<Capsule> {
        Ok(jvalue!(container.value).into())
    }
    
    #[no_mangle]
    pub unsafe extern "Rust" fn example_component_container_get_factory() -> JuizResult<ContainerProcessFactoryStruct> {
        Ok(juiz_base::container_process_factory(
            ProcessManifest::new("example_component_container_get").container(ExampleComponentContainer::manifest()).into(),
            &example_component_container_get_function))
    }
    

    fn example_component_container_increment_function(container: &mut ContainerImpl<ExampleComponentContainer>, _v: CapsuleMap) -> JuizResult<Capsule> {
        container.value = container.value + 1;
        Ok(jvalue!(container.value).into())
    }
    
    #[no_mangle]
    pub unsafe extern "Rust" fn example_component_container_increment_factory() -> JuizResult<ContainerProcessFactoryStruct> {
        Ok(juiz_base::container_process_factory(
            ProcessManifest::new("example_component_container_increment").container(ExampleComponentContainer::manifest()),
            &example_component_container_increment_function))
    }

    fn example_component_container_add_function(container: &mut ContainerImpl<ExampleComponentContainer>, v: CapsuleMap) -> JuizResult<Capsule> {
        let iv = v.get_int("arg1")?;
        container.value = container.value + iv;
        Ok(jvalue!(container.value).into())
    }
    
    #[no_mangle]
    pub unsafe extern "Rust" fn example_component_container_add_factory() -> JuizResult<ContainerProcessFactoryStruct> {
        Ok(juiz_base::container_process_factory(
            ProcessManifest::new(
                "example_component_container_increment")
                .add_int_arg("arg1", "This value waill be added to value", 1)
                .container(ExampleComponentContainer::manifest()),
            &example_component_container_add_function))
    }

}