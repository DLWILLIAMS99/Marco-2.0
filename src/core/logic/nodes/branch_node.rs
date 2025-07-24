use crate::core::logic::{InputMap, OutputMap, Evaluatable, EvalContext};
use crate::core::types::MetaValue;
use crate::core::types::error::MarcoError;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BranchNode;

impl Evaluatable for BranchNode {
    fn evaluate(&self, inputs: &InputMap, _ctx: &EvalContext) -> Result<OutputMap, MarcoError> {
        let cond = inputs.get("condition").and_then(|v| v.as_bool()).ok_or_else(|| MarcoError::NodeEval("Missing condition".into()))?;
        let true_val = inputs.get("true_value").cloned().unwrap_or(MetaValue::Bool(true));
        let false_val = inputs.get("false_value").cloned().unwrap_or(MetaValue::Bool(false));
        
        let mut result = HashMap::new();
        result.insert("result".to_string(), if cond { true_val } else { false_val });
        Ok(result)
    }
}
