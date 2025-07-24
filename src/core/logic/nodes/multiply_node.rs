use crate::core::logic::{InputMap, OutputMap, Evaluatable, EvalContext};
use crate::core::types::MetaValue;
use crate::core::types::error::MarcoError;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MultiplyNode;

impl Evaluatable for MultiplyNode {
    fn evaluate(&self, inputs: &InputMap, _ctx: &EvalContext) -> Result<OutputMap, MarcoError> {
        let a = inputs.get("a").and_then(|v| v.as_scalar()).unwrap_or(1.0);
        let b = inputs.get("b").and_then(|v| v.as_scalar()).unwrap_or(1.0);
        
        let mut result = HashMap::new();
        result.insert("result".to_string(), MetaValue::Scalar(a * b));
        Ok(result)
    }

    fn node_type(&self) -> &'static str {
        "multiply"
    }
}
