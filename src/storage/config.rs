use serde::Deserialize;
use std::fmt;

/// Configuration for database connections
#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    /// Database connection URL
    pub url: String,
    /// Maximum number of connections in the connection pool
    pub max_connections: u32,
}

impl fmt::Display for DatabaseConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\"url\":\"{}\",\"max_connections\":{}}}",
            self.url, self.max_connections
        )
    }
}
