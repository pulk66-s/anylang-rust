use serde_json::Value;

pub struct AstHelper;

impl AstHelper {
    pub fn is_function_definition(ast: &Value) -> bool {
        if let Some(items) = ast.as_object().and_then(|obj| obj.get("List")).and_then(|v| v.as_array()) {
            if let Some(first) = items.get(0) {
                if let Some(atom) = first.as_object().and_then(|obj| obj.get("Atom")).and_then(|v| v.as_str()) {
                    return atom == "defun";
                }
            }
        }
        false
    }

    pub fn is_function_call(ast: &Value) -> bool {
        ast.as_object()
            .and_then(|obj| obj.get("List"))
            .and_then(|v| v.as_array())
            .is_some()
    }

    pub fn extract_list_items(ast: &Value) -> Option<Vec<Value>> {
        ast.as_object()
            .and_then(|obj| obj.get("List"))
            .and_then(|v| v.as_array())
            .map(|arr| arr.to_vec())
    }
}
