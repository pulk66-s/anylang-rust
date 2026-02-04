mod ast;
mod optimizer;
mod ffi;

pub use ast::TaggedAst;
pub use optimizer::LogicOptimizer;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Logic layer trait for optimization passes
pub trait LogicLayer {
    fn name(&self) -> Result<String, String>;
    fn optimize(&self, ast_json: &str) -> Result<String, String>;
}

/// Main logic optimizer
pub struct LispLogic;

impl LogicLayer for LispLogic {
    fn name(&self) -> Result<String, String> {
        Ok("LispLogic".to_string())
    }

    fn optimize(&self, ast_json: &str) -> Result<String, String> {
        let optimizer = LogicOptimizer::new();
        optimizer.optimize(ast_json)
    }
}

// FFI exports
#[unsafe(no_mangle)]
pub extern "C" fn logic_name() -> *const c_char {
    CString::new("LispLogic").unwrap().into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn optimize(ast_json: *const c_char) -> *mut c_char {
    let ast_str = unsafe {
        if ast_json.is_null() {
            return CString::new("ERROR: Null input").unwrap().into_raw();
        }
        match CStr::from_ptr(ast_json).to_str() {
            Ok(s) => s,
            Err(_) => return CString::new("ERROR: Invalid UTF-8").unwrap().into_raw(),
        }
    };

    let logic = LispLogic;
    match logic.optimize(ast_str) {
        Ok(result) => CString::new(result).unwrap().into_raw(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_folding() {
        let logic = LispLogic;
        let input = r#"{
            "tag": "binary_op",
            "attributes": {"operator": "+"},
            "children": [
                {"tag": "literal_number", "attributes": {"value": 2}},
                {"tag": "literal_number", "attributes": {"value": 3}}
            ]
        }"#;
        
        let result = logic.optimize(input).unwrap();
        assert!(result.contains("literal_number"));
        assert!(result.contains("5"));
    }
}
