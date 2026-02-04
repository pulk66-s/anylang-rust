use libloading::{Library, Symbol};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

type ParserNameFn = unsafe extern "C" fn() -> *const c_char;
type ParseFn = unsafe extern "C" fn(*const c_char) -> *mut c_char;
type FreeStringFn = unsafe extern "C" fn(*mut c_char);

pub trait Parser {
    fn name(&self) -> Result<String, String>;
    fn parse(&self, input: &str) -> Result<String, String>;
}

pub struct DynamicParser {
    lib: Library,
}

impl DynamicParser {
    pub fn load(path: &str) -> Result<Self, String> {
        unsafe {
            match Library::new(path) {
                Ok(lib) => Ok(DynamicParser { lib }),
                Err(e) => Err(format!("Failed to load library {}: {}", path, e)),
            }
        }
    }
}

impl Parser for DynamicParser {
    fn name(&self) -> Result<String, String> {
        unsafe {
            match self.lib.get::<Symbol<ParserNameFn>>(b"parser_name") {
                Ok(func) => {
                    let ptr = func();
                    let c_str = CStr::from_ptr(ptr);
                    Ok(c_str.to_string_lossy().to_string())
                }
                Err(e) => Err(format!("Failed to get parser_name function: {}", e)),
            }
        }
    }

    fn parse(&self, input: &str) -> Result<String, String> {
        unsafe {
            let parse_fn: Symbol<ParseFn> = self
                .lib
                .get(b"parse")
                .map_err(|e| format!("Failed to get parse function: {}", e))?;
            let input_cstr =
                CString::new(input).map_err(|e| format!("Invalid input string: {}", e))?;
            let result_ptr = parse_fn(input_cstr.as_ptr());

            if result_ptr.is_null() {
                return Err("Parser returned null".to_string());
            }

            let result_cstr = CStr::from_ptr(result_ptr);
            let result = result_cstr.to_string_lossy().to_string();

            if let Ok(free_fn) = self.lib.get::<Symbol<FreeStringFn>>(b"free_string") {
                free_fn(result_ptr);
            }
            if result.starts_with("ERROR: ") {
                Err(result[7..].to_string())
            } else {
                Ok(result)
            }
        }
    }
}
