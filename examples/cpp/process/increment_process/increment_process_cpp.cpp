
#include "juiz/juiz.h"

juiz::Value manifest() {
    return ProcessManifest{"increment_process_cpp"}
        .add_int_arg("arg1", "test_argument", 1)
        .into_value();
}

std::optional<int64_t> increment_process(juiz::CapsuleMap cm) {
    auto a = cm.get_int("arg1");
    return a + 1;
}

PROCESS_FACTORY(manifest, increment_process);

