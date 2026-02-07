use serde_json::Value;

pub struct Logic;

impl Logic {
    pub fn new() -> Self {
        Logic
    }

    pub fn parse(&self, json_value: Value) -> Result<String, String> {
        println!("Received JSON for parsing: {}", json_value);
        Err("Not implemented".to_string())
    }
}
