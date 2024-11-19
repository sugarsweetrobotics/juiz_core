
#include "juiz/juiz.h"

auto manifest() {
    return ProcessManifest{"increment_process_cpp"}
        .add_int_arg("arg1", "test_argument", 1);
}

std::optional<int64_t> increment_process(int64_t arg1) {
    return arg1 + 1;
}

PROCESS_FACTORY(manifest, increment_process);
