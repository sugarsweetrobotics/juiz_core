
#include "juiz/juiz.h"
#include "example_container.h"

juiz::Value manifest() {
    return {
        {"container_type_name", "example_container_cpp"},
        {"type_name", "example_container_cpp_increment"},
        {"arguments", {
            {"arg0", {
                {"type", "int"},
                {"description", "test_argument"},
                {"default", 1},
            }}
        } }
    };
}



std::optional<int64_t> example_container_increment(CppContainer* container, juiz::CapsuleMap cm) {
    int64_t v = cm.get_int("arg0");
    container->value = container->value + v;
    return container->value;
}


CONTAINER_PROCESS_FACTORY(CppContainer, manifest, example_container_increment)

