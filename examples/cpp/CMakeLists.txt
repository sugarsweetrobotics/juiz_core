cmake_minimum_required(VERSION 3.13)

project(juiz_cpp_examples CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

set(JUIZ_HEADER_DIR bindings/cppjuiz/include)
set(PROJECT_HOME ${PROJECT_SOURCE_DIR}/../../)
include_directories(${PROJECT_HOME}/${JUIZ_HEADER_DIR})

link_directories(${PROJECT_HOME}/target/debug)

add_subdirectory(process)
add_subdirectory(container)
add_subdirectory(component)