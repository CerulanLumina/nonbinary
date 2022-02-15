cmake_minimum_required(VERSION 3.21)
project(nonbinary_abi_test C)

set(CMAKE_C_STANDARD 11)

include_directories(../headers)

add_executable(nonbinary_abi_test_static
        call_rust.c)
add_executable(nonbinary_abi_test_shared
        call_rust.c)

target_link_directories(nonbinary_abi_test_static PRIVATE ../target/release)
target_link_directories(nonbinary_abi_test_shared PRIVATE ../target/release)
target_link_libraries(nonbinary_abi_test_static PRIVATE nonbinary.a pthread dl m)
target_link_libraries(nonbinary_abi_test_shared PRIVATE nonbinary)