
#include <iostream>
#include "juiz/juiz.h"


juiz::Value manifest() {
    return ProcessManifest{"listener_cpp"}
        .add_string_arg("arg1", "test_argument", "Hello, Juiz!")
        .into_value();

}

std::optional<int64_t> listener_cpp(juiz::CapsuleMap cm) {
    //std::cout << "listener_cpp() called" << std::endl;
    auto a = cm.get_string("arg1");
    std::cout << "lister_cpp:" << a << std::endl;
    return 0;
}

PROCESS_FACTORY(manifest, listener_cpp);

