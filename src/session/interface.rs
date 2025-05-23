use std::sync::Arc;
use crate::error::{AppError, AuthError};
use crate::utils::rate_limiter::{RateLimiter, RateLimitType};

/// Session information for IG Markets API authentication
#[derive(Debug, Clone)]
pub struct IgSession {
    /// Client Session Token (CST) used for authentication
    pub cst: String,
    /// Security token used for authentication
    pub token: String,
    /// Account ID associated with the session
    pub account_id: String,
    /// Rate limiter for controlling request rates
    pub(crate) rate_limiter: Option<Arc<RateLimiter>>,
}

impl IgSession {
    /// Creates a new session with the given credentials
    pub fn new(cst: String, token: String, account_id: String) -> Self {
        Self {
            cst,
            token,
            account_id,
            rate_limiter: None,
        }
    }
    
    /// Creates a new session with the given credentials and a rate limiter
    pub fn with_rate_limiter(cst: String, token: String, account_id: String, limit_type: RateLimitType) -> Self {
        Self {
            cst,
            token,
            account_id,
            rate_limiter: Some(Arc::new(RateLimiter::new(limit_type))),
        }
    }
    
    /// Waits if necessary to respect rate limits before making a request
    /// Returns an error if the rate limit has been exceeded and cannot be respected
    pub async fn respect_rate_limit(&self) -> Result<(), AppError> {
        if let Some(limiter) = &self.rate_limiter {
            limiter.wait().await;
            Ok(())
        } else {
            // If no rate limiter is configured, proceed without waiting
            Ok(())
        }
    }
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
        default_account: Option<bool>,
    ) -> Result<IgSession, AuthError>;
}
