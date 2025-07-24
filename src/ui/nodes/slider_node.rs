use crate::core::logic::{InputMap, OutputMap, Evaluatable, EvalContext};
use crate::core::types::MetaValue;
use crate::core::types::error::MarcoError;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SliderNode;

impl Evaluatable for SliderNode {
    fn evaluate(&self, inputs: &InputMap, _ctx: &EvalContext) -> Result<OutputMap, MarcoError> {
        let value = inputs.get("value").and_then(|v| v.as_scalar()).unwrap_or(0.0);
        let min = inputs.get("min").and_then(|v| v.as_scalar()).unwrap_or(0.0);
        let max = inputs.get("max").and_then(|v| v.as_scalar()).unwrap_or(100.0);
        
        let mut result = HashMap::new();
        result.insert("value".to_string(), MetaValue::Scalar(value));
        result.insert("min".to_string(), MetaValue::Scalar(min));
        result.insert("max".to_string(), MetaValue::Scalar(max));
        Ok(result)
    }
}
