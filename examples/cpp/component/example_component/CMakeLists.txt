cmake_minimum_required(VERSION 3.13)
project(cpp_component CXX)

include_directories(../cpp_component)


add_juiz_component(${PROJECT_NAME} SOURCES ${PROJECT_NAME}.cpp)