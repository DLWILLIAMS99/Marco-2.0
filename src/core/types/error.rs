use thiserror::Error;

#[derive(Debug, Error, Clone, serde::Serialize, serde::Deserialize)]
pub enum MarcoError {
    #[error("Registry error: {0}")]
    Registry(String),
    #[error("Node evaluation error: {0}")]
    NodeEval(String),
    #[error("UI error: {0}")]
    UI(String),
    #[error("Persistence error: {0}")]
    Persistence(String),
    #[error("Template not found: {0}")]
    TemplateNotFound(String),
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<Box<dyn std::error::Error>> for MarcoError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        MarcoError::Unknown(err.to_string())
    }
}
