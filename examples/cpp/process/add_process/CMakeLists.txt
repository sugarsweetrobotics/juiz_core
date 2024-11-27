cmake_minimum_required(VERSION 3.13)
project(add_process_cpp CXX)

add_library(${PROJECT_NAME} SHARED ${PROJECT_NAME}.cpp)
target_link_libraries(${PROJECT_NAME} juiz_core)
set_target_properties(${PROJECT_NAME}
    PROPERTIES
    ARCHIVE_OUTPUT_DIRECTORY "${PROJECT_HOME}/target/debug"
    LIBRARY_OUTPUT_DIRECTORY "${PROJECT_HOME}/target/debug"
    RUNTIME_OUTPUT_DIRECTORY "${PROJECT_HOME}/target/debug"
)