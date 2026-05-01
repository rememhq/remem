#pragma once

#include <string>
#include <vector>
#include <memory>

#ifdef USE_ONNXRUNTIME
#include <onnxruntime_cxx_api.h>
#endif

#include "tokenizer.h"

namespace remem {
namespace embedding {

class ONNXEngine {
public:
    ONNXEngine(const std::string& model_path, const std::string& vocab_path);
    ~ONNXEngine();

    std::vector<float> embed(const std::string& text);
    size_t dimension() const { return dim_; }

private:
    size_t dim_ = 768;
    std::unique_ptr<Tokenizer> tokenizer_;

#ifdef USE_ONNXRUNTIME
    Ort::Env env_{ORT_LOGGING_LEVEL_WARNING, "remem"};
    std::unique_ptr<Ort::Session> session_;
    Ort::MemoryInfo memory_info_{Ort::MemoryInfo::CreateCpu(OrtArenaAllocator, OrtMemTypeDefault)};
#else
    void* session_ = nullptr; 
#endif
};

} // namespace embedding
} // namespace remem
