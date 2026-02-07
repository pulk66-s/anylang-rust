use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Universal Tagged AST node
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaggedAst {
    pub tag: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, serde_json::Value>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<TaggedAst>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl TaggedAst {
    pub fn new(tag: impl Into<String>) -> Self {
        Self {
            tag: tag.into(),
            id: None,
            attributes: None,
            children: None,
            metadata: None,
        }
    }

    pub fn with_attribute(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.attributes
            .get_or_insert_with(HashMap::new)
            .insert(key.into(), value);
        self
    }

    pub fn with_child(mut self, child: TaggedAst) -> Self {
        self.children.get_or_insert_with(Vec::new).push(child);
        self
    }

    pub fn with_children(mut self, children: Vec<TaggedAst>) -> Self {
        self.children = Some(children);
        self
    }

    pub fn get_attr(&self, key: &str) -> Option<&serde_json::Value> {
        self.attributes.as_ref()?.get(key)
    }

    pub fn get_attr_string(&self, key: &str) -> Option<String> {
        self.get_attr(key)?.as_str().map(|s| s.to_string())
    }

    pub fn get_attr_number(&self, key: &str) -> Option<f64> {
        self.get_attr(key)?.as_f64()
    }

    pub fn get_attr_bool(&self, key: &str) -> Option<bool> {
        self.get_attr(key)?.as_bool()
    }
}

// Helper functions for creating common nodes
impl TaggedAst {
    pub fn literal_number(value: f64) -> Self {
        Self::new("literal_number")
            .with_attribute("value", serde_json::json!(value))
    }

    pub fn literal_string(value: impl Into<String>) -> Self {
        Self::new("literal_string")
            .with_attribute("value", serde_json::json!(value.into()))
    }

    pub fn literal_boolean(value: bool) -> Self {
        Self::new("literal_boolean")
            .with_attribute("value", serde_json::json!(value))
    }

    pub fn variable_reference(name: impl Into<String>) -> Self {
        Self::new("variable_reference")
            .with_attribute("name", serde_json::json!(name.into()))
    }

    pub fn binary_op(op: impl Into<String>, left: TaggedAst, right: TaggedAst) -> Self {
        Self::new("binary_op")
            .with_attribute("operator", serde_json::json!(op.into()))
            .with_children(vec![left, right])
    }

    pub fn function_call(name: impl Into<String>, args: Vec<TaggedAst>) -> Self {
        Self::new("function_call")
            .with_attribute("name", serde_json::json!(name.into()))
            .with_attribute("args_count", serde_json::json!(args.len()))
            .with_children(args)
    }

    pub fn conditional(condition: TaggedAst, then_branch: TaggedAst, else_branch: TaggedAst) -> Self {
        Self::new("conditional")
            .with_children(vec![condition, then_branch, else_branch])
    }
}

