use libloading::{Library, Symbol};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

type BackendNameFn = unsafe extern "C" fn() -> *const c_char;
type BackendArchsFn = unsafe extern "C" fn() -> *const c_char;
type CompileFn = unsafe extern "C" fn(*const c_char, *const c_char) -> *mut c_char;
type FreeStringFn = unsafe extern "C" fn(*mut c_char);

pub trait Backend {
    fn name(&self) -> Result<String, String>;
    fn architectures(&self) -> Result<Vec<String>, String>;
    fn compile(&self, ast_json: &str, output_path: &str) -> Result<(), String>;
}

pub struct DynamicBackend {
    lib: Library,
}

impl DynamicBackend {
    pub fn load(path: &str) -> Result<Self, String> {
        unsafe {
            let lib = Library::new(path)
                .map_err(|e| format!("Failed to load backend library {}: {}", path, e))?;
            Ok(DynamicBackend { lib })
        }
    }
}

impl Backend for DynamicBackend {
    fn name(&self) -> Result<String, String> {
        unsafe {
            let func: Symbol<BackendNameFn> = self
                .lib
                .get(b"backend_name")
                .map_err(|e| format!("Failed to get backend_name function: {}", e))?;

            let ptr = func();
            let c_str = CStr::from_ptr(ptr);
            Ok(c_str.to_string_lossy().to_string())
        }
    }

    fn architectures(&self) -> Result<Vec<String>, String> {
        unsafe {
            let func: Symbol<BackendArchsFn> = self
                .lib
                .get(b"backend_architectures")
                .map_err(|e| format!("Failed to get backend_architectures function: {}", e))?;

            let ptr = func();
            let c_str = CStr::from_ptr(ptr);
            let archs_str = c_str.to_string_lossy();
            
            Ok(archs_str
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect())
        }
    }

    fn compile(&self, ast_json: &str, output_path: &str) -> Result<(), String> {
        unsafe {
            let compile_fn: Symbol<CompileFn> = self
                .lib
                .get(b"compile")
                .map_err(|e| format!("Failed to get compile function: {}", e))?;
            let ast_cstr = CString::new(ast_json)
                .map_err(|e| format!("Invalid AST JSON string: {}", e))?;
            let output_cstr = CString::new(output_path)
                .map_err(|e| format!("Invalid output path string: {}", e))?;
            let result_ptr = compile_fn(ast_cstr.as_ptr(), output_cstr.as_ptr());

            if result_ptr.is_null() {
                return Err("Backend returned null".to_string());
            }

            let result_cstr = CStr::from_ptr(result_ptr);
            let result = result_cstr.to_string_lossy().to_string();

            if let Ok(free_fn) = self.lib.get::<Symbol<FreeStringFn>>(b"free_string") {
                free_fn(result_ptr);
            }
            if result.starts_with("ERROR: ") {
                Err(result[7..].to_string())
            } else {
                Ok(())
            }
        }
    }
}
