
#include <stdlib.h>
#include <stdio.h>
#include <iostream>


#include "juiz/juiz.h"


juiz::Value manifest() {
    return {
        {"type_name", "decrement_process_cpp"},
        {"language", "c++"},
        {"arguments", {
            {"arg1", {
                {"type", "int"},
                {"description", "test_argument"},
                {"default", 1},
            }}
        }}
    };
}

std::optional<int64_t> decrement_process(juiz::CapsuleMap cm) {
    auto a = cm.get_int("arg1");
    return a - 1;
}

PROCESS_FACTORY(manifest, decrement_process);

