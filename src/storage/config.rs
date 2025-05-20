use serde::{Deserialize, Serialize};
use crate::impl_json_display;

/// Configuration for database connections
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseConfig {
    /// Database connection URL
    pub url: String,
    /// Maximum number of connections in the connection pool
    pub max_connections: u32,
}

impl_json_display!(DatabaseConfig);