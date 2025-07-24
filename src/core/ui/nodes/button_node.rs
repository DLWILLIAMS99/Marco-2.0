use crate::core::logic::{InputMap, OutputMap, Evaluatable, EvalContext};
use crate::core::types::MetaValue;
use crate::core::types::error::MarcoError;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ButtonNode;

impl Evaluatable for ButtonNode {
    fn evaluate(&self, inputs: &InputMap, _ctx: &EvalContext) -> Result<OutputMap, MarcoError> {
        let label = inputs.get("label").and_then(|v| v.as_string()).unwrap_or("Button".to_string());
        let clicked = inputs.get("clicked").and_then(|v| v.as_bool()).unwrap_or(false);
        Ok(indexmap::indexmap! { "label".into() => MetaValue::String(label), "clicked".into() => MetaValue::Bool(clicked) })
    }
}
