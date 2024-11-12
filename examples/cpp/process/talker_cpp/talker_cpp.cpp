
#include "juiz/juiz.h"
#include <iostream>
juiz::Value manifest() {
    return ProcessManifest{"talker_cpp"}
        .into_value();
}

std::optional<std::string> talker_cpp(juiz::CapsuleMap cm) {
    std::cout << "talker_cpp() called" << std::endl;
    return "Hello, Juiz!";
}

PROCESS_FACTORY(manifest, talker_cpp);

