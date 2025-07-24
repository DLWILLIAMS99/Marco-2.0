use crate::core::logic::{InputMap, OutputMap, Evaluatable, EvalContext};
use crate::core::types::MetaValue;
use crate::core::types::error::MarcoError;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SliderNode;

impl Evaluatable for SliderNode {
    fn evaluate(&self, inputs: &InputMap, _ctx: &EvalContext) -> Result<OutputMap, MarcoError> {
        let value = inputs.get("value").and_then(|v| v.as_scalar()).unwrap_or(0.0);
        let min = inputs.get("min").and_then(|v| v.as_scalar()).unwrap_or(0.0);
        let max = inputs.get("max").and_then(|v| v.as_scalar()).unwrap_or(1.0);
        Ok(indexmap::indexmap! { "value".into() => MetaValue::Scalar(value), "min".into() => MetaValue::Scalar(min), "max".into() => MetaValue::Scalar(max) })
    }
}
