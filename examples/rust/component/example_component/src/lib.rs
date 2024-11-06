
use juiz_sdk::prelude::*;


#[juiz_component_process]
fn example_component_increment(arg1: i64) -> JuizResult<Capsule> {
    log::trace!("increment_process({:?}) called", arg1);
    return Ok(jvalue!(arg1+1).into());
}
// #[no_mangle]
// pub unsafe extern "Rust" fn increment_process_factory() -> JuizResult<ProcessFactoryStruct> {
//     // env_logger::init();
//     let manif = ProcessManifest::new("increment_process")
//         .description("Example(incremnet_process)")
//         .add_int_arg("arg1", "The output will be 'arg1 + 1'.", 1)
//         .into();
//     Ok(juiz_sdk::process_factory(manif, increment_process))
// }

// #[repr(Rust)]
// pub struct ExampleComponentContainer {
//     pub value: i64
// }


// #[juiz_component_container]
// fn example_component_container(initial_value: i64) -> JuizResult<Box<ExampleComponentContainer>> {
//     Ok(Box::new(ExampleComponentContainer{value: initial_value}))
// }

// // impl ExampleComponentContainer {
// //     pub fn manifest() -> ContainerManifest {
// //         ContainerManifest::new("example_component_container")
// //     }
// // }
// // #[no_mangle]
// // pub unsafe extern "Rust" fn example_component_container_factory() -> JuizResult<ContainerFactoryStruct> {
// //     Ok(juiz_sdk::container_factory(ExampleComponentContainer::manifest(), create_example_component_container))
// // }


// #[juiz_component_container_process(
//     container_type = "example_component_container"
// )]
// fn example_component_container_get(container: &mut ContainerImpl<ExampleComponentContainer>) -> JuizResult<Capsule> {
//     Ok(jvalue!(container.value).into())
// }
// // #[no_mangle]
// // pub unsafe extern "Rust" fn example_component_container_get_factory() -> JuizResult<ContainerProcessFactoryStruct> {
// //     Ok(juiz_sdk::container_process_factory(
// //         ProcessManifest::new("example_component_container_get").container(ExampleComponentContainer::manifest()).into(),
// //         &example_component_container_get_function))
// // }

// #[juiz_component_container_process(
//     container_type = "example_component_container"
// )]
// fn example_component_container_increment(container: &mut ContainerImpl<ExampleComponentContainer>) -> JuizResult<Capsule> {
//     container.value = container.value + 1;
//     Ok(jvalue!(container.value).into())
// }   
// // #[no_mangle]
// // pub unsafe extern "Rust" fn example_component_container_increment_factory() -> JuizResult<ContainerProcessFactoryStruct> {
// //     Ok(juiz_sdk::container_process_factory(
// //         ProcessManifest::new("example_component_container_increment").container(ExampleComponentContainer::manifest()),
// //         &example_component_container_increment_function))
// // }

// #[juiz_component_container_process(
//     container_type = "example_container"
// )]
// fn example_component_container_add(container: &mut ContainerImpl<ExampleComponentContainer>, arg1: i64) -> JuizResult<Capsule> {
//     container.value = container.value + arg1;
//     Ok(jvalue!(container.value).into())
// }
// // #[no_mangle]
// //     pub unsafe extern "Rust" fn example_component_container_add_factory() -> JuizResult<ContainerProcessFactoryStruct> {
// //         Ok(juiz_sdk::container_process_factory(
// //             ProcessManifest::new(
// //                 "example_component_container_increment")
// //                 .add_int_arg("arg1", "This value waill be added to value", 1)
// //                 .container(ExampleComponentContainer::manifest()),
// //             &example_component_container_add_function))
// //     }

juiz_component_manifest!(
    container_name = "example_component"
    // containers = [
    //     example_component_container
    // ]
    // container_processes = [
    //     example_component_container_get,
    //     example_component_container_increment,
    //     example_component_container_add.
    // ],
    processes = [
        example_component_increment
    ]
);
    // #[no_mangle]
    // pub unsafe extern "Rust" fn component_manifest() -> ComponentManifest {
    //     env_logger::init();
    //     ComponentManifest::new("example_component")
    //       .add_container(ContainerManifest::new("example_component_container")
    //         .factory("example_component_container_factory")
    //         .add_process(ProcessManifest::new("example_component_container_get")
    //           .factory("example_component_container_get_factory"))
    //         .add_process(ProcessManifest::new("example_component_container_increment")
    //           .factory("example_component_container_increment_factory"))
    //       ).add_process(ProcessManifest::new("increment_process")
    //         .factory("increment_process_factory"))
    // }

