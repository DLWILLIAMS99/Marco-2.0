use crate::core::types::{MetaValue, ScopeId};
use crate::core::logic::{EvalContext, Evaluatable};
use crate::core::logic::nodes::*;
use crate::core::registry::MetaRegistry;
use std::collections::HashMap;
use std::sync::Arc;

/// Test utilities for node evaluation
pub fn create_test_inputs(pairs: &[(&str, MetaValue)]) -> HashMap<String, MetaValue> {
    pairs.iter().map(|(k, v)| (k.to_string(), v.clone())).collect()
}

pub fn create_test_context() -> EvalContext {
    EvalContext {
        registry: Arc::new(MetaRegistry::new()),
        scope_id: ScopeId::new(),
        parent: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math_node_operations() {
        let node = MathNode;
        let ctx = create_test_context();

        // Test addition
        let inputs = create_test_inputs(&[
            ("a", MetaValue::Scalar(5.0)),
            ("b", MetaValue::Scalar(3.0)),
            ("operation", MetaValue::String("add".to_string())),
        ]);
        let result = node.evaluate(&inputs, &ctx).expect("Node evaluation should succeed");
        assert_eq!(result.get("result"), Some(&MetaValue::Scalar(8.0)));

        // Test square root
        let inputs = create_test_inputs(&[
            ("a", MetaValue::Scalar(16.0)),
            ("operation", MetaValue::String("sqrt".to_string())),
        ]);
        let result = node.evaluate(&inputs, &ctx).expect("Node evaluation should succeed");
        assert_eq!(result.get("result"), Some(&MetaValue::Scalar(4.0)));

        // Test power
        let inputs = create_test_inputs(&[
            ("a", MetaValue::Scalar(2.0)),
            ("b", MetaValue::Scalar(3.0)),
            ("operation", MetaValue::String("power".to_string())),
        ]);
        let result = node.evaluate(&inputs, &ctx).expect("Node evaluation should succeed");
        assert_eq!(result.get("result"), Some(&MetaValue::Scalar(8.0)));

        // Test trigonometry
        let inputs = create_test_inputs(&[
            ("a", MetaValue::Scalar(0.0)),
            ("operation", MetaValue::String("sin".to_string())),
        ]);
        let result = node.evaluate(&inputs, &ctx).expect("Node evaluation should succeed");
        assert_eq!(result.get("result"), Some(&MetaValue::Scalar(0.0)));
    }

    #[test]
    fn test_string_node_operations() {
        let node = StringNode;
        let ctx = create_test_context();

        // Test length
        let inputs = create_test_inputs(&[
            ("text", MetaValue::String("hello".to_string())),
            ("operation", MetaValue::String("length".to_string())),
        ]);
        let result = node.evaluate(&inputs, &ctx).expect("Node evaluation should succeed");
        assert_eq!(result.get("result"), Some(&MetaValue::Scalar(5.0)));
        assert_eq!(result.get("length"), Some(&MetaValue::Scalar(5.0)));

        // Test uppercase
        let inputs = create_test_inputs(&[
            ("text", MetaValue::String("hello".to_string())),
            ("operation", MetaValue::String("uppercase".to_string())),
        ]);
        let result = node.evaluate(&inputs, &ctx).expect("Node evaluation should succeed");
        assert_eq!(result.get("result"), Some(&MetaValue::String("HELLO".to_string())));

        // Test concat
        let inputs = create_test_inputs(&[
            ("text", MetaValue::String("hello".to_string())),
            ("operation", MetaValue::String("concat".to_string())),
            ("param", MetaValue::String(" world".to_string())),
        ]);
        let result = node.evaluate(&inputs, &ctx).expect("Node evaluation should succeed");
        assert_eq!(result.get("result"), Some(&MetaValue::String("hello world".to_string())));

        // Test contains
        let inputs = create_test_inputs(&[
            ("text", MetaValue::String("hello world".to_string())),
            ("operation", MetaValue::String("contains".to_string())),
            ("param", MetaValue::String("world".to_string())),
        ]);
        let result = node.evaluate(&inputs, &ctx).expect("Node evaluation should succeed");
        assert_eq!(result.get("result"), Some(&MetaValue::Bool(true)));
    }

    #[test]
    fn test_timer_node_functionality() {
        let mut node = TimerNode;
        let ctx = create_test_context();

        // Test basic timer initialization
        let inputs = create_test_inputs(&[
            ("duration", MetaValue::Scalar(10.0)),
            ("reset", MetaValue::Bool(false)),
        ]);
        let result = node.evaluate(&inputs, &ctx).expect("Node evaluation should succeed");
        
        // Should have valid outputs
        assert!(result.contains_key("elapsed"));
        assert!(result.contains_key("progress"));
        assert!(result.contains_key("complete"));
        
        // Progress should be between 0 and 1
        if let Some(MetaValue::Scalar(progress)) = result.get("progress") {
            assert!(*progress >= 0.0 && *progress <= 1.0);
        }

        // Test reset functionality
        let inputs = create_test_inputs(&[
            ("duration", MetaValue::Scalar(10.0)),
            ("reset", MetaValue::Bool(true)),
        ]);
        let result = node.evaluate(&inputs, &ctx).expect("Node evaluation should succeed");
        assert_eq!(result.get("elapsed"), Some(&MetaValue::Scalar(0.0)));
        assert_eq!(result.get("progress"), Some(&MetaValue::Scalar(0.0)));
        assert_eq!(result.get("complete"), Some(&MetaValue::Bool(false)));
    }

    #[test]
    fn test_calculator_node_expressions() {
        let node = CalculatorNode;
        let ctx = create_test_context();

        // Test simple expression
        let inputs = create_test_inputs(&[
            ("expression", MetaValue::String("2 + 3 * 4".to_string())),
        ]);
        let result = node.evaluate(&inputs, &ctx).expect("Node evaluation should succeed");
        assert_eq!(result.get("result"), Some(&MetaValue::Scalar(14.0)));
        assert_eq!(result.get("valid"), Some(&MetaValue::Bool(true)));

        // Test expression with variables
        let mut variables = HashMap::new();
        variables.insert("x".to_string(), MetaValue::Scalar(5.0));
        variables.insert("y".to_string(), MetaValue::Scalar(3.0));
        
        let inputs = create_test_inputs(&[
            ("expression", MetaValue::String("x * y + 2".to_string())),
            ("variables", MetaValue::Object(variables)),
        ]);
        let result = node.evaluate(&inputs, &ctx).expect("Node evaluation should succeed");
        assert_eq!(result.get("result"), Some(&MetaValue::Scalar(17.0)));

        // Test invalid expression
        let inputs = create_test_inputs(&[
            ("expression", MetaValue::String("2 + + 3".to_string())),
        ]);
        let result = node.evaluate(&inputs, &ctx).expect("Node evaluation should succeed");
        assert_eq!(result.get("valid"), Some(&MetaValue::Bool(false)));
    }

    #[test]
    fn test_database_node_operations() {
        let node = DatabaseNode;
        let ctx = create_test_context();

        // Test insert operation
        let mut data = HashMap::new();
        data.insert("name".to_string(), MetaValue::String("John".to_string()));
        data.insert("age".to_string(), MetaValue::Scalar(30.0));

        let inputs = create_test_inputs(&[
            ("operation", MetaValue::String("insert".to_string())),
            ("table", MetaValue::String("users".to_string())),
            ("data", MetaValue::Object(data)),
        ]);
        let result = node.evaluate(&inputs, &ctx).expect("Node evaluation should succeed");
        
        assert_eq!(result.get("status"), Some(&MetaValue::String("success".to_string())));
        assert!(result.contains_key("result"));

        // Test select operation
        let inputs = create_test_inputs(&[
            ("operation", MetaValue::String("select".to_string())),
            ("table", MetaValue::String("users".to_string())),
            ("query", MetaValue::String("name = John".to_string())),
        ]);
        let result = node.evaluate(&inputs, &ctx).expect("Node evaluation should succeed");
        
        assert_eq!(result.get("status"), Some(&MetaValue::String("success".to_string())));
        if let Some(MetaValue::Scalar(count)) = result.get("count") {
            assert!(count >= &0.0);
        }
    }

    #[test]
    fn test_validation_node_rules() {
        let node = ValidationNode;
        let ctx = create_test_context();

        // Test email validation
        let mut rules = HashMap::new();
        rules.insert("email".to_string(), MetaValue::Bool(true));

        let inputs = create_test_inputs(&[
            ("value", MetaValue::String("test@example.com".to_string())),
            ("rules", MetaValue::Object(rules.clone())),
        ]);
        let result = node.evaluate(&inputs, &ctx).expect("Node evaluation should succeed");
        assert_eq!(result.get("valid"), Some(&MetaValue::Bool(true)));

        // Test invalid email
        let inputs = create_test_inputs(&[
            ("value", MetaValue::String("invalid-email".to_string())),
            ("rules", MetaValue::Object(rules)),
        ]);
        let result = node.evaluate(&inputs, &ctx).expect("Node evaluation should succeed");
        assert_eq!(result.get("valid"), Some(&MetaValue::Bool(false)));

        // Test number validation
        let mut rules = HashMap::new();
        rules.insert("number".to_string(), MetaValue::Bool(true));

        let inputs = create_test_inputs(&[
            ("value", MetaValue::String("123.45".to_string())),
            ("rules", MetaValue::Object(rules)),
        ]);
        let result = node.evaluate(&inputs, &ctx).expect("Node evaluation should succeed");
        assert_eq!(result.get("valid"), Some(&MetaValue::Bool(true)));
    }

    #[test]
    fn test_api_node_requests() {
        let node = ApiNode;
        let ctx = create_test_context();

        // Test GET request
        let inputs = create_test_inputs(&[
            ("url", MetaValue::String("https://api.example.com/data".to_string())),
            ("method", MetaValue::String("GET".to_string())),
        ]);
        let result = node.evaluate(&inputs, &ctx).expect("Node evaluation should succeed");
        
        assert!(result.contains_key("response"));
        assert!(result.contains_key("status"));
        assert!(result.contains_key("success"));

        // Test POST request with body
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), MetaValue::String("application/json".to_string()));

        let inputs = create_test_inputs(&[
            ("url", MetaValue::String("https://api.example.com/create".to_string())),
            ("method", MetaValue::String("POST".to_string())),
            ("headers", MetaValue::Object(headers)),
            ("body", MetaValue::String("{\"name\":\"test\"}".to_string())),
        ]);
        let result = node.evaluate(&inputs, &ctx).expect("Node evaluation should succeed");
        
        // Should return mock response for testing
        assert!(result.contains_key("response"));
    }

    #[test]
    fn test_data_transform_node_operations() {
        let node = DataTransformNode;
        let ctx = create_test_context();

        // Test filter operation
        let data = vec![
            MetaValue::Scalar(1.0),
            MetaValue::Scalar(2.0),
            MetaValue::Scalar(3.0),
            MetaValue::Scalar(4.0),
            MetaValue::Scalar(5.0),
        ];

        let mut operations = HashMap::new();
        operations.insert("filter".to_string(), MetaValue::String("> 3".to_string()));

        let inputs = create_test_inputs(&[
            ("data", MetaValue::List(data)),
            ("operations", MetaValue::Object(operations)),
        ]);
        let result = node.evaluate(&inputs, &ctx).expect("Node evaluation should succeed");
        
        if let Some(MetaValue::List(filtered)) = result.get("result") {
            assert_eq!(filtered.len(), 2); // Should have 4.0 and 5.0
        }

        // Test sort operation
        let data = vec![
            MetaValue::Scalar(3.0),
            MetaValue::Scalar(1.0),
            MetaValue::Scalar(4.0),
            MetaValue::Scalar(2.0),
        ];

        let mut operations = HashMap::new();
        operations.insert("sort".to_string(), MetaValue::String("asc".to_string()));

        let inputs = create_test_inputs(&[
            ("data", MetaValue::List(data)),
            ("operations", MetaValue::Object(operations)),
        ]);
        let result = node.evaluate(&inputs, &ctx).expect("Node evaluation should succeed");
        
        if let Some(MetaValue::List(sorted)) = result.get("result") {
            assert_eq!(sorted[0], MetaValue::Scalar(1.0));
            assert_eq!(sorted[1], MetaValue::Scalar(2.0));
            assert_eq!(sorted[2], MetaValue::Scalar(3.0));
            assert_eq!(sorted[3], MetaValue::Scalar(4.0));
        }
    }

    #[test]
    fn test_node_error_handling() {
        let math_node = MathNode;
        let ctx = create_test_context();

        // Test invalid operation
        let inputs = create_test_inputs(&[
            ("a", MetaValue::Scalar(5.0)),
            ("operation", MetaValue::String("invalid_op".to_string())),
        ]);
        let result = math_node.evaluate(&inputs, &ctx).expect("Node evaluation should succeed");
        // Should return default value without panicking
        assert!(result.contains_key("result"));

        // Test missing inputs
        let inputs = HashMap::new();
        let result = math_node.evaluate(&inputs, &ctx).expect("Node evaluation should succeed");
        // Should handle gracefully with defaults
        assert!(result.contains_key("result"));
    }

    #[test]
    fn test_node_type_safety() {
        let string_node = StringNode;
        let ctx = create_test_context();

        // Test with wrong input type
        let inputs = create_test_inputs(&[
            ("text", MetaValue::Scalar(123.0)), // Number instead of string
            ("operation", MetaValue::String("length".to_string())),
        ]);
        let result = string_node.evaluate(&inputs, &ctx).expect("Node evaluation should succeed");
        // Should handle type mismatch gracefully
        assert!(result.contains_key("result"));
        assert!(result.contains_key("length"));
    }
}
