#pragma once

#include <string>
#include <vector>
#include <cstdint>

namespace remem {
namespace embedding {

class Tokenizer {
public:
    Tokenizer(const std::string& vocab_path);
    std::vector<int64_t> tokenize(const std::string& text);
    
private:
    // TODO: Use a real tokenizer library (e.g. tokenizers-cpp)
};

} // namespace embedding
} // namespace remem
