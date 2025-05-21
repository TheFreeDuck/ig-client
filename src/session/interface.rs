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
    /// Switches the active account for the current session
    /// 
    /// # Arguments
    /// * `session` - The current session
    /// * `account_id` - The ID of the account to switch to
    /// * `default_account` - Whether to set this account as the default (optional)
    /// 
    /// # Returns
    /// * A new session with the updated account ID
    async fn switch_account(
        &self,
        session: &IgSession,
        account_id: &str,
        default_account: Option<bool>
    ) -> Result<IgSession, AuthError>;
}
