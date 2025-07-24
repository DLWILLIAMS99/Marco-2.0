use crate::core::logic::{InputMap, OutputMap, Evaluatable, EvalContext};
use crate::core::types::MetaValue;
use crate::core::types::error::MarcoError;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DataTransformNode;

impl Evaluatable for DataTransformNode {
    fn evaluate(&self, inputs: &InputMap, _ctx: &EvalContext) -> Result<OutputMap, MarcoError> {
        let data = inputs.get("data").cloned().unwrap_or(MetaValue::List(Vec::new()));
        let operation = inputs.get("operation").and_then(|v| v.as_string()).unwrap_or("filter".to_string());
        let field = inputs.get("field").and_then(|v| v.as_string()).unwrap_or("value".to_string());
        let value = inputs.get("value").cloned().unwrap_or(MetaValue::Scalar(0.0));
        let condition = inputs.get("condition").and_then(|v| v.as_string()).unwrap_or("equals".to_string());
        
        let mut result = HashMap::new();
        
        match &data {
            MetaValue::List(items) => {
                let transformed_data = match operation.as_str() {
                    "filter" => {
                        let filtered: Vec<MetaValue> = items.iter()
                            .filter(|item| {
                                if let MetaValue::Object(obj) = item {
                                    if let Some(item_value) = obj.get(&field) {
                                        match condition.as_str() {
                                            "equals" => item_value == &value,
                                            "greater" => {
                                                if let (Some(a), Some(b)) = (item_value.as_scalar(), value.as_scalar()) {
                                                    a > b
                                                } else { false }
                                            },
                                            "less" => {
                                                if let (Some(a), Some(b)) = (item_value.as_scalar(), value.as_scalar()) {
                                                    a < b
                                                } else { false }
                                            },
                                            "contains" => {
                                                if let (Some(a), Some(b)) = (item_value.as_string(), value.as_string()) {
                                                    a.contains(&b)
                                                } else { false }
                                            },
                                            _ => false
                                        }
                                    } else { false }
                                } else { false }
                            })
                            .cloned()
                            .collect();
                        MetaValue::List(filtered)
                    },
                    "map" => {
                        let mapped: Vec<MetaValue> = items.iter()
                            .map(|item| {
                                if let MetaValue::Object(mut obj) = item.clone() {
                                    // Add a computed field
                                    obj.insert("computed".to_string(), value.clone());
                                    if let Some(existing) = obj.get(&field).and_then(|v| v.as_scalar()) {
                                        if let Some(modifier) = value.as_scalar() {
                                            obj.insert("modified".to_string(), MetaValue::Scalar(existing + modifier));
                                        }
                                    }
                                    MetaValue::Object(obj)
                                } else {
                                    item.clone()
                                }
                            })
                            .collect();
                        MetaValue::List(mapped)
                    },
                    "sort" => {
                        let mut sorted = items.clone();
                        sorted.sort_by(|a, b| {
                            if let (MetaValue::Object(obj_a), MetaValue::Object(obj_b)) = (a, b) {
                                let val_a = obj_a.get(&field).and_then(|v| v.as_scalar()).unwrap_or(0.0);
                                let val_b = obj_b.get(&field).and_then(|v| v.as_scalar()).unwrap_or(0.0);
                                val_a.partial_cmp(&val_b).unwrap_or(std::cmp::Ordering::Equal)
                            } else {
                                std::cmp::Ordering::Equal
                            }
                        });
                        MetaValue::List(sorted)
                    },
                    "group" => {
                        // Simple grouping by field value
                        let mut groups: HashMap<String, Vec<MetaValue>> = HashMap::new();
                        for item in items {
                            if let MetaValue::Object(obj) = item {
                                let group_key = obj.get(&field)
                                    .and_then(|v| v.as_string())
                                    .unwrap_or("unknown".to_string());
                                groups.entry(group_key).or_insert_with(Vec::new).push(item.clone());
                            }
                        }
                        
                        let grouped_list: Vec<MetaValue> = groups.into_iter()
                            .map(|(key, items)| {
                                let mut group_obj = HashMap::new();
                                group_obj.insert("group_key".to_string(), MetaValue::String(key));
                                group_obj.insert("items".to_string(), MetaValue::List(items.clone()));
                                group_obj.insert("count".to_string(), MetaValue::Scalar(items.len() as f64));
                                MetaValue::Object(group_obj)
                            })
                            .collect();
                        MetaValue::List(grouped_list)
                    },
                    "aggregate" => {
                        // Calculate aggregations
                        let numbers: Vec<f64> = items.iter()
                            .filter_map(|item| {
                                if let MetaValue::Object(obj) = item {
                                    obj.get(&field).and_then(|v| v.as_scalar())
                                } else {
                                    item.as_scalar()
                                }
                            })
                            .collect();
                        
                        let mut agg_obj = HashMap::new();
                        agg_obj.insert("count".to_string(), MetaValue::Scalar(numbers.len() as f64));
                        agg_obj.insert("sum".to_string(), MetaValue::Scalar(numbers.iter().sum()));
                        agg_obj.insert("avg".to_string(), MetaValue::Scalar(
                            if !numbers.is_empty() { numbers.iter().sum::<f64>() / numbers.len() as f64 } else { 0.0 }
                        ));
                        agg_obj.insert("min".to_string(), MetaValue::Scalar(
                            numbers.iter().fold(f64::INFINITY, |a, &b| a.min(b))
                        ));
                        agg_obj.insert("max".to_string(), MetaValue::Scalar(
                            numbers.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
                        ));
                        
                        MetaValue::Object(agg_obj)
                    },
                    _ => data.clone()
                };
                
                result.insert("result".to_string(), transformed_data.clone());
                
                // Statistics about the transformation
                if let MetaValue::List(ref result_items) = transformed_data {
                    result.insert("result_count".to_string(), MetaValue::Scalar(result_items.len() as f64));
                    result.insert("original_count".to_string(), MetaValue::Scalar(items.len() as f64));
                    result.insert("change_count".to_string(), MetaValue::Scalar(
                        result_items.len() as f64 - items.len() as f64
                    ));
                } else {
                    result.insert("result_count".to_string(), MetaValue::Scalar(1.0));
                    result.insert("original_count".to_string(), MetaValue::Scalar(items.len() as f64));
                }
            },
            _ => {
                result.insert("result".to_string(), data);
                result.insert("error".to_string(), MetaValue::String("Input data must be a list".to_string()));
            }
        }
        
        result.insert("operation".to_string(), MetaValue::String(operation));
        result.insert("field".to_string(), MetaValue::String(field));
        result.insert("success".to_string(), MetaValue::Bool(true));
        
        Ok(result)
    }

    fn node_type(&self) -> &'static str {
        "data_transform"
    }
}
