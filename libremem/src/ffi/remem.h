#pragma once

#include <stddef.h>
#include <stdint.h>

#ifdef _WIN32
#define REMEM_API __declspec(dllexport)
#else
#define REMEM_API
#endif

#ifdef __cplusplus
extern "C" {
#endif

// Vector Index Opaque Handle
typedef struct remem_index_t remem_index_t;

// Embedding Engine Opaque Handle
typedef struct remem_embedder_t remem_embedder_t;

// Search Result structure
typedef struct {
    char id[40]; // UUID string + null
    float similarity;
} remem_search_result_t;

// Lifecycle
REMEM_API remem_index_t* remem_index_new(size_t dim, size_t max_elements);
REMEM_API void remem_index_free(remem_index_t* index);

// Operations
REMEM_API void remem_index_add(remem_index_t* index, const char* id, const float* data, size_t len);
REMEM_API void remem_index_remove(remem_index_t* index, const char* id);
REMEM_API size_t remem_index_size(remem_index_t* index);
REMEM_API remem_search_result_t* remem_index_search(remem_index_t* index, const float* query, size_t k, size_t* out_count);
REMEM_API void remem_free_results(remem_search_result_t* results);

REMEM_API void remem_index_save(remem_index_t* index, const char* path);
REMEM_API void remem_index_load(remem_index_t* index, const char* path);

// --- Embedding Engine (v0.2+) ---

// Lifecycle
REMEM_API remem_embedder_t* remem_embedder_new(const char* model_path);
REMEM_API void remem_embedder_free(remem_embedder_t* embedder);

// Inference
REMEM_API float* remem_embed_text(remem_embedder_t* embedder, const char* text, size_t* out_dim);
REMEM_API void remem_free_embedding(float* embedding);

// Info
REMEM_API size_t remem_embedder_dim(remem_embedder_t* embedder);

#ifdef __cplusplus
}
#endif
