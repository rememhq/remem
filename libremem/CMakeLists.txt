# libremem CMake build (v0.2+)
# Placeholder — will build the C++ embedding engine and HNSW index

cmake_minimum_required(VERSION 3.10)
project(libremem CXX C)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

include_directories(include)

add_library(remem SHARED
    src/vector_store/index.cpp
    src/ffi/remem.cpp
)

target_include_directories(remem PUBLIC include)

# TODO (v0.2): Add ONNX Runtime, hnswlib, tokenizers-cpp
message(STATUS "libremem C++ core — not yet implemented in v0.1")
