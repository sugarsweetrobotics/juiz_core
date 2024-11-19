
#include <iostream>
#include "juiz/juiz.h"

class CppContainer {
public:
    int64_t value;
    CppContainer(int64_t v) : value(v) {}
};

extern "C" {
    CppContainer* create_example_cpp_container(value* manifest);
    int64_t (*create_example_cpp_container_get())(CppContainer*, capsule_map*,capsule*);
    int64_t (*create_example_cpp_container_increment())(CppContainer*, capsule_map*,capsule*);
}

CppContainer* create_example_cpp_container(value* manifest) {
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

int64_t example_container_process_get(CppContainer* container, capsule_map* cm, capsule* cp) {
    try {
        if (!container) {
            return -2;
        }
        if (capsule_set_int(cp, container->value)) {
            return 0;
        }
        return -1;
    } catch (juiz::ValueNotFoundError &e) {\
        return -11;
    }
}


int64_t example_container_process_increment(CppContainer* container, capsule_map* pcm, capsule* cp) {
    try {
        auto cm = juiz::CapsuleMap(pcm);
        int64_t v = cm.get_int("arg0");
        if (!container) {
            return -2;
        }
        container->value += v;
        if (capsule_set_int(cp, container->value)) {
            return 0;
        }
        return -1;
    } catch (juiz::ValueNotFoundError &e) {\
        return -11;
    }
}

int64_t (*create_example_cpp_container_get())(CppContainer*, capsule_map*, capsule*) {
    return example_container_process_get;
}

int64_t (*create_example_cpp_container_increment())(CppContainer*, capsule_map*, capsule*) {
    return example_container_process_increment;
}

juiz::Value component_manifest() {
    juiz::Value v = {
        {"type_name", "cpp_component"},
        {"containers", std::vector<juiz::Value>{{
                {{"type_name", "example_cpp_container"},
                {"factory", "create_example_cpp_container"},
                {"processes", {
                    {
                        {
                            {"type_name", "example_cpp_container_get"},
                            {"factory", "create_example_cpp_container_get"},
                            {"arguments", juiz::Value::object() }
                        },
                        {
                            {"type_name", "example_cpp_container_increment"},
                            {"factory", "create_example_cpp_container_increment"},
                            {"arguments", {
                                {"arg0", {
                                    {"type", "int"},
                                    {"description", "test_argument"},
                                    {"default", 1},
                                }}
                            } }
                        }
                    }
                }}},
        }}}
    };
    std::cout << "v:" << juiz::str(v) << std::endl;
    return v;
}
 
 
DEFINE_COMPONENT_MANIFEST_ENTRY_POINT(component_manifest) 