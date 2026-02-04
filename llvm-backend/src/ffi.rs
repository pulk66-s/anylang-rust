use crate::compiler::LLVMBackend;
use inkwell::context::Context;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[unsafe(no_mangle)]
pub extern "C" fn backend_name() -> *const c_char {
    let name = CString::new("LLVMBackend").unwrap();
    name.into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn backend_architectures() -> *const c_char {
    let archs = CString::new("x86_64,arm64,any").unwrap();
    archs.into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn compile(ast_json: *const c_char, output_path: *const c_char) -> *mut c_char {
    let ast_str = unsafe {
        if ast_json.is_null() {
            return CString::new("ERROR: AST JSON is null").unwrap().into_raw();
        }
        match CStr::from_ptr(ast_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                let error = format!("ERROR: Invalid AST string: {}", e);
                return CString::new(error).unwrap().into_raw();
            }
        }
    };

    let output_str = unsafe {
        if output_path.is_null() {
            return CString::new("ERROR: Output path is null").unwrap().into_raw();
        }
        match CStr::from_ptr(output_path).to_str() {
            Ok(s) => s,
            Err(e) => {
                let error = format!("ERROR: Invalid output path: {}", e);
                return CString::new(error).unwrap().into_raw();
            }
        }
    };

    let context = Context::create();
    let mut backend = LLVMBackend::new(&context);
    match backend.compile(ast_str, output_str) {
        Ok(()) => CString::new("OK").unwrap().into_raw(),
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