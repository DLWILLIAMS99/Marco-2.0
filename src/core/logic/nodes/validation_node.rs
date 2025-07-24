use crate::core::logic::{InputMap, OutputMap, Evaluatable, EvalContext};
use crate::core::types::MetaValue;
use crate::core::types::error::MarcoError;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ValidationNode;

impl Evaluatable for ValidationNode {
    fn evaluate(&self, inputs: &InputMap, _ctx: &EvalContext) -> Result<OutputMap, MarcoError> {
        let input_value = inputs.get("value").cloned().unwrap_or(MetaValue::String("".to_string()));
        let validation_type = inputs.get("type").and_then(|v| v.as_string()).unwrap_or("text".to_string());
        let min_length = inputs.get("min_length").and_then(|v| v.as_scalar()).unwrap_or(0.0) as usize;
        let max_length = inputs.get("max_length").and_then(|v| v.as_scalar()).unwrap_or(1000.0) as usize;
        let required = inputs.get("required").and_then(|v| v.as_bool()).unwrap_or(false);
        let _pattern = inputs.get("pattern").and_then(|v| v.as_string()).unwrap_or_default();
        
        let mut result = HashMap::new();
        let mut is_valid = true;
        let mut errors = Vec::new();
        
        match &input_value {
            MetaValue::String(text) => {
                // Length validation
                if text.len() < min_length {
                    is_valid = false;
                    errors.push(MetaValue::String(format!("Minimum length is {}", min_length)));
                }
                if text.len() > max_length {
                    is_valid = false;
                    errors.push(MetaValue::String(format!("Maximum length is {}", max_length)));
                }
                
                // Required validation
                if required && text.is_empty() {
                    is_valid = false;
                    errors.push(MetaValue::String("Field is required".to_string()));
                }
                
                // Type-specific validation
                match validation_type.as_str() {
                    "email" => {
                        if !text.contains('@') || !text.contains('.') {
                            is_valid = false;
                            errors.push(MetaValue::String("Invalid email format".to_string()));
                        }
                    },
                    "number" => {
                        if text.parse::<f64>().is_err() {
                            is_valid = false;
                            errors.push(MetaValue::String("Must be a number".to_string()));
                        }
                    },
                    "url" => {
                        if !text.starts_with("http://") && !text.starts_with("https://") {
                            is_valid = false;
                            errors.push(MetaValue::String("Must be a valid URL".to_string()));
                        }
                    },
                    "phone" => {
                        let digits_only: String = text.chars().filter(|c| c.is_ascii_digit()).collect();
                        if digits_only.len() < 10 {
                            is_valid = false;
                            errors.push(MetaValue::String("Phone number too short".to_string()));
                        }
                    },
                    _ => {} // "text" or unknown types don't have additional validation
                }
                
                result.insert("length".to_string(), MetaValue::Scalar(text.len() as f64));
                result.insert("is_empty".to_string(), MetaValue::Bool(text.is_empty()));
            },
            MetaValue::Scalar(num) => {
                result.insert("is_positive".to_string(), MetaValue::Bool(*num > 0.0));
                result.insert("is_negative".to_string(), MetaValue::Bool(*num < 0.0));
                result.insert("is_zero".to_string(), MetaValue::Bool(num.abs() < f64::EPSILON));
                result.insert("is_integer".to_string(), MetaValue::Bool(num.fract() == 0.0));
            },
            _ => {
                if required {
                    is_valid = false;
                    errors.push(MetaValue::String("Invalid data type".to_string()));
                }
            }
        }
        
        result.insert("is_valid".to_string(), MetaValue::Bool(is_valid));
        result.insert("errors".to_string(), MetaValue::List(errors.clone()));
        result.insert("error_count".to_string(), MetaValue::Scalar(errors.len() as f64));
        result.insert("validated_value".to_string(), input_value.clone());
        
        // Sanitized output (basic cleaning)
        let sanitized = match &input_value {
            MetaValue::String(text) => {
                let cleaned = text.trim().replace('\n', " ").replace('\r', "");
                MetaValue::String(cleaned)
            },
            other => other.clone(),
        };
        result.insert("sanitized".to_string(), sanitized);
        
        // Summary message
        let message = if is_valid {
            "Validation passed".to_string()
        } else {
            format!("Validation failed: {} error(s)", errors.len())
        };
        result.insert("message".to_string(), MetaValue::String(message));
        
        Ok(result)
    }

    fn node_type(&self) -> &'static str {
        "validation"
    }
}
