use crate::core::logic::{InputMap, OutputMap, Evaluatable, EvalContext};
use crate::core::types::MetaValue;
use crate::core::types::error::MarcoError;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CalculatorNode;

impl Evaluatable for CalculatorNode {
    fn evaluate(&self, inputs: &InputMap, _ctx: &EvalContext) -> Result<OutputMap, MarcoError> {
        let expression = inputs.get("expression").and_then(|v| v.as_string()).unwrap_or_default();
        let x = inputs.get("x").and_then(|v| v.as_scalar()).unwrap_or(0.0);
        let y = inputs.get("y").and_then(|v| v.as_scalar()).unwrap_or(0.0);
        let z = inputs.get("z").and_then(|v| v.as_scalar()).unwrap_or(0.0);
        
        let mut result = HashMap::new();
        
        // Simple expression evaluator (simplified - in practice you'd use a proper parser)
        let computed_result = match expression.as_str() {
            "x + y" => x + y,
            "x - y" => x - y,
            "x * y" => x * y,
            "x / y" => if y != 0.0 { x / y } else { 0.0 },
            "x^2" => x * x,
            "sqrt(x)" => x.sqrt(),
            "sin(x)" => x.sin(),
            "cos(x)" => x.cos(),
            "x + y + z" => x + y + z,
            "x * y * z" => x * y * z,
            "avg(x,y)" => (x + y) / 2.0,
            "avg(x,y,z)" => (x + y + z) / 3.0,
            "distance(x,y)" => (x * x + y * y).sqrt(), // 2D distance from origin
            "lerp(x,y,z)" => x + (y - x) * z.clamp(0.0, 1.0), // z as interpolation factor
            "clamp(x,y,z)" => x.clamp(y, z), // x clamped between y and z
            _ => {
                // Try to parse as a simple number
                expression.parse::<f64>().unwrap_or(0.0)
            }
        };
        
        result.insert("result".to_string(), MetaValue::Scalar(computed_result));
        result.insert("expression".to_string(), MetaValue::String(expression));
        
        // Additional utility outputs
        result.insert("is_valid".to_string(), MetaValue::Bool(!computed_result.is_nan()));
        result.insert("is_positive".to_string(), MetaValue::Bool(computed_result > 0.0));
        result.insert("is_negative".to_string(), MetaValue::Bool(computed_result < 0.0));
        result.insert("is_zero".to_string(), MetaValue::Bool(computed_result.abs() < f64::EPSILON));
        result.insert("absolute".to_string(), MetaValue::Scalar(computed_result.abs()));
        result.insert("rounded".to_string(), MetaValue::Scalar(computed_result.round()));
        
        // Formatted output strings
        result.insert("formatted".to_string(), MetaValue::String(format!("{:.2}", computed_result)));
        result.insert("scientific".to_string(), MetaValue::String(format!("{:.2e}", computed_result)));
        
        Ok(result)
    }

    fn node_type(&self) -> &'static str {
        "calculator"
    }
}
