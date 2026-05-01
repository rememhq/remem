use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_float};
use std::path::Path;
use uuid::Uuid;
use async_trait::async_trait;

#[repr(C)]
struct RememSearchResult {
    id: [c_char; 40],
    similarity: c_float,
}

extern "C" {
    fn remem_index_new(dim: usize, max_elements: usize) -> *mut std::ffi::c_void;
    fn remem_index_free(index: *mut std::ffi::c_void);
    fn remem_index_add(index: *mut std::ffi::c_void, id: *const c_char, data: *const c_float, len: usize);
    fn remem_index_remove(index: *mut std::ffi::c_void, id: *const c_char);
    fn remem_index_size(index: *mut std::ffi::c_void) -> usize;
    fn remem_index_search(
        index: *mut std::ffi::c_void,
        query: *const c_float,
        k: usize,
        out_count: *mut usize,
    ) -> *mut RememSearchResult;
    fn remem_free_results(results: *mut RememSearchResult);
    fn remem_index_save(index: *mut std::ffi::c_void, path: *const c_char);
    fn remem_index_load(index: *mut std::ffi::c_void, path: *const c_char);
}

#[async_trait]
pub trait VectorIndex: Send + Sync {
    async fn add(&self, id: Uuid, embedding: &[f32]) -> anyhow::Result<()>;
    async fn remove(&self, id: Uuid) -> anyhow::Result<()>;
    async fn search(&self, query: &[f32], k: usize) -> anyhow::Result<Vec<VectorResult>>;
    fn len(&self) -> usize;
    async fn save(&self, path: &Path) -> anyhow::Result<()>;
    async fn load(&self, path: &Path) -> anyhow::Result<()>;
}

#[derive(Debug, Clone)]
pub struct VectorResult {
    pub id: Uuid,
    pub similarity: f32,
}

pub struct HNSWVectorIndex {
    handle: *mut std::ffi::c_void,
}

impl HNSWVectorIndex {
    pub fn new(dim: usize, max_elements: usize) -> Self {
        unsafe {
            let handle = remem_index_new(dim, max_elements);
            Self { handle }
        }
    }
}

impl Drop for HNSWVectorIndex {
    fn drop(&mut self) {
        unsafe {
            remem_index_free(self.handle);
        }
    }
}

unsafe impl Send for HNSWVectorIndex {}
unsafe impl Sync for HNSWVectorIndex {}

#[async_trait]
impl VectorIndex for HNSWVectorIndex {
    async fn add(&self, id: Uuid, embedding: &[f32]) -> anyhow::Result<()> {
        let id_str = CString::new(id.to_string())?;
        unsafe {
            remem_index_add(self.handle, id_str.as_ptr(), embedding.as_ptr(), embedding.len());
        }
        Ok(())
    }

    async fn remove(&self, id: Uuid) -> anyhow::Result<()> {
        let id_str = CString::new(id.to_string())?;
        unsafe {
            remem_index_remove(self.handle, id_str.as_ptr());
        }
        Ok(())
    }

    async fn search(&self, query: &[f32], k: usize) -> anyhow::Result<Vec<VectorResult>> {
        let mut count: usize = 0;
        unsafe {
            let results_ptr = remem_index_search(self.handle, query.as_ptr(), k, &mut count);
            if results_ptr.is_null() {
                return Ok(vec![]);
            }

            let results_slice = std::slice::from_raw_parts(results_ptr, count);
            let mut output = Vec::with_capacity(count);

            for res in results_slice {
                let id_cstr = CStr::from_ptr(res.id.as_ptr());
                let id_str = id_cstr.to_string_lossy();
                if let Ok(uuid) = Uuid::parse_str(&id_str) {
                    output.push(VectorResult {
                        id: uuid,
                        similarity: res.similarity,
                    });
                }
            }

            remem_free_results(results_ptr);
            Ok(output)
        }
    }

    fn len(&self) -> usize {
        unsafe { remem_index_size(self.handle) }
    }

    async fn save(&self, path: &Path) -> anyhow::Result<()> {
        let path_str = CString::new(path.to_string_lossy().to_string())?;
        unsafe {
            remem_index_save(self.handle, path_str.as_ptr());
        }
        Ok(())
    }

    async fn load(&self, path: &Path) -> anyhow::Result<()> {
        let path_str = CString::new(path.to_string_lossy().to_string())?;
        unsafe {
            remem_index_load(self.handle, path_str.as_ptr());
        }
        Ok(())
    }
}
