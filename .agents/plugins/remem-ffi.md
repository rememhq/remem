# Plugin: remem-ffi — C++ FFI Bridge Rules

## Scope
`libremem/`, `remem-core/src/storage/vector.rs`

## Rules

### Adding a New FFI Function

1. **C++ Header** (`libremem/src/ffi/remem.h`):
   - Declare the function with `extern "C"` linkage
   - Use only C-compatible types: `const char*`, `float*`, `size_t`, opaque struct pointers

2. **C++ Implementation** (`libremem/src/ffi/remem.cpp`):
   - Wrap ALL logic in `try-catch(const std::exception& e)`
   - Log errors to `stderr` with `[libremem]` prefix
   - Return `nullptr` / `0` on failure — NEVER throw across the FFI boundary

3. **Rust Declaration** (`remem-core/src/storage/vector.rs::remem_ffi`):
   - Declare in the `extern "C"` block inside the `remem_ffi` module
   - Use `*mut c_void` for opaque handles OR define an empty enum type outside the extern block

4. **Build Script** (`remem-core/build.rs`):
   - Add any new `.cpp` files to the `cc::Build` chain

### Forbidden Patterns
- ❌ Rust enums inside `extern "C"` blocks
- ❌ C++ exceptions propagating to Rust (causes `SIGABRT`)
- ❌ Passing Rust `String` directly to C++ (use `CString`)
- ❌ Forgetting to free C-allocated memory (`remem_free_*` functions)
