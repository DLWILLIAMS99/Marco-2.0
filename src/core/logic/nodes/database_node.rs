use crate::core::logic::{InputMap, OutputMap, Evaluatable, EvalContext};
use crate::core::types::MetaValue;
use crate::core::types::error::MarcoError;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DatabaseNode;

impl Evaluatable for DatabaseNode {
    fn evaluate(&self, inputs: &InputMap, _ctx: &EvalContext) -> Result<OutputMap, MarcoError> {
        let operation = inputs.get("operation").and_then(|v| v.as_string()).unwrap_or("select".to_string());
        let table = inputs.get("table").and_then(|v| v.as_string()).unwrap_or("data".to_string());
        let key = inputs.get("key").and_then(|v| v.as_string()).unwrap_or_default();
        let value = inputs.get("value").cloned().unwrap_or(MetaValue::String("".to_string()));
        let query = inputs.get("query").and_then(|v| v.as_string()).unwrap_or_default();
        
        let mut result = HashMap::new();
        
        // Simulate database operations (in practice, this would connect to a real database)
        match operation.to_lowercase().as_str() {
            "select" => {
                // Simulate a select operation
                let mock_data = vec![
                    MetaValue::Object({
                        let mut obj = HashMap::new();
                        obj.insert("id".to_string(), MetaValue::Scalar(1.0));
                        obj.insert("name".to_string(), MetaValue::String("John".to_string()));
                        obj.insert("age".to_string(), MetaValue::Scalar(30.0));
                        obj
                    }),
                    MetaValue::Object({
                        let mut obj = HashMap::new();
                        obj.insert("id".to_string(), MetaValue::Scalar(2.0));
                        obj.insert("name".to_string(), MetaValue::String("Jane".to_string()));
                        obj.insert("age".to_string(), MetaValue::Scalar(25.0));
                        obj
                    }),
                ];
                
                result.insert("data".to_string(), MetaValue::List(mock_data.clone()));
                result.insert("count".to_string(), MetaValue::Scalar(mock_data.len() as f64));
                result.insert("success".to_string(), MetaValue::Bool(true));
            },
            "insert" => {
                // Simulate an insert operation
                result.insert("inserted_id".to_string(), MetaValue::Scalar(3.0));
                result.insert("success".to_string(), MetaValue::Bool(!key.is_empty()));
                result.insert("affected_rows".to_string(), MetaValue::Scalar(1.0));
            },
            "update" => {
                // Simulate an update operation
                result.insert("success".to_string(), MetaValue::Bool(!key.is_empty()));
                result.insert("affected_rows".to_string(), MetaValue::Scalar(1.0));
            },
            "delete" => {
                // Simulate a delete operation
                result.insert("success".to_string(), MetaValue::Bool(!key.is_empty()));
                result.insert("affected_rows".to_string(), MetaValue::Scalar(1.0));
            },
            _ => {
                result.insert("error".to_string(), MetaValue::String(format!("Unknown operation: {}", operation)));
                result.insert("success".to_string(), MetaValue::Bool(false));
            }
        }
        
        // Common outputs for all operations
        result.insert("operation".to_string(), MetaValue::String(operation.clone()));
        result.insert("table".to_string(), MetaValue::String(table.clone()));
        result.insert("connection_status".to_string(), MetaValue::String("connected".to_string()));
        
        // Query building helper
        let built_query = match operation.to_lowercase().as_str() {
            "select" => format!("SELECT * FROM {} WHERE {} = '{}'", table, key, 
                value.as_string().unwrap_or_default()),
            "insert" => format!("INSERT INTO {} ({}) VALUES ('{}')", table, key,
                value.as_string().unwrap_or_default()),
            "update" => format!("UPDATE {} SET {} = '{}' WHERE id = 1", table, key,
                value.as_string().unwrap_or_default()),
            "delete" => format!("DELETE FROM {} WHERE {} = '{}'", table, key,
                value.as_string().unwrap_or_default()),
            _ => query
        };
        result.insert("generated_query".to_string(), MetaValue::String(built_query));
        
        Ok(result)
    }

    fn node_type(&self) -> &'static str {
        "database"
    }
}
