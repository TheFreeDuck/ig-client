use crate::error::AppError;

/// Result type for listener operations that don't return a value but may return an error
pub type ListenerResult = Result<(), AppError>;
