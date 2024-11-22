
#include "juiz/juiz.h"
#include "example_container.h"

auto manifest() {
    return ProcessManifest("example_container_cpp_increment")
        .container_type("example_container_cpp")
        .add_int_arg("arg0", "test_argument", 2);
}



std::optional<int64_t> example_container_increment(CppContainer* container, int64_t arg0) {
    container->value = container->value + arg0;
    return container->value;
}


CONTAINER_PROCESS_FACTORY(CppContainer, manifest, example_container_increment)

