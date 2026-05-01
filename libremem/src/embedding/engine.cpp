#include "engine.h"
#include <iostream>
#include <numeric>
#include <cmath>
#include <algorithm>

namespace remem {
namespace embedding {

ONNXEngine::ONNXEngine(const std::string& model_path, const std::string& vocab_path) {
    std::cout << "[libremem] Initializing ONNX engine with model: " << model_path << std::endl;
    tokenizer_ = std::make_unique<Tokenizer>(vocab_path);
#ifdef USE_ONNXRUNTIME
    try {
        Ort::SessionOptions session_options;
        session_options.SetIntraOpNumThreads(1);
        session_options.SetGraphOptimizationLevel(GraphOptimizationLevel::ORT_ENABLE_EXTENDED);
        
        // Convert string to wide string for Windows Ort API
#ifdef _WIN32
        std::wstring w_model_path(model_path.begin(), model_path.end());
        session_ = std::make_unique<Ort::Session>(env_, w_model_path.c_str(), session_options);
#else
        session_ = std::make_unique<Ort::Session>(env_, model_path.c_str(), session_options);
#endif
        std::cout << "[libremem] ONNX session created successfully." << std::endl;
    } catch (const std::exception& e) {
        std::cerr << "[libremem] Failed to load ONNX model: " << e.what() << std::endl;
        throw;
    }
#endif
}

ONNXEngine::~ONNXEngine() {
#ifdef USE_ONNXRUNTIME
    session_.reset();
#endif
}

std::vector<float> ONNXEngine::embed(const std::string& text) {
#ifdef USE_ONNXRUNTIME
    // 1. Real Tokenization (WordPiece)
    const size_t max_seq_length = 512;
    std::vector<int64_t> input_ids = tokenizer_->tokenize(text, max_seq_length);
    
    std::vector<int64_t> attention_mask(max_seq_length, 0);
    for (size_t i = 0; i < input_ids.size(); ++i) {
        if (input_ids[i] != 0) { // Assuming 0 is [PAD]
            attention_mask[i] = 1;
        }
    }
    
    std::vector<int64_t> input_shape = {1, static_cast<int64_t>(max_seq_length)};

    auto input_tensor = Ort::Value::CreateTensor<int64_t>(
        memory_info_, input_ids.data(), input_ids.size(), input_shape.data(), input_shape.size());
    auto mask_tensor = Ort::Value::CreateTensor<int64_t>(
        memory_info_, attention_mask.data(), attention_mask.size(), input_shape.data(), input_shape.size());

    const char* input_names[] = {"input_ids", "attention_mask"};
    const char* output_names[] = {"last_hidden_state"};
    Ort::Value inputs[] = {std::move(input_tensor), std::move(mask_tensor)};

    auto output_tensors = session_->Run(Ort::RunOptions{nullptr}, input_names, inputs, 2, output_names, 1);
    
    float* float_data = output_tensors[0].GetTensorMutableData<float>();
    // Mean pooling or [CLS] token selection (using CLS for now)
    std::vector<float> embedding(float_data, float_data + dim_);
#else
    // Mock implementation for testing bridge without ONNX Runtime binaries
    std::vector<float> embedding(dim_, 0.0f);
    if (text.empty()) return embedding;

    float sum = 0.0f;
    for (size_t i = 0; i < dim_; ++i) {
        float val = static_cast<float>(text[i % text.length()]) / 255.0f;
        val += std::sin(static_cast<float>(i) * 0.1f);
        embedding[i] = val;
        sum += val * val;
    }
#endif

    // L2 Normalize (Crucial for cosine similarity)
    float sum_sq = std::inner_product(embedding.begin(), embedding.end(), embedding.begin(), 0.0f);
    float norm = std::sqrt(sum_sq);
    if (norm > 0) {
        for (float& v : embedding) v /= norm;
    }

    return embedding;
}

} // namespace embedding
} // namespace remem
