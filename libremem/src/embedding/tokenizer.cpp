#include "tokenizer.h"
#include <fstream>
#include <sstream>
#include <algorithm>
#include <cctype>

namespace remem {
namespace embedding {

Tokenizer::Tokenizer(const std::string& vocab_path) {
    std::ifstream file(vocab_path);
    if (!file.is_open()) {
        // Fallback or error handling
        return;
    }

    std::string line;
    int64_t id = 0;
    while (std::getline(file, line)) {
        if (!line.empty()) {
            vocab_[line] = id;
            if (line == "[UNK]") unk_id_ = id;
            if (line == "[CLS]") cls_id_ = id;
            if (line == "[SEP]") sep_id_ = id;
            if (line == "[PAD]") pad_id_ = id;
        }
        id++;
    }
}

std::vector<int64_t> Tokenizer::tokenize(const std::string& text, size_t max_length) {
    std::vector<int64_t> ids;
    ids.push_back(cls_id_);

    std::vector<std::string> tokens = basic_tokenize(text);
    for (const auto& token : tokens) {
        std::vector<int64_t> subword_ids = wordpiece_tokenize(token);
        ids.insert(ids.end(), subword_ids.begin(), subword_ids.end());
        
        if (ids.size() >= max_length - 1) break;
    }

    if (ids.size() > max_length - 1) {
        ids.resize(max_length - 1);
    }
    ids.push_back(sep_id_);

    // Padding
    while (ids.size() < max_length) {
        ids.push_back(pad_id_);
    }

    return ids;
}

std::vector<std::string> Tokenizer::basic_tokenize(const std::string& text) {
    std::vector<std::string> tokens;
    std::string current;

    for (unsigned char c : text) {
        if (std::isspace(c)) {
            if (!current.empty()) {
                tokens.push_back(current);
                current.clear();
            }
        } else if (std::ispunct(c)) {
            if (!current.empty()) {
                tokens.push_back(current);
                current.clear();
            }
            tokens.push_back(std::string(1, c));
        } else {
            current += static_cast<char>(std::tolower(c));
        }
    }

    if (!current.empty()) {
        tokens.push_back(current);
    }

    return tokens;
}

std::vector<int64_t> Tokenizer::wordpiece_tokenize(const std::string& token) {
    if (vocab_.count(token)) {
        return {vocab_.at(token)};
    }

    std::vector<int64_t> ids;
    size_t start = 0;
    while (start < token.length()) {
        size_t end = token.length();
        std::string cur_substr;
        int64_t cur_id = -1;

        while (start < end) {
            std::string substr = token.substr(start, end - start);
            if (start > 0) substr = "##" + substr;
            
            if (vocab_.count(substr)) {
                cur_id = vocab_.at(substr);
                cur_substr = substr;
                break;
            }
            end--;
        }

        if (cur_id == -1) {
            return {unk_id_};
        }

        ids.push_back(cur_id);
        start = end;
    }

    return ids;
}

} // namespace embedding
} // namespace remem
