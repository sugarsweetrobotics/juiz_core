
#include <stdlib.h>
#include <stdio.h>
#include <iostream>

#include <cstdint>
#include "juiz/juiz.h"
#include "cpp_container.h"

juiz::Value manifest() {
    return {
        {"container_type_name", "cpp_container"},
        {"type_name", "cpp_container_get"},
        {"arguments", juiz::Value::object() }
    };
}

extern "C" {
   int64_t cpp_container_get(CppContainer* container, capsule_map* cm);
    int64_t (*container_process_factory())(CppContainer*, capsule_map*,capsule*);
}

std::tuple<> deserializer(juiz::CapsuleMap cm) {
    return std::make_tuple();
}

std::optional<int64_t> get(CppContainer* container) {
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
        auto return_value = get(container);
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

