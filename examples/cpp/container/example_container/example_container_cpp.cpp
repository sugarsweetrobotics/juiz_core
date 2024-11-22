#include <iostream>
#include "juiz/juiz.h"
#include "example_container.h"

auto manifest() {
    return ContainerManifest("example_container_cpp");
}

CppContainer* create_container(juiz::Value value) {
    int64_t int_value = 0;
    
    if (value.isObjectValue()) {
        if (value.hasKey("value")) {
            auto objv = value.objectValue();
            auto v = objv["value"];
            if (v.isIntValue()) {
               int_value = v.intValue();
            }
        }   
    }
    return new CppContainer(int_value);
}


CONTAINER_FACTORY(manifest, create_container);

