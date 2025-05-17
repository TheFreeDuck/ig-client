use crate::error::AuthError;

/// Session information for IG Markets API authentication
#[derive(Debug, Clone)]
pub struct IgSession {
    /// Client Session Token (CST) used for authentication
    pub cst: String,
    /// Security token used for authentication
    pub token: String,
    /// Account ID associated with the session
    pub account_id: String,
}

/// Trait for authenticating with the IG Markets API
#[async_trait::async_trait]
pub trait IgAuthenticator: Send + Sync {
    /// Logs in to the IG Markets API and returns a new session
    async fn login(&self) -> Result<IgSession, AuthError>;
    /// Refreshes an existing session with the IG Markets API
    async fn refresh(&self, session: &IgSession) -> Result<IgSession, AuthError>;
}
