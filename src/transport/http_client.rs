use async_trait::async_trait;
use reqwest::{Client, Method, RequestBuilder, Response, StatusCode};
use serde::{Serialize, de::DeserializeOwned};
use std::sync::Arc;
use tracing::{debug, error, info, warn};

use crate::utils::rate_limiter::app_non_trading_limiter;

use crate::{config::Config, error::AppError, session::interface::IgSession};

/// Interface for the IG HTTP client
#[async_trait]
pub trait IgHttpClient: Send + Sync {
    /// Makes an HTTP request to the IG API
    async fn request<T, R>(
        &self,
        method: Method,
        path: &str,
        session: &IgSession,
        body: Option<&T>,
        version: &str,
    ) -> Result<R, AppError>
    where
        for<'de> R: DeserializeOwned + 'static,
        T: Serialize + Send + Sync + 'static;

    /// Makes an unauthenticated HTTP request (for login)
    async fn request_no_auth<T, R>(
        &self,
        method: Method,
        path: &str,
        body: Option<&T>,
        version: &str,
    ) -> Result<R, AppError>
    where
        for<'de> R: DeserializeOwned + 'static,
        T: Serialize + Send + Sync + 'static;
}

/// Implementation of the HTTP client for IG
pub struct IgHttpClientImpl {
    config: Arc<Config>,
    client: Client,
}

impl IgHttpClientImpl {
    /// Creates a new instance of the HTTP client
    pub fn new(config: Arc<Config>) -> Self {
        let client = Client::builder()
            .user_agent("ig-client/0.1.0")
            .timeout(std::time::Duration::from_secs(config.rest_api.timeout))
            .build()
            .expect("Failed to create HTTP client");

        Self { config, client }
    }

    /// Builds the complete URL for a request
    fn build_url(&self, path: &str) -> String {
        format!(
            "{}/{}",
            self.config.rest_api.base_url.trim_end_matches('/'),
            path.trim_start_matches('/')
        )
    }

    /// Adds common headers to all requests
    fn add_common_headers(&self, builder: RequestBuilder, version: &str) -> RequestBuilder {
        builder
            .header("X-IG-API-KEY", &self.config.credentials.api_key)
            .header("Content-Type", "application/json; charset=UTF-8")
            .header("Accept", "application/json; charset=UTF-8")
            .header("Version", version)
    }

    /// Adds authentication headers to a request
    fn add_auth_headers(&self, builder: RequestBuilder, session: &IgSession) -> RequestBuilder {
        builder
            .header("CST", &session.cst)
            .header("X-SECURITY-TOKEN", &session.token)
    }

    /// Processes the HTTP response
    async fn process_response<R>(&self, response: Response) -> Result<R, AppError>
    where
        R: DeserializeOwned,
    {
        let status = response.status();
        let url = response.url().to_string();

        match status {
            StatusCode::OK | StatusCode::CREATED | StatusCode::ACCEPTED => {
                // Clone the response to get the raw body for debugging
                let response_bytes = response.bytes().await?;
                let response_text = String::from_utf8_lossy(&response_bytes);
                debug!("Raw response from {}: {}", url, response_text);

                // Try to deserialize the response
                match serde_json::from_slice::<R>(&response_bytes) {
                    Ok(json) => {
                        debug!("Request to {} successfully deserialized", url);
                        Ok(json)
                    }
                    Err(e) => {
                        error!("Failed to deserialize response from {}: {}", url, e);
                        error!("Response body: {}", response_text);
                        Err(AppError::Deserialization(format!(
                            "Failed to deserialize response: {}",
                            e
                        )))
                    }
                }
            }
            StatusCode::UNAUTHORIZED => {
                error!("Unauthorized request to {}", url);
                Err(AppError::Unauthorized)
            }
            StatusCode::NOT_FOUND => {
                error!("Resource not found at {}", url);
                Err(AppError::NotFound)
            }
            StatusCode::TOO_MANY_REQUESTS => {
                error!("Rate limit exceeded for {}", url);
                Err(AppError::RateLimitExceeded)
            }
            StatusCode::FORBIDDEN => {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                if error_text.contains("exceeded-api-key-allowance") {
                    error!("Rate Limit Exceeded to {}: {}", url, error_text);
                    return Err(AppError::RateLimitExceeded);
                }
                error!("Forbidden request to {}: {}", url, error_text);
                Err(AppError::Unauthorized)
            }
            _ => {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                error!(
                    "Request to {} failed with status {}: {}",
                    url, status, error_text
                );
                Err(AppError::Unexpected(status))
            }
        }
    }
}

#[async_trait]
impl IgHttpClient for IgHttpClientImpl {
    async fn request<T, R>(
        &self,
        method: Method,
        path: &str,
        session: &IgSession,
        body: Option<&T>,
        version: &str,
    ) -> Result<R, AppError>
    where
        for<'de> R: DeserializeOwned + 'static,
        T: Serialize + Send + Sync + 'static,
    {
        let url = self.build_url(path);
        let method_str = method.as_str().to_string(); // Store method as string for logging
        debug!("Making {} request to {}", method_str, url);

        // Respect rate limits before making the request
        session.respect_rate_limit().await?;

        let mut builder = self.client.request(method, &url);
        builder = self.add_common_headers(builder, version);
        builder = self.add_auth_headers(builder, session);

        if let Some(data) = body {
            builder = builder.json(data);
        }

        let response = builder.send().await?;
        let result = self.process_response::<R>(response).await;
        
        // If we get a rate limit error, we'll return it directly
        // The caller can implement retry logic if needed
        if let Err(AppError::RateLimitExceeded) = &result {
            error!("Rate limit exceeded for {} request to {}", method_str, url);
        }
        
        result
    }

    async fn request_no_auth<T, R>(
        &self,
        method: Method,
        path: &str,
        body: Option<&T>,
        version: &str,
    ) -> Result<R, AppError>
    where
        for<'de> R: DeserializeOwned + 'static,
        T: Serialize + Send + Sync + 'static,
    {
        let url = self.build_url(path);
        let method_str = method.as_str().to_string(); // Store method as string for logging
        info!("Making unauthenticated {} request to {}", method_str, url);

        // Use the global app rate limiter for unauthenticated requests
        let limiter = app_non_trading_limiter();
        limiter.wait().await;

        let mut builder = self.client.request(method, &url);
        builder = self.add_common_headers(builder, version);

        if let Some(data) = body {
            builder = builder.json(data);
        }

        let response = builder.send().await?;
        let result = self.process_response::<R>(response).await;
        
        // If we get a rate limit error, we'll return it directly
        // The caller can implement retry logic if needed
        if let Err(AppError::RateLimitExceeded) = &result {
            warn!("Rate limit exceeded for unauthenticated {} request to {}", method_str, url);
        }
        
        result
    }
}
