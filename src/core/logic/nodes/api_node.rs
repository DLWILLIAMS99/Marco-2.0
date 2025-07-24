use crate::core::logic::{InputMap, OutputMap, Evaluatable, EvalContext};
use crate::core::types::MetaValue;
use crate::core::types::error::MarcoError;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ApiNode;

impl Evaluatable for ApiNode {
    fn evaluate(&self, inputs: &InputMap, _ctx: &EvalContext) -> Result<OutputMap, MarcoError> {
        let url = inputs.get("url").and_then(|v| v.as_string()).unwrap_or_default();
        let method = inputs.get("method").and_then(|v| v.as_string()).unwrap_or("GET".to_string());
        let _headers = inputs.get("headers").cloned().unwrap_or(MetaValue::Object(HashMap::new()));
        let _body = inputs.get("body").cloned().unwrap_or(MetaValue::String("".to_string()));
        let timeout = inputs.get("timeout").and_then(|v| v.as_scalar()).unwrap_or(30.0);
        
        let mut result = HashMap::new();
        
        // Simulate API response (in practice, this would make real HTTP requests)
        let mock_success = !url.is_empty() && url.starts_with("http");
        
        if mock_success {
            // Simulate successful response
            let mock_response = match url.as_str() {
                url if url.contains("users") => {
                    MetaValue::List(vec![
                        MetaValue::Object({
                            let mut user = HashMap::new();
                            user.insert("id".to_string(), MetaValue::Scalar(1.0));
                            user.insert("name".to_string(), MetaValue::String("John Doe".to_string()));
                            user.insert("email".to_string(), MetaValue::String("john@example.com".to_string()));
                            user
                        })
                    ])
                },
                url if url.contains("weather") => {
                    MetaValue::Object({
                        let mut weather = HashMap::new();
                        weather.insert("temperature".to_string(), MetaValue::Scalar(22.5));
                        weather.insert("humidity".to_string(), MetaValue::Scalar(65.0));
                        weather.insert("condition".to_string(), MetaValue::String("Sunny".to_string()));
                        weather
                    })
                },
                _ => {
                    MetaValue::Object({
                        let mut generic = HashMap::new();
                        generic.insert("message".to_string(), MetaValue::String("Success".to_string()));
                        generic.insert("timestamp".to_string(), MetaValue::Scalar(1642678800.0));
                        generic
                    })
                }
            };
            
            result.insert("success".to_string(), MetaValue::Bool(true));
            result.insert("status_code".to_string(), MetaValue::Scalar(200.0));
            result.insert("response".to_string(), mock_response);
            result.insert("error".to_string(), MetaValue::String("".to_string()));
        } else {
            // Simulate error response
            result.insert("success".to_string(), MetaValue::Bool(false));
            result.insert("status_code".to_string(), MetaValue::Scalar(400.0));
            result.insert("response".to_string(), MetaValue::String("".to_string()));
            result.insert("error".to_string(), MetaValue::String("Invalid URL or request failed".to_string()));
        }
        
        // Request information
        result.insert("url".to_string(), MetaValue::String(url));
        result.insert("method".to_string(), MetaValue::String(method.to_uppercase()));
        result.insert("timeout".to_string(), MetaValue::Scalar(timeout));
        
        // Response metadata
        result.insert("response_time".to_string(), MetaValue::Scalar(150.0)); // Mock response time in ms
        result.insert("content_type".to_string(), MetaValue::String("application/json".to_string()));
        
        // Parsed response helpers
        if let MetaValue::Object(ref obj) = result.get("response").unwrap_or(&MetaValue::String("".to_string())) {
            if let Some(message) = obj.get("message") {
                result.insert("message".to_string(), message.clone());
            }
        }
        
        // Connection info
        result.insert("is_online".to_string(), MetaValue::Bool(mock_success));
        result.insert("retry_count".to_string(), MetaValue::Scalar(0.0));
        
        Ok(result)
    }

    fn node_type(&self) -> &'static str {
        "api"
    }
}
