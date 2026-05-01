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

// Persistence
REMEM_API void remem_index_save(remem_index_t* index, const char* path);
REMEM_API void remem_index_load(remem_index_t* index, const char* path);

#ifdef __cplusplus
}
#endif
