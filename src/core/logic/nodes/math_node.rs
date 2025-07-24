use crate::core::logic::{InputMap, OutputMap, Evaluatable, EvalContext};
use crate::core::types::MetaValue;
use crate::core::types::error::MarcoError;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MathNode;

impl Evaluatable for MathNode {
    fn evaluate(&self, inputs: &InputMap, _ctx: &EvalContext) -> Result<OutputMap, MarcoError> {
        let a = inputs.get("a").and_then(|v| v.as_scalar()).unwrap_or(0.0);
        let b = inputs.get("b").and_then(|v| v.as_scalar()).unwrap_or(0.0);
        
        let mut result = HashMap::new();
        // Basic operations
        result.insert("add".to_string(), MetaValue::Scalar(a + b));
        result.insert("subtract".to_string(), MetaValue::Scalar(a - b));
        result.insert("multiply".to_string(), MetaValue::Scalar(a * b));
        result.insert("divide".to_string(), MetaValue::Scalar(if b != 0.0 { a / b } else { 0.0 }));
        result.insert("modulo".to_string(), MetaValue::Scalar(if b != 0.0 { a % b } else { 0.0 }));
        result.insert("power".to_string(), MetaValue::Scalar(a.powf(b)));
        
        // Single input operations on 'a'
        result.insert("sqrt".to_string(), MetaValue::Scalar(a.sqrt()));
        result.insert("sin".to_string(), MetaValue::Scalar(a.sin()));
        result.insert("cos".to_string(), MetaValue::Scalar(a.cos()));
        result.insert("tan".to_string(), MetaValue::Scalar(a.tan()));
        result.insert("abs".to_string(), MetaValue::Scalar(a.abs()));
        result.insert("floor".to_string(), MetaValue::Scalar(a.floor()));
        result.insert("ceil".to_string(), MetaValue::Scalar(a.ceil()));
        result.insert("round".to_string(), MetaValue::Scalar(a.round()));
        
        // Utility operations
        result.insert("min".to_string(), MetaValue::Scalar(a.min(b)));
        result.insert("max".to_string(), MetaValue::Scalar(a.max(b)));
        
        Ok(result)
    }

    fn node_type(&self) -> &'static str {
        "math"
    }
}
