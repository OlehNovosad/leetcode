cmake_minimum_required(VERSION 4.1)
project(make_binary_string_alternating)

set(CMAKE_CXX_STANDARD 20)

find_package(Threads REQUIRED)

add_executable(make_binary_string_alternating main.cpp)
target_link_libraries(make_binary_string_alternating Threads::Threads)
