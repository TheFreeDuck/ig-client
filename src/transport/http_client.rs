use async_trait::async_trait;
use once_cell::sync::Lazy;
use reqwest::{Client, Method, RequestBuilder, Response, StatusCode};
use serde::{Serialize, de::DeserializeOwned};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::Semaphore;
use tracing::{debug, error, info, warn};

use crate::utils::rate_limiter::app_non_trading_limiter;
use crate::{config::Config, error::AppError, session::interface::IgSession};

// Global semaphore to limit concurrent API requests
// This ensures that we don't exceed rate limits by making too many
// concurrent requests
static API_SEMAPHORE: Lazy<Arc<Semaphore>> = Lazy::new(|| {
    Arc::new(Semaphore::new(3)) // Allow up to 3 concurrent requests
});

// Flag to indicate if we're in a rate-limited situation
static RATE_LIMITED: Lazy<Arc<AtomicBool>> = Lazy::new(|| Arc::new(AtomicBool::new(false)));

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

    /// Processes the HTTP response and handles rate limiting centrally
    async fn process_response<R>(&self, response: Response) -> Result<R, AppError>
    where
        for<'de> R: DeserializeOwned + 'static,
    {
        let status = response.status();
        let url = response.url().to_string();

        // Handle rate limiting centrally
        if status == StatusCode::TOO_MANY_REQUESTS {
            self.handle_rate_limit(&url, "TOO_MANY_REQUESTS status code")
                .await;
            return Err(AppError::RateLimitExceeded);
        }

        match status {
            StatusCode::OK | StatusCode::CREATED | StatusCode::ACCEPTED => {
                let body = response.text().await?;
                match serde_json::from_str::<R>(&body) {
                    Ok(data) => Ok(data),
                    Err(e) => {
                        error!("Error deserializing response from {}: {}", url, e);
                        error!("Response body: {}", body);
                        Err(AppError::Json(e))
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
            StatusCode::FORBIDDEN => {
                let body = response.text().await?;
                if body.contains("exceeded-api-key-allowance") {
                    self.handle_rate_limit(&url, "FORBIDDEN with exceeded-api-key-allowance")
                        .await;
                    Err(AppError::RateLimitExceeded)
                } else {
                    error!("Forbidden access to {}: {}", url, body);
                    Err(AppError::Unauthorized)
                }
            }
            _ => {
                let body = response.text().await?;
                error!(
                    "Unexpected status code {} for request to {}: {}",
                    status, url, body
                );
                Err(AppError::Unexpected(status))
            }
        }
    }

    /// Helper method to handle rate limiting
    async fn handle_rate_limit(&self, url: &str, reason: &str) {
        // Set the rate limited flag
        RATE_LIMITED.store(true, Ordering::SeqCst);
        error!("Rate limit exceeded for request to {} ({})", url, reason);

        // Notify all rate limiters about the exceeded limit
        // This will cause them to enforce a mandatory cooldown period
        let non_trading_limiter = app_non_trading_limiter();
        non_trading_limiter.notify_rate_limit_exceeded().await;

        // Schedule a task to reset the flag after a delay
        let rate_limited = RATE_LIMITED.clone();
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
            rate_limited.store(false, Ordering::SeqCst);
            info!("Rate limit flag reset after 30 second cooldown");
        });
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

        // Check if we're currently rate limited
        if RATE_LIMITED.load(Ordering::SeqCst) {
            warn!("System is currently rate limited. Adding extra delay before request.");
            // Add an extra delay if we're in a rate-limited situation
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        }

        // Acquire a permit from the semaphore to limit concurrent requests
        // This ensures we don't overwhelm the API with too many concurrent requests
        let permit = API_SEMAPHORE.acquire().await.unwrap();
        debug!(
            "Acquired API semaphore permit for {} request to {}",
            method_str, url
        );

        // Respect rate limits before making the request
        // This will handle the actual rate limiting based on request history
        session.respect_rate_limit().await?;

        let mut builder = self.client.request(method, &url);
        builder = self.add_common_headers(builder, version);
        builder = self.add_auth_headers(builder, session);

        if let Some(data) = body {
            builder = builder.json(data);
        }

        // Send the request
        let response_result = builder.send().await;

        // Check for network errors
        let response = match response_result {
            Ok(resp) => resp,
            Err(e) => {
                error!("Network error for {} request to {}: {}", method_str, url, e);
                // Release the permit before returning
                drop(permit);
                return Err(AppError::Network(e));
            }
        };

        // Process the response - rate limiting is handled inside process_response
        let result = self.process_response::<R>(response).await;

        // If the request was successful, reset the rate limited flag
        if result.is_ok() && RATE_LIMITED.load(Ordering::SeqCst) {
            RATE_LIMITED.store(false, Ordering::SeqCst);
            info!("Rate limit flag reset after successful request to {}", url);
        }

        // Release the permit (this happens automatically when _permit goes out of scope,
        // but we do it explicitly for clarity)
        drop(permit);

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

        // Check if we're currently rate limited
        if RATE_LIMITED.load(Ordering::SeqCst) {
            warn!(
                "System is currently rate limited. Adding extra delay before unauthenticated request."
            );
            // Add an extra delay if we're in a rate-limited situation
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }

        // Acquire a permit from the semaphore to limit concurrent requests
        let _permit = API_SEMAPHORE.acquire().await.unwrap();
        debug!(
            "Acquired API semaphore permit for unauthenticated {} request to {}",
            method_str, url
        );

        // Use the global app rate limiter for unauthenticated requests
        // This is thread-safe and can be called from multiple threads concurrently
        let limiter = app_non_trading_limiter();
        limiter.wait().await;

        let mut builder = self.client.request(method, &url);
        builder = self.add_common_headers(builder, version);

        if let Some(data) = body {
            builder = builder.json(data);
        }

        // Send the request
        let response_result = builder.send().await;

        // Check for network errors
        let response = match response_result {
            Ok(resp) => resp,
            Err(e) => {
                error!(
                    "Network error for unauthenticated {} request to {}: {}",
                    method_str, url, e
                );
                // Release the permit before returning
                drop(_permit);
                return Err(AppError::Network(e));
            }
        };

        // Process the response - rate limiting is handled inside process_response
        let result = self.process_response::<R>(response).await;

        // If the request was successful, reset the rate limited flag
        if result.is_ok() && RATE_LIMITED.load(Ordering::SeqCst) {
            RATE_LIMITED.store(false, Ordering::SeqCst);
            info!(
                "Rate limit flag reset after successful unauthenticated request to {}",
                url
            );
        }

        // Release the permit
        drop(_permit);

        result
    }
}
