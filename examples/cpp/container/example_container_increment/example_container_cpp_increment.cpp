
#include "juiz/juiz.h"
#include "example_container.h"

juiz::Value manifest() {
    return ProcessManifest("example_container_cpp_increment")
        .container_type("example_container_cpp")
        .add_int_arg("arg0", "test_argument", 2)
        .into_value();
}



std::optional<int64_t> example_container_increment(CppContainer* container, juiz::CapsuleMap cm) {
    int64_t v = cm.get_int("arg0");
    container->value = container->value + v;
    return container->value;
}


CONTAINER_PROCESS_FACTORY(CppContainer, manifest, example_container_increment)

