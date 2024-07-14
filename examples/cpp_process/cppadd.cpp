
#include <stdlib.h>
#include <stdio.h>
#include <iostream>


#include "juiz/juiz.h"

auto deserializer(juiz::CapsuleMap cm) {
    return std::optional(std::make_tuple(
        cm.get_int("arg1"), 
        cm.get_int("arg2")
    ));
}

int64_t serializer(capsule* cp, int64_t retval) {
    return capsule_set_int(cp, retval);
}

std::optional<int64_t> cppadd(int64_t a, int64_t b) {
    return a + b;
}

juiz::Value manifest() {
    return {
        {"type_name", "cppadd"},
        {"arguments", {
            {"arg1", {
                {"type", "int"},
                {"description", "test_argument"},
                {"default", 1},
            }}, 
            {"arg2", {
                {"type", "int"},
                {"description", "test_argument"},
                {"default", 1},
            }}, 
        }}
    };
}

DEFINE_PROCESS_ENTRY_POINT(cppadd, deserializer, serializer)
DEFINE_MANIFEST_ENTRY_POINT(manifest)

