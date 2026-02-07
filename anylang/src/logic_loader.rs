use libloading::{Library, Symbol};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

type ParseLogicFn = unsafe extern "C" fn(*const c_char) -> *mut c_char;
type FreeStringFn = unsafe extern "C" fn(*mut c_char);

pub trait Logic {
    fn parse(&self, ast_json: &str) -> Result<String, String>;
}

pub struct DynamicLogic {
    lib: Library,
}

impl DynamicLogic {
    pub fn load(path: &str) -> Result<Self, String> {
        unsafe {
            let lib = Library::new(path)
                .map_err(|e| format!("Failed to load logic library {}: {}", path, e))?;
            Ok(DynamicLogic { lib })
        }
    }
}

impl Logic for DynamicLogic {
    fn parse(&self, ast_json: &str) -> Result<String, String> {
        unsafe {
            let parse_fn: Symbol<ParseLogicFn> = self
                .lib
                .get(b"parse_logic")
                .map_err(|e| format!("Failed to get parse_logic function: {}", e))?;
            let input_cstr = CString::new(ast_json)
                .map_err(|e| format!("Invalid input string: {}", e))?;
            let result_ptr = parse_fn(input_cstr.as_ptr());

            if result_ptr.is_null() {
                return Err("Logic parser returned null".to_string());
            }

            let result_cstr = CStr::from_ptr(result_ptr);
            let result = result_cstr.to_string_lossy().to_string();

            if let Ok(free_fn) = self.lib.get::<Symbol<FreeStringFn>>(b"free_string") {
                free_fn(result_ptr);
            }
            if result.starts_with("Error") {
                Err(result)
            } else {
                Ok(result)
            }
        }
    }
}
