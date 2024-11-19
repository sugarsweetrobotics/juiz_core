
#include <iostream>
#include "juiz/juiz.h"


ProcessManifest manifest() {
    return ProcessManifest{"listener_cpp"}
        .add_string_arg("arg1", "test_argument", "Hello, Juiz!");

}

std::optional<int64_t> listener_cpp(const std::string& msg) {
    //std::cout << "listener_cpp() called" << std::endl;
    // auto a = cm.get_string("arg1");
    std::cout << "lister_cpp:" << msg << std::endl;
    return 0;
}

PROCESS_FACTORY(manifest, listener_cpp);

