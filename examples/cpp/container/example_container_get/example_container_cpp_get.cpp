
#include "juiz/juiz.h"
#include "example_container.h"

juiz::Value manifest() {
    return ProcessManifest("example_container_cpp_get")
        .container_type("examlpe_container_cpp")
        .into_value();
        
}

std::optional<int64_t> example_container_get(CppContainer* container, juiz::CapsuleMap cm) {
    return container->value;
}

CONTAINER_PROCESS_FACTORY(CppContainer, manifest, example_container_get)

