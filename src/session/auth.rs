// Authentication module for IG Markets API

use crate::{
    config::Config,
    error::AuthError,
    session::interface::{IgAuthenticator, IgSession},
    session::response::{AccountSwitchRequest, AccountSwitchResponse, SessionResp},
    utils::rate_limiter::app_non_trading_limiter,
};
use async_trait::async_trait;
use rand;
use reqwest::{Client, StatusCode};
use std::time::Duration;
use tracing::{debug, error, info, warn};

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
        // Configuración para reintentos
        const MAX_RETRIES: u32 = 3;
        const INITIAL_RETRY_DELAY_MS: u64 = 10000; // 10 segundos

        
        let mut retry_count = 0;
        let mut retry_delay_ms = INITIAL_RETRY_DELAY_MS;

        loop {
            // Use the global app rate limiter for unauthenticated requests
            let limiter = app_non_trading_limiter();
            limiter.wait().await;

            // Following the exact approach from trading-ig Python library
            let url = self.rest_url("session");

            // Ensure the API key is trimmed and has no whitespace
            let api_key = self.cfg.credentials.api_key.trim();
            let username = self.cfg.credentials.username.trim();
            let password = self.cfg.credentials.password.trim();

            // Log the request details for debugging
            info!("Login request to URL: {}", url);
            info!("Using API key (length): {}", api_key.len());
            info!("Using username: {}", username);

            if retry_count > 0 {
                info!("Retry attempt {} of {}", retry_count, MAX_RETRIES);
            }

            // Create the body exactly as in the Python library
            let body = serde_json::json!({
                "identifier": username,
                "password": password,
                "encryptedPassword": false
            });

            debug!(
                "Request body: {}",
                serde_json::to_string(&body).unwrap_or_default()
            );

            // Create a new client for each request to avoid any potential issues with cached state
            let client = Client::builder()
                .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0 Safari/537.36")
                .build()
                .expect("reqwest client");

            // Add headers exactly as in the Python library
            let resp = match client
                .post(url.clone())
                .header("X-IG-API-KEY", api_key)
                .header("Content-Type", "application/json; charset=UTF-8")
                .header("Accept", "application/json; charset=UTF-8")
                .header("Version", "2")
                .json(&body)
                .send()
                .await
            {
                Ok(resp) => resp,
                Err(e) => {
                    error!("Failed to send login request: {}", e);
                    return Err(AuthError::Unexpected(StatusCode::INTERNAL_SERVER_ERROR));
                }
            };

            // Log the response status and headers for debugging
            info!("Login response status: {}", resp.status());
            debug!("Response headers: {:#?}", resp.headers());

            match resp.status() {
                StatusCode::OK => {
                    // Extract CST and X-SECURITY-TOKEN from headers
                    let cst = match resp.headers().get("CST") {
                        Some(value) => {
                            let cst_str = value
                                .to_str()
                                .map_err(|_| AuthError::Unexpected(StatusCode::OK))?;
                            info!(
                                "Successfully obtained CST token of length: {}",
                                cst_str.len()
                            );
                            cst_str.to_owned()
                        }
                        None => {
                            error!("CST header not found in response");
                            return Err(AuthError::Unexpected(StatusCode::OK));
                        }
                    };

                    let token = match resp.headers().get("X-SECURITY-TOKEN") {
                        Some(value) => {
                            let token_str = value
                                .to_str()
                                .map_err(|_| AuthError::Unexpected(StatusCode::OK))?;
                            info!(
                                "Successfully obtained X-SECURITY-TOKEN of length: {}",
                                token_str.len()
                            );
                            token_str.to_owned()
                        }
                        None => {
                            error!("X-SECURITY-TOKEN header not found in response");
                            return Err(AuthError::Unexpected(StatusCode::OK));
                        }
                    };

                    // Extract account ID from the response
                    let json: SessionResp = resp.json().await?;
                    let account_id = json.account_id.clone();

                    // Return a new session with the CST, token, and account ID
                    // Use the rate limit type and safety margin from the config
                    let session =
                        IgSession::from_config(cst.clone(), token.clone(), account_id, self.cfg);

                    // Log rate limiter stats if available
                    if let Some(stats) = session.get_rate_limit_stats().await {
                        debug!("Rate limiter initialized: {}", stats);
                    }

                    return Ok(session);
                }
                StatusCode::UNAUTHORIZED => {
                    error!("Authentication failed with UNAUTHORIZED");
                    let body = resp
                        .text()
                        .await
                        .unwrap_or_else(|_| "Could not read response body".to_string());
                    error!("Response body: {}", body);
                    return Err(AuthError::BadCredentials);
                }
                StatusCode::FORBIDDEN => {
                    error!("Authentication failed with FORBIDDEN");
                    let body = resp
                        .text()
                        .await
                        .unwrap_or_else(|_| "Could not read response body".to_string());

                    if body.contains("exceeded-api-key-allowance") {
                        error!("Rate Limit Exceeded: {}", &body);

                        // Implementamos reintento con espera exponencial para este caso específico
                        if retry_count < MAX_RETRIES {
                            retry_count += 1;
                            // Usamos un retraso más largo y añadimos un poco de aleatoriedad para evitar patrones
                            let jitter = rand::random::<u64>() % 5000; // Hasta 5 segundos de jitter
                            let delay = retry_delay_ms + jitter;
                            warn!(
                                "Rate limit exceeded. Retrying in {} ms (attempt {} of {})",
                                delay, retry_count, MAX_RETRIES
                            );

                            // Esperar antes de reintentar
                            tokio::time::sleep(Duration::from_millis(delay)).await;

                            // Aumentar el tiempo de espera exponencialmente para el próximo reintento
                            retry_delay_ms *= 2; // Exponential backoff
                            continue;
                        } else {
                            error!(
                                "Maximum retry attempts ({}) reached. Giving up.",
                                MAX_RETRIES
                            );
                            return Err(AuthError::RateLimitExceeded);
                        }
                    }

                    error!("Response body: {}", body);
                    return Err(AuthError::BadCredentials);
                }
                other => {
                    error!("Authentication failed with unexpected status: {}", other);
                    let body = resp
                        .text()
                        .await
                        .unwrap_or_else(|_| "Could not read response body".to_string());
                    error!("Response body: {}", body);
                    return Err(AuthError::Unexpected(other));
                }
            }
        }
    }

    async fn refresh(&self, sess: &IgSession) -> Result<IgSession, AuthError> {
        let url = self.rest_url("session/refresh-token");

        // Ensure the API key is trimmed and has no whitespace
        let api_key = self.cfg.credentials.api_key.trim();

        // Log the request details for debugging
        info!("Refresh request to URL: {}", url);
        info!("Using API key (length): {}", api_key.len());
        info!("Using CST token (length): {}", sess.cst.len());
        info!("Using X-SECURITY-TOKEN (length): {}", sess.token.len());

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
        info!("Refresh response status: {}", resp.status());
        tracing::debug!("Response headers: {:#?}", resp.headers());

        match resp.status() {
            StatusCode::OK => {
                // Extract CST and X-SECURITY-TOKEN from headers
                let cst = match resp.headers().get("CST") {
                    Some(value) => {
                        let cst_str = value
                            .to_str()
                            .map_err(|_| AuthError::Unexpected(StatusCode::OK))?;
                        info!(
                            "Successfully obtained refreshed CST token of length: {}",
                            cst_str.len()
                        );
                        cst_str.to_owned()
                    }
                    None => {
                        error!("CST header not found in refresh response");
                        return Err(AuthError::Unexpected(StatusCode::OK));
                    }
                };

                let token = match resp.headers().get("X-SECURITY-TOKEN") {
                    Some(value) => {
                        let token_str = value
                            .to_str()
                            .map_err(|_| AuthError::Unexpected(StatusCode::OK))?;
                        info!(
                            "Successfully obtained refreshed X-SECURITY-TOKEN of length: {}",
                            token_str.len()
                        );
                        token_str.to_owned()
                    }
                    None => {
                        error!("X-SECURITY-TOKEN header not found in refresh response");
                        return Err(AuthError::Unexpected(StatusCode::OK));
                    }
                };

                // Parse the response body to get the account ID
                let json: SessionResp = resp.json().await?;
                info!("Refreshed session for Account ID: {}", json.account_id);

                // Return a new session with the updated tokens
                Ok(IgSession::from_config(
                    cst,
                    token,
                    json.account_id,
                    self.cfg,
                ))
            }
            other => {
                error!("Session refresh failed with status: {}", other);
                let body = resp
                    .text()
                    .await
                    .unwrap_or_else(|_| "Could not read response body".to_string());
                error!("Response body: {}", body);
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
            info!("Already on account ID: {}. No need to switch.", account_id);
            // Return a copy of the current session with the same rate limiter configuration
            return Ok(IgSession::from_config(
                session.cst.clone(),
                session.token.clone(),
                session.account_id.clone(),
                self.cfg,
            ));
        }

        let url = self.rest_url("session");

        // Ensure the API key is trimmed and has no whitespace
        let api_key = self.cfg.credentials.api_key.trim();

        // Log the request details for debugging
        info!("Account switch request to URL: {}", url);
        info!("Using API key (length): {}", api_key.len());
        info!("Switching to account ID: {}", account_id);
        info!("Set as default account: {:?}", default_account);

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
        info!("Account switch response status: {}", resp.status());
        tracing::debug!("Response headers: {:#?}", resp.headers());

        match resp.status() {
            StatusCode::OK => {
                // Parse the response body
                let switch_response: AccountSwitchResponse = resp.json().await?;
                info!("Account switch successful");
                tracing::debug!("Account switch response: {:?}", switch_response);

                // Return a new session with the updated account ID and the config's rate limiter settings
                // The CST and token remain the same
                Ok(IgSession::from_config(
                    session.cst.clone(),
                    session.token.clone(),
                    account_id.to_string(),
                    self.cfg,
                ))
            }
            other => {
                error!("Account switch failed with status: {}", other);
                let body = resp
                    .text()
                    .await
                    .unwrap_or_else(|_| "Could not read response body".to_string());
                error!("Response body: {}", body);

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
