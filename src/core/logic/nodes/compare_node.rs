use crate::core::logic::{InputMap, OutputMap, Evaluatable, EvalContext};
use crate::core::types::MetaValue;
use crate::core::types::error::MarcoError;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CompareNode;

impl Evaluatable for CompareNode {
    fn evaluate(&self, inputs: &InputMap, _ctx: &EvalContext) -> Result<OutputMap, MarcoError> {
        let a = inputs.get("a").and_then(|v| v.as_scalar()).unwrap_or(0.0);
        let b = inputs.get("b").and_then(|v| v.as_scalar()).unwrap_or(0.0);
        
        let mut result = HashMap::new();
        result.insert("greater".to_string(), MetaValue::Bool(a > b));
        result.insert("less".to_string(), MetaValue::Bool(a < b));
        result.insert("equal".to_string(), MetaValue::Bool((a - b).abs() < f64::EPSILON));
        result.insert("greater_equal".to_string(), MetaValue::Bool(a >= b));
        result.insert("less_equal".to_string(), MetaValue::Bool(a <= b));
        Ok(result)
    }

    fn node_type(&self) -> &'static str {
        "compare"
    }
}
