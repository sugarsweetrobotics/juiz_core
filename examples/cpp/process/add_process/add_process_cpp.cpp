
#include "juiz/juiz.h"

auto manifest() {
    return ProcessManifest{"iadd_process_cpp"}
        .add_int_arg("arg1", "test_argument", 1)
        .add_int_arg("arg2", "test_argument", 1);
}

std::optional<int64_t> add_process(const int64_t arg1, const int64_t arg2) {
    return arg1 + arg2;
}

PROCESS_FACTORY(manifest, add_process);