use crate::core::logic::{InputMap, OutputMap, Evaluatable, EvalContext};
use crate::core::types::MetaValue;
use crate::core::types::error::MarcoError;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TimerNode;

impl Evaluatable for TimerNode {
    fn evaluate(&self, inputs: &InputMap, ctx: &EvalContext) -> Result<OutputMap, MarcoError> {
        let duration = inputs.get("duration").and_then(|v| v.as_scalar()).unwrap_or(1.0);
        let _auto_reset = inputs.get("auto_reset").and_then(|v| v.as_bool()).unwrap_or(false);
        let start_trigger = inputs.get("start").and_then(|v| v.as_bool()).unwrap_or(false);
        let _reset_trigger = inputs.get("reset").and_then(|v| v.as_bool()).unwrap_or(false);
        
        // Get current time from context or use a simple counter approach
        // In a real implementation, this would track actual time
        let current_time = ctx.registry.get_scoped(&ctx.scope_id, &crate::core::types::DotPath::from("system.time"))
            .and_then(|v| Ok(v.as_scalar().unwrap_or(0.0)))
            .unwrap_or(0.0);
        
        // Simple timer logic (simplified for demonstration)
        let elapsed = current_time % (duration + 1.0); // Simple cycling
        let progress = (elapsed / duration).min(1.0);
        let is_finished = elapsed >= duration;
        
        let mut result = HashMap::new();
        result.insert("elapsed".to_string(), MetaValue::Scalar(elapsed));
        result.insert("progress".to_string(), MetaValue::Scalar(progress));
        result.insert("remaining".to_string(), MetaValue::Scalar((duration - elapsed).max(0.0)));
        result.insert("finished".to_string(), MetaValue::Bool(is_finished));
        result.insert("running".to_string(), MetaValue::Bool(!is_finished && start_trigger));
        
        // Percentage as integer for display
        result.insert("percentage".to_string(), MetaValue::Scalar((progress * 100.0).round()));
        
        Ok(result)
    }

    fn node_type(&self) -> &'static str {
        "timer"
    }
}
