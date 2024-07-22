
#include <stdlib.h>
#include <stdio.h>
#include <iostream>

#include <cstdint>
#include "juiz/juiz.h"


#include "cpp_container.h"

juiz::Value manifest() {
    return {
        {"type_name", "cpp_container"},
    };
}

extern "C" {
  CppContainer* create_container(value* manifest);
  bool destroy_container(CppContainer* p_container);
}


CppContainer* create_container(value* manifest) {
    int64_t int_value = 0;
    if (value_is_object(manifest)) {
        auto v = value_object_get_value(manifest, "value");
        if (v) {
            if(value_get_int(v, &int_value) != 0) {
                int_value = 0;
            }
        }   
    }
    return new CppContainer(int_value);
}

bool destroy_container(CppContainer* p_container) {
    if (p_container) {
        delete p_container;
        return true;
    }
    return false;
}

DEFINE_MANIFEST_ENTRY_POINT(manifest)

