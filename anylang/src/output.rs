use std::fs;

pub fn write_output(path: &str, content: &str) -> Result<(), String> {
    fs::write(path, content).map_err(|e| format!("Failed to write output: {}", e))
}
