// src/session/ig_auth.rs  (o donde te encaje)

use async_trait::async_trait;
use reqwest::{Client, StatusCode};

use crate::{
    config::Config,   // <─ tu struct de antes
    error::AuthError, // mismo enum/impl que ya usas
    session::interface::{IgAuthenticator, IgSession},
    session::response::SessionResp,
};

/// Mantiene una referencia a la Config global
pub struct IgAuth<'a> {
    cfg: &'a Config,
    http: Client,
}

impl<'a> IgAuth<'a> {
    pub fn new(cfg: &'a Config) -> Self {
        Self {
            cfg,
            http: Client::builder()
                .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0 Safari/537.36")
                .build()
                .expect("reqwest client"),
        }
    }

    /// Devuelve la URL base correcta (demo vs live) según la config
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
}
