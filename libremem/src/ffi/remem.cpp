#include "remem.h"
#include "../vector_store/index.h"
#include <cstring>
#include <vector>

using namespace remem::vector_store;

struct remem_index_t {
    HNSWIndex* impl;
};

remem_index_t* remem_index_new(size_t dim, size_t max_elements) {
    auto index = new remem_index_t();
    index->impl = new HNSWIndex(dim, max_elements);
    return index;
}

void remem_index_free(remem_index_t* index) {
    if (index) {
        delete index->impl;
        delete index;
    }
}

void remem_index_add(remem_index_t* index, const char* id, const float* data, size_t len) {
    std::vector<float> embedding(data, data + len);
    index->impl->add(id, embedding);
}

void remem_index_remove(remem_index_t* index, const char* id) {
    index->impl->remove(id);
}

size_t remem_index_size(remem_index_t* index) {
    return index->impl->size();
}

remem_search_result_t* remem_index_search(remem_index_t* index, const float* query, size_t k, size_t* out_count) {
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
}

void remem_free_results(remem_search_result_t* results) {
    if (results) free(results);
}

void remem_index_save(remem_index_t* index, const char* path) {
    index->impl->save(path);
}

void remem_index_load(remem_index_t* index, const char* path) {
    index->impl->load(path);
}
