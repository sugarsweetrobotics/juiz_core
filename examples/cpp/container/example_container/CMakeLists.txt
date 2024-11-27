cmake_minimum_required(VERSION 3.13)
project(example_container_cpp CXX)

include_directories(../example_container)

add_juiz_container(${PROJECT_NAME} SOURCES ${PROJECT_NAME}.cpp)