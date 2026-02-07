use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use crate::logic::Logic;

#[unsafe(no_mangle)]
pub extern "C" fn parse_logic(input: *const c_char) -> *mut c_char {
    let input_str = unsafe {
        if input.is_null() {
            return std::ptr::null_mut();
        }
        CStr::from_ptr(input).to_str().unwrap_or("")
    };

    let json_value: serde_json::Value = match serde_json::from_str(input_str) {
        Ok(v) => v,
        Err(e) => {
            let error_msg = format!("Error parsing JSON: {}", e);
            return match CString::new(error_msg) {
                Ok(c_str) => c_str.into_raw(),
                Err(_) => std::ptr::null_mut(),
            };
        }
    };
    
    let mut logic = Logic::new();
    let result = match logic.parse(&json_value) {
        Ok(output) => format!("{:?}", output),
        Err(e) => format!("Error parsing logic: {}", e),
    };

    match CString::new(result) {
        Ok(c_str) => c_str.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}
