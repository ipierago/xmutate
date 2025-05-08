use thiserror::Error;

#[derive(Debug, Error)]
pub enum MutateError {
    #[error("Missing required parameter: {0}")]
    MissingParam(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Unsupported operation: {0}")]
    UnsupportedOp(String),

    #[error("Mutator '{name}' failed: {reason}")]
    Mutator { name: String, reason: String },

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(#[from] serde_json::Error),
}
