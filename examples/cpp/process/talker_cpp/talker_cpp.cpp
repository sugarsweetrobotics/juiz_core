
#include "juiz/juiz.h"
#include <iostream>
auto manifest() {
    return ProcessManifest{"talker_cpp"};
}

std::optional<std::string> talker_cpp() {
    std::cout << "talker_cpp() called" << std::endl;
    return "Hello, Juiz!";
}

PROCESS_FACTORY(manifest, talker_cpp);
