#include "remem.h"
#include "../vector_store/index.h"
#include "../embedding/engine.h"
#include <cstring>
#include <vector>
#include <iostream>

using namespace remem::vector_store;
using namespace remem::embedding;

struct remem_index_t {
    HNSWIndex* impl;
};

struct remem_embedder_t {
    ONNXEngine* impl;
};

remem_index_t* remem_index_new(size_t dim, size_t max_elements) {
    try {
        auto index = new remem_index_t();
        index->impl = new HNSWIndex(dim, max_elements);
        return index;
    } catch (const std::exception& e) {
        std::cerr << "[libremem] Error in remem_index_new: " << e.what() << std::endl;
        return nullptr;
    }
}

void remem_index_free(remem_index_t* index) {
    try {
        if (index) {
            delete index->impl;
            delete index;
        }
    } catch (...) {}
}

void remem_index_add(remem_index_t* index, const char* id, const float* data, size_t len) {
    try {
        std::vector<float> embedding(data, data + len);
        index->impl->add(id, embedding);
    } catch (const std::exception& e) {
        std::cerr << "[libremem] Error in remem_index_add: " << e.what() << std::endl;
    }
}

void remem_index_remove(remem_index_t* index, const char* id) {
    try {
        index->impl->remove(id);
    } catch (...) {}
}

size_t remem_index_size(remem_index_t* index) {
    try {
        return index->impl->size();
    } catch (...) {
        return 0;
    }
}

remem_search_result_t* remem_index_search(remem_index_t* index, const float* query, size_t k, size_t* out_count) {
    try {
        std::vector<float> q(query, query + index->impl->dim());
        auto results = index->impl->search(q, k);
        
        *out_count = results.size();
        if (results.empty()) return nullptr;
        
        auto out = (remem_search_result_t*)malloc(sizeof(remem_search_result_t) * results.size());
        for (size_t i = 0; i < results.size(); ++i) {
            strncpy(out[i].id, results[i].id.c_str(), 39);
            out[i].id[39] = '\0';
            out[i].similarity = results[i].similarity;
        }
        
        return out;
    } catch (const std::exception& e) {
        std::cerr << "[libremem] Error in remem_index_search: " << e.what() << std::endl;
        *out_count = 0;
        return nullptr;
    }
}

void remem_free_results(remem_search_result_t* results) {
    if (results) free(results);
}

void remem_index_save(remem_index_t* index, const char* path) {
    try {
        index->impl->save(path);
    } catch (const std::exception& e) {
        std::cerr << "[libremem] Error in remem_index_save: " << e.what() << std::endl;
    }
}

void remem_index_load(remem_index_t* index, const char* path) {
    try {
        index->impl->load(path);
    } catch (const std::exception& e) {
        std::cerr << "[libremem] Error in remem_index_load: " << e.what() << std::endl;
    }
}

// --- Embedding Engine (v0.2+) ---

remem_embedder_t* remem_embedder_new(const char* model_path) {
    try {
        auto embedder = new remem_embedder_t();
        embedder->impl = new ONNXEngine(model_path);
        return embedder;
    } catch (const std::exception& e) {
        std::cerr << "[libremem] Error in remem_embedder_new: " << e.what() << std::endl;
        return nullptr;
    }
}

void remem_embedder_free(remem_embedder_t* embedder) {
    try {
        if (embedder) {
            delete embedder->impl;
            delete embedder;
        }
    } catch (...) {}
}

float* remem_embed_text(remem_embedder_t* embedder, const char* text, size_t* out_dim) {
    try {
        auto embedding = embedder->impl->embed(text);
        *out_dim = embedding.size();
        
        float* out = (float*)malloc(sizeof(float) * embedding.size());
        std::memcpy(out, embedding.data(), sizeof(float) * embedding.size());
        
        return out;
    } catch (const std::exception& e) {
        std::cerr << "[libremem] Error in remem_embed_text: " << e.what() << std::endl;
        *out_dim = 0;
        return nullptr;
    }
}

void remem_free_embedding(float* embedding) {
    if (embedding) free(embedding);
}

size_t remem_embedder_dim(remem_embedder_t* embedder) {
    try {
        return embedder ? embedder->impl->dimension() : 0;
    } catch (...) {
        return 0;
    }
}
