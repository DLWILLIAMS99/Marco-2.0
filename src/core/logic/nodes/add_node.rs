use crate::core::logic::{InputMap, OutputMap, Evaluatable, EvalContext};
use crate::core::types::MetaValue;
use crate::core::types::error::MarcoError;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AddNode;

impl Evaluatable for AddNode {
    fn evaluate(&self, inputs: &InputMap, _ctx: &EvalContext) -> Result<OutputMap, MarcoError> {
        let a = inputs.get("a").and_then(|v| v.as_scalar()).ok_or_else(|| MarcoError::NodeEval("Missing input 'a'".into()))?;
        let b = inputs.get("b").and_then(|v| v.as_scalar()).ok_or_else(|| MarcoError::NodeEval("Missing input 'b'".into()))?;
        
        let mut result = HashMap::new();
        result.insert("result".to_string(), MetaValue::Scalar(a + b));
        Ok(result)
    }
}
