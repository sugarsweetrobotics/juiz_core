
#include "juiz/juiz.h"
#include "example_container.h"

juiz::Value manifest() {
    return {
        {"container_type_name", "example_container_cpp"},
        {"type_name", "example_container_cpp_get"},
        {"arguments", juiz::Value::object() }
    };
}

std::optional<int64_t> example_container_get(CppContainer* container, juiz::CapsuleMap cm) {
    return container->value;
}

CONTAINER_PROCESS_FACTORY(CppContainer, manifest, example_container_get)

