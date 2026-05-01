#pragma once

#include <string>
#include <vector>
#include <cstdint>
#include <unordered_map>

namespace remem {
namespace embedding {

class Tokenizer {
public:
    explicit Tokenizer(const std::string& vocab_path);
    std::vector<int64_t> tokenize(const std::string& text, size_t max_length = 512);

private:
    std::unordered_map<std::string, int64_t> vocab_;
    int64_t unk_id_ = 100;
    int64_t cls_id_ = 101;
    int64_t sep_id_ = 102;
    int64_t pad_id_ = 0;

    std::vector<std::string> basic_tokenize(const std::string& text);
    std::vector<int64_t> wordpiece_tokenize(const std::string& token);
};

} // namespace embedding
} // namespace remem
