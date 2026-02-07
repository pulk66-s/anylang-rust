use crate::parser::LispParser;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[unsafe(no_mangle)]
pub extern "C" fn parser_name() -> *const c_char {
    let name = CString::new("LispParser").unwrap();
    name.into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn parser_extensions() -> *const c_char {
    let extensions = CString::new("lisp,lsp,cl").unwrap();
    extensions.into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn parse(input: *const c_char) -> *mut c_char {
    let input_str = unsafe {
        if input.is_null() {
            return std::ptr::null_mut();
        }
        CStr::from_ptr(input).to_str().unwrap_or("")
    };

    let parser = LispParser::new();
    match parser.parse(input_str) {
        Ok(ast) => {
            match serde_json::to_string_pretty(&ast) {
                Ok(json) => {
                    println!("Parsed AST: {}", json);
                    CString::new(json).unwrap().into_raw()
                },
                Err(e) => {
                    let error = format!("ERROR: JSON serialization failed: {}", e);
                    CString::new(error).unwrap().into_raw()
                }
            }
        }
        Err(e) => {
            let error = format!("ERROR: {}", e);
            CString::new(error).unwrap().into_raw()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}
