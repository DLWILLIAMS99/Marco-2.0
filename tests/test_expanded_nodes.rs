//! Tests for expanded node library
use std::collections::HashMap;
use crate::core::logic::{InputMap, EvalContext, Evaluatable};
use crate::core::types::MetaValue;
use crate::core::logic::nodes::{
    multiply_node::MultiplyNode,
    compare_node::CompareNode,
    clamp_node::ClampNode,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_node() {
        let node = MultiplyNode;
        let mut inputs = HashMap::new();
        inputs.insert("a".to_string(), MetaValue::Scalar(6.0));
        inputs.insert("b".to_string(), MetaValue::Scalar(7.0));
        
        let ctx = EvalContext::default();
        let result = node.evaluate(&inputs, &ctx).unwrap();
        
        assert_eq!(result.get("result").unwrap().as_scalar(), Some(42.0));
    }

    #[test]
    fn test_compare_node() {
        let node = CompareNode;
        let mut inputs = HashMap::new();
        inputs.insert("a".to_string(), MetaValue::Scalar(10.0));
        inputs.insert("b".to_string(), MetaValue::Scalar(5.0));
        
        let ctx = EvalContext::default();
        let result = node.evaluate(&inputs, &ctx).unwrap();
        
        assert_eq!(result.get("greater").unwrap().as_bool(), Some(true));
        assert_eq!(result.get("less").unwrap().as_bool(), Some(false));
        assert_eq!(result.get("equal").unwrap().as_bool(), Some(false));
    }

    #[test]
    fn test_clamp_node() {
        let node = ClampNode;
        let mut inputs = HashMap::new();
        inputs.insert("value".to_string(), MetaValue::Scalar(15.0));
        inputs.insert("min".to_string(), MetaValue::Scalar(0.0));
        inputs.insert("max".to_string(), MetaValue::Scalar(10.0));
        
        let ctx = EvalContext::default();
        let result = node.evaluate(&inputs, &ctx).unwrap();
        
        assert_eq!(result.get("result").unwrap().as_scalar(), Some(10.0));
    }

    #[test]
    fn test_clamp_node_within_bounds() {
        let node = ClampNode;
        let mut inputs = HashMap::new();
        inputs.insert("value".to_string(), MetaValue::Scalar(5.0));
        inputs.insert("min".to_string(), MetaValue::Scalar(0.0));
        inputs.insert("max".to_string(), MetaValue::Scalar(10.0));
        
        let ctx = EvalContext::default();
        let result = node.evaluate(&inputs, &ctx).unwrap();
        
        assert_eq!(result.get("result").unwrap().as_scalar(), Some(5.0));
    }
}
