cmake_minimum_required(VERSION 3.10)
project(libremem CXX C)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

option(USE_ONNXRUNTIME "Enable ONNX Runtime for local embeddings" OFF)

if(USE_ONNXRUNTIME)
    # This assumes ONNX Runtime is installed or provided via CMAKE_PREFIX_PATH
    find_package(onnxruntime REQUIRED)
    add_compile_definitions(USE_ONNXRUNTIME)
endif()

include_directories(include)
include_directories(src)

add_library(remem SHARED
    src/vector_store/index.cpp
    src/embedding/engine.cpp
    src/ffi/remem.cpp
)

target_include_directories(remem PUBLIC include)

# TODO (v0.2): Add ONNX Runtime, hnswlib, tokenizers-cpp
message(STATUS "libremem C++ core — not yet implemented in v0.1")
