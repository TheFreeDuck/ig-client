// Authentication module for IG Markets API

use async_trait::async_trait;
use reqwest::{Client, StatusCode};

use crate::{
    config::Config,
    error::AuthError,
    session::interface::{IgAuthenticator, IgSession},
    session::response::{AccountSwitchRequest, AccountSwitchResponse, SessionResp},
};

/// Authentication handler for IG Markets API
pub struct IgAuth<'a> {
    pub(crate) cfg: &'a Config,
    http: Client,
}

impl<'a> IgAuth<'a> {
    /// Creates a new IG authentication handler
    ///
    /// # Arguments
    /// * `cfg` - Reference to the configuration
    ///
    /// # Returns
    /// * A new IgAuth instance
    pub fn new(cfg: &'a Config) -> Self {
        Self {
            cfg,
            http: Client::builder()
                .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0 Safari/537.36")
                .build()
                .expect("reqwest client"),
        }
    }

    /// Returns the correct base URL (demo vs live) according to the configuration
    fn rest_url(&self, path: &str) -> String {
        format!(
            "{}/{}",
            self.cfg.rest_api.base_url.trim_end_matches('/'),
            path.trim_start_matches('/')
        )
    }

    /// Retrieves a reference to the `Client` instance.
    ///
    /// This method returns a reference to the `Client` object,
    /// which is typically used for making HTTP requests or interacting
    /// with other network-related services.
    ///
    /// # Returns
    ///
    /// * `&Client` - A reference to the internally stored `Client` object.
    ///
    #[allow(dead_code)]
    fn get_client(&self) -> &Client {
        &self.http
    }
}

#[async_trait]
impl IgAuthenticator for IgAuth<'_> {
    async fn login(&self) -> Result<IgSession, AuthError> {
        // Following the exact approach from trading-ig Python library
        let url = self.rest_url("session");

        // Ensure the API key is trimmed and has no whitespace
        let api_key = self.cfg.credentials.api_key.trim();
        let username = self.cfg.credentials.username.trim();
        let password = self.cfg.credentials.password.trim();

        // Log the request details for debugging
        tracing::info!("Login request to URL: {}", url);
        tracing::info!("Using API key (length): {}", api_key.len());
        tracing::info!("Using username: {}", username);

        // Create the body exactly as in the Python library
        let body = serde_json::json!({
            "identifier": username,
            "password": password,
            "encryptedPassword": false
        });

        tracing::debug!(
            "Request body: {}",
            serde_json::to_string(&body).unwrap_or_default()
        );

        // Create a new client for each request to avoid any potential issues with cached state
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0 Safari/537.36")
            .build()
            .expect("reqwest client");

        // Add headers exactly as in the Python library
        let resp = client
            .post(url)
            .header("X-IG-API-KEY", api_key)
            .header("Content-Type", "application/json; charset=UTF-8")
            .header("Accept", "application/json; charset=UTF-8")
            .header("Version", "2")
            .json(&body)
            .send()
            .await?;

        // Log the response status and headers for debugging
        tracing::info!("Login response status: {}", resp.status());
        tracing::debug!("Response headers: {:#?}", resp.headers());

        match resp.status() {
            StatusCode::OK => {
                // Extract CST and X-SECURITY-TOKEN from headers
                let cst = match resp.headers().get("CST") {
                    Some(value) => {
                        let cst_str = value
                            .to_str()
                            .map_err(|_| AuthError::Unexpected(StatusCode::OK))?;
                        tracing::info!(
                            "Successfully obtained CST token of length: {}",
                            cst_str.len()
                        );
                        cst_str.to_owned()
                    }
                    None => {
                        tracing::error!("CST header not found in response");
                        return Err(AuthError::Unexpected(StatusCode::OK));
                    }
                };

                let token = match resp.headers().get("X-SECURITY-TOKEN") {
                    Some(value) => {
                        let token_str = value
                            .to_str()
                            .map_err(|_| AuthError::Unexpected(StatusCode::OK))?;
                        tracing::info!(
                            "Successfully obtained X-SECURITY-TOKEN of length: {}",
                            token_str.len()
                        );
                        token_str.to_owned()
                    }
                    None => {
                        tracing::error!("X-SECURITY-TOKEN header not found in response");
                        return Err(AuthError::Unexpected(StatusCode::OK));
                    }
                };

                // Parse the response body to get the account ID
                let json: SessionResp = resp.json().await?;
                tracing::info!("Account ID: {}", json.account_id);

                Ok(IgSession {
                    cst,
                    token,
                    account_id: json.account_id,
                })
            }
            StatusCode::UNAUTHORIZED => {
                tracing::error!("Authentication failed with UNAUTHORIZED");
                let body = resp
                    .text()
                    .await
                    .unwrap_or_else(|_| "Could not read response body".to_string());
                tracing::error!("Response body: {}", body);
                Err(AuthError::BadCredentials)
            }
            StatusCode::FORBIDDEN => {
                tracing::error!("Authentication failed with FORBIDDEN");
                let body = resp
                    .text()
                    .await
                    .unwrap_or_else(|_| "Could not read response body".to_string());
                tracing::error!("Response body: {}", body);
                Err(AuthError::BadCredentials)
            }
            other => {
                tracing::error!("Authentication failed with unexpected status: {}", other);
                let body = resp
                    .text()
                    .await
                    .unwrap_or_else(|_| "Could not read response body".to_string());
                tracing::error!("Response body: {}", body);
                Err(AuthError::Unexpected(other))
            }
        }
    }

    async fn refresh(&self, sess: &IgSession) -> Result<IgSession, AuthError> {
        let url = self.rest_url("session/refresh-token");

        // Ensure the API key is trimmed and has no whitespace
        let api_key = self.cfg.credentials.api_key.trim();

        // Log the request details for debugging
        tracing::info!("Refresh request to URL: {}", url);
        tracing::info!("Using API key (length): {}", api_key.len());
        tracing::info!("Using CST token (length): {}", sess.cst.len());
        tracing::info!("Using X-SECURITY-TOKEN (length): {}", sess.token.len());

        // Create a new client for each request to avoid any potential issues with cached state
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0 Safari/537.36")
            .build()
            .expect("reqwest client");

        let resp = client
            .post(url)
            .header("X-IG-API-KEY", api_key)
            .header("CST", &sess.cst)
            .header("X-SECURITY-TOKEN", &sess.token)
            .header("Version", "3")
            .header("Content-Type", "application/json; charset=UTF-8")
            .header("Accept", "application/json; charset=UTF-8")
            .send()
            .await?;

        // Log the response status and headers for debugging
        tracing::info!("Refresh response status: {}", resp.status());
        tracing::debug!("Response headers: {:#?}", resp.headers());

        match resp.status() {
            StatusCode::OK => {
                // Extract CST and X-SECURITY-TOKEN from headers
                let cst = match resp.headers().get("CST") {
                    Some(value) => {
                        let cst_str = value
                            .to_str()
                            .map_err(|_| AuthError::Unexpected(StatusCode::OK))?;
                        tracing::info!(
                            "Successfully obtained refreshed CST token of length: {}",
                            cst_str.len()
                        );
                        cst_str.to_owned()
                    }
                    None => {
                        tracing::error!("CST header not found in refresh response");
                        return Err(AuthError::Unexpected(StatusCode::OK));
                    }
                };

                let token = match resp.headers().get("X-SECURITY-TOKEN") {
                    Some(value) => {
                        let token_str = value
                            .to_str()
                            .map_err(|_| AuthError::Unexpected(StatusCode::OK))?;
                        tracing::info!(
                            "Successfully obtained refreshed X-SECURITY-TOKEN of length: {}",
                            token_str.len()
                        );
                        token_str.to_owned()
                    }
                    None => {
                        tracing::error!("X-SECURITY-TOKEN header not found in refresh response");
                        return Err(AuthError::Unexpected(StatusCode::OK));
                    }
                };

                // Parse the response body to get the account ID
                let json: SessionResp = resp.json().await?;
                tracing::info!("Refreshed session for Account ID: {}", json.account_id);

                Ok(IgSession {
                    cst,
                    token,
                    account_id: json.account_id,
                })
            }
            other => {
                tracing::error!("Session refresh failed with status: {}", other);
                let body = resp
                    .text()
                    .await
                    .unwrap_or_else(|_| "Could not read response body".to_string());
                tracing::error!("Response body: {}", body);
                Err(AuthError::Unexpected(other))
            }
        }
    }

    async fn switch_account(
        &self,
        session: &IgSession,
        account_id: &str,
        default_account: Option<bool>,
    ) -> Result<IgSession, AuthError> {
        // Check if the account to switch to is the same as the current one
        if session.account_id == account_id {
            tracing::info!("Already on account ID: {}. No need to switch.", account_id);
            // Return a copy of the current session
            return Ok(IgSession {
                cst: session.cst.clone(),
                token: session.token.clone(),
                account_id: session.account_id.clone(),
            });
        }

        let url = self.rest_url("session");

        // Ensure the API key is trimmed and has no whitespace
        let api_key = self.cfg.credentials.api_key.trim();

        // Log the request details for debugging
        tracing::info!("Account switch request to URL: {}", url);
        tracing::info!("Using API key (length): {}", api_key.len());
        tracing::info!("Switching to account ID: {}", account_id);
        tracing::info!("Set as default account: {:?}", default_account);

        // Create the request body
        let body = AccountSwitchRequest {
            account_id: account_id.to_string(),
            default_account,
        };

        tracing::debug!(
            "Request body: {}",
            serde_json::to_string(&body).unwrap_or_default()
        );

        // Create a new client for each request
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0 Safari/537.36")
            .build()
            .expect("reqwest client");

        // Make the PUT request to switch accounts
        let resp = client
            .put(url)
            .header("X-IG-API-KEY", api_key)
            .header("CST", &session.cst)
            .header("X-SECURITY-TOKEN", &session.token)
            .header("Version", "1")
            .header("Content-Type", "application/json; charset=UTF-8")
            .header("Accept", "application/json; charset=UTF-8")
            .json(&body)
            .send()
            .await?;

        // Log the response status and headers for debugging
        tracing::info!("Account switch response status: {}", resp.status());
        tracing::debug!("Response headers: {:#?}", resp.headers());

        match resp.status() {
            StatusCode::OK => {
                // Parse the response body
                let switch_response: AccountSwitchResponse = resp.json().await?;
                tracing::info!("Account switch successful");
                tracing::debug!("Account switch response: {:?}", switch_response);

                // Return a new session with the updated account ID
                // The CST and token remain the same
                Ok(IgSession {
                    cst: session.cst.clone(),
                    token: session.token.clone(),
                    account_id: account_id.to_string(),
                })
            }
            other => {
                tracing::error!("Account switch failed with status: {}", other);
                let body = resp
                    .text()
                    .await
                    .unwrap_or_else(|_| "Could not read response body".to_string());
                tracing::error!("Response body: {}", body);

                // Si el error es 401 Unauthorized, podría ser que el ID de cuenta no sea válido
                // o no pertenezca al usuario autenticado
                if other == StatusCode::UNAUTHORIZED {
                    tracing::warn!(
                        "Cannot switch to account ID: {}. The account might not exist or you don't have permission.",
                        account_id
                    );
                }

                Err(AuthError::Unexpected(other))
            }
        }
    }
}
