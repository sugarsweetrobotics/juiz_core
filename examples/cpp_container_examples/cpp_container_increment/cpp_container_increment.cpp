
#include <stdlib.h>
#include <stdio.h>
#include <iostream>

#include <cstdint>
#include "juiz/juiz.h"
#include "cpp_container.h"

juiz::Value manifest() {
    return {
        {"container_type_name", "cpp_container"},
        {"type_name", "cpp_container_increment"},
        {"arguments", {
            {"arg0", {
                {"type", "int"},
                {"description", "test_argument"},
                {"default", 1},
            }}
        } }
    };
}

extern "C" {
   int64_t cpp_container_increment(CppContainer* container, capsule_map* cm);
   int64_t (*container_process_factory())(CppContainer*, capsule_map*,capsule*);
}

std::tuple<CppContainer*, int64_t> deserializer(CppContainer* container, juiz::CapsuleMap cm) {
    int64_t v = cm.get_int("arg0");
    return std::make_tuple(container, v);
}

std::optional<int64_t> increment(CppContainer* container, int64_t v) {
    container->value = container->value + v;
    return container->value;
}

int64_t serializer(capsule* cp, int64_t retval) {
    if (capsule_set_int(cp, retval)) {
        return 0;
    }
    return -1;
}


int64_t container_process_entry_point(CppContainer* container, capsule_map* cm, capsule* cp) {
    try {
        auto v = deserializer(container, juiz::CapsuleMap(cm));
        auto return_value = std::apply(increment, v);
        if (!return_value) {\
            return -2;\
        }\
        return serializer(cp, return_value.value());\
    } catch (juiz::ValueNotFoundError &e) {\
        return -11;
    }
}

int64_t (*container_process_factory()) (CppContainer*,capsule_map*,capsule*) {
    return container_process_entry_point;
}
DEFINE_MANIFEST_ENTRY_POINT(manifest)

