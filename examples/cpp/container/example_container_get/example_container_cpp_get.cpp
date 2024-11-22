
#include "juiz/juiz.h"
#include "example_container.h"

auto manifest() {
    return ProcessManifest("example_container_cpp_get")
        .container_type("examlpe_container_cpp");
}

std::optional<int64_t> example_container_get(CppContainer* container) {
    return container->value;
}

CONTAINER_PROCESS_FACTORY(CppContainer, manifest, example_container_get)

