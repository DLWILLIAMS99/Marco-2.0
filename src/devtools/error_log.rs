use std::sync::{Arc, Mutex};
use crate::core::types::error::MarcoError;

#[derive(Debug, Clone, Default)]
pub struct ErrorLog {
    errors: Arc<Mutex<Vec<MarcoError>>>,
}

impl ErrorLog {
    pub fn new() -> Self {
        Self { errors: Arc::new(Mutex::new(Vec::new())) }
    }
    
    pub fn add(&self, err: MarcoError) {
        self.errors.lock().unwrap().push(err);
    }
    
    pub fn get_all(&self) -> Vec<MarcoError> {
        self.errors.lock().unwrap().clone()
    }
    
    pub fn get_recent_errors(&self, count: usize) -> Vec<MarcoError> {
        let errors = self.errors.lock().unwrap();
        if errors.len() <= count {
            errors.clone()
        } else {
            errors[errors.len() - count..].to_vec()
        }
    }
    
    pub fn clear(&self) {
        self.errors.lock().unwrap().clear();
    }
    
    pub fn clear_errors(&self) {
        self.clear();
    }
    
    pub fn log_error(&self, error: MarcoError) {
        self.add(error);
    }
}
