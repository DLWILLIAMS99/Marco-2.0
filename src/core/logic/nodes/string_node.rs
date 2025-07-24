use crate::core::logic::{InputMap, OutputMap, Evaluatable, EvalContext};
use crate::core::types::MetaValue;
use crate::core::types::error::MarcoError;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StringNode;

impl Evaluatable for StringNode {
    fn evaluate(&self, inputs: &InputMap, _ctx: &EvalContext) -> Result<OutputMap, MarcoError> {
        let text = inputs.get("text").and_then(|v| v.as_string()).unwrap_or_default();
        let other = inputs.get("other").and_then(|v| v.as_string()).unwrap_or_default();
        let separator = inputs.get("separator").and_then(|v| v.as_string()).unwrap_or(" ".to_string());
        let index = inputs.get("index").and_then(|v| v.as_scalar()).unwrap_or(0.0) as usize;
        
        let mut result = HashMap::new();
        
        // String operations
        result.insert("length".to_string(), MetaValue::Scalar(text.len() as f64));
        result.insert("uppercase".to_string(), MetaValue::String(text.to_uppercase()));
        result.insert("lowercase".to_string(), MetaValue::String(text.to_lowercase()));
        result.insert("trimmed".to_string(), MetaValue::String(text.trim().to_string()));
        result.insert("concat".to_string(), MetaValue::String(format!("{}{}", text, other)));
        result.insert("join".to_string(), MetaValue::String(format!("{}{}{}", text, separator, other)));
        
        // String analysis
        result.insert("is_empty".to_string(), MetaValue::Bool(text.is_empty()));
        result.insert("contains".to_string(), MetaValue::Bool(text.contains(&other)));
        result.insert("starts_with".to_string(), MetaValue::Bool(text.starts_with(&other)));
        result.insert("ends_with".to_string(), MetaValue::Bool(text.ends_with(&other)));
        
        // Character access
        let char_at = text.chars().nth(index).map(|c| c.to_string()).unwrap_or_default();
        result.insert("char_at".to_string(), MetaValue::String(char_at));
        
        // Split operations
        let words: Vec<MetaValue> = text.split_whitespace()
            .map(|s| MetaValue::String(s.to_string()))
            .collect();
        result.insert("words".to_string(), MetaValue::List(words));
        
        let lines: Vec<MetaValue> = text.lines()
            .map(|s| MetaValue::String(s.to_string()))
            .collect();
        result.insert("lines".to_string(), MetaValue::List(lines));
        
        Ok(result)
    }

    fn node_type(&self) -> &'static str {
        "string"
    }
}
