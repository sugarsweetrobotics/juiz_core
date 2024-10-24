
#include <stdlib.h>
#include <stdio.h>
#include <iostream>


#include "juiz/juiz.h"


juiz::Value manifest() {
    return ProcessManifest{"descrement_process_cpp"}
        .add_int_arg("arg1", "test_argument", 1)
        .into_value();
}

std::optional<int64_t> decrement_process(juiz::CapsuleMap cm) {
    auto a = cm.get_int("arg1");
    return a - 1;
}

PROCESS_FACTORY(manifest, decrement_process);

