// Rate limiter for integration tests
// This module provides utilities to prevent hitting API rate limits

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::info;

/// Rate limiter type for different API endpoints
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RateLimitType {
    /// Non-trading requests (per-account): 30 per minute
    NonTradingAccount,
    /// Trading requests (per-account): 100 per minute
    TradingAccount,
    /// Non-trading requests (per-app): 60 per minute
    NonTradingApp,
    /// Historical price data: 10,000 points per week
    HistoricalPrice,
}

impl RateLimitType {
    /// Gets the minimum interval in milliseconds for this rate limit type
    pub fn min_interval_ms(&self) -> u64 {
        match self {
            // 30 requests per minute = 1 request per 2 seconds
            // Using a more conservative 1 request per 4 seconds to avoid hitting limits
            Self::NonTradingAccount => 4000,
            // 100 requests per minute = 1 request per 600ms
            // Using a more conservative 1 request per 2 seconds
            Self::TradingAccount => 2000,
            // 60 requests per minute = 1 request per second
            // Using a more conservative 1 request per 3 seconds
            Self::NonTradingApp => 3000,
            // For historical price data, we'll use a very conservative limit
            // 10,000 points per week = ~1 request per 60 seconds
            Self::HistoricalPrice => 120000, // 2 minutes between requests
        }
    }
}

/// Singleton rate limiter for API calls
pub struct RateLimiter {
    last_call: AtomicU64,
    limit_type: RateLimitType,
}

impl RateLimiter {
    /// Creates a new rate limiter with the specified rate limit type
    pub fn new(limit_type: RateLimitType) -> Self {
        Self {
            last_call: AtomicU64::new(0),
            limit_type,
        }
    }

    /// Waits if necessary to respect the rate limit
    pub async fn wait(&self) {
        let now = Instant::now().elapsed().as_millis() as u64;
        let last = self.last_call.load(Ordering::Acquire);
        let min_interval_ms = self.limit_type.min_interval_ms();

        if last > 0 && now - last < min_interval_ms {
            let wait_time = min_interval_ms - (now - last);
            info!(
                "Rate limiter ({:?}): waiting for {}ms",
                self.limit_type, wait_time
            );
            sleep(Duration::from_millis(wait_time)).await;
        }

        self.last_call.store(
            Instant::now().elapsed().as_millis() as u64,
            Ordering::Release,
        );
    }
}

/// Global rate limiter for non-trading account requests (30 per minute)
pub fn account_non_trading_limiter() -> Arc<RateLimiter> {
    static INSTANCE: once_cell::sync::Lazy<Arc<RateLimiter>> =
        once_cell::sync::Lazy::new(|| Arc::new(RateLimiter::new(RateLimitType::NonTradingAccount)));

    INSTANCE.clone()
}

/// Global rate limiter for trading account requests (100 per minute)
pub fn account_trading_limiter() -> Arc<RateLimiter> {
    static INSTANCE: once_cell::sync::Lazy<Arc<RateLimiter>> =
        once_cell::sync::Lazy::new(|| Arc::new(RateLimiter::new(RateLimitType::TradingAccount)));

    INSTANCE.clone()
}

/// Global rate limiter for non-trading app requests (60 per minute)
pub fn app_non_trading_limiter() -> Arc<RateLimiter> {
    static INSTANCE: once_cell::sync::Lazy<Arc<RateLimiter>> =
        once_cell::sync::Lazy::new(|| Arc::new(RateLimiter::new(RateLimitType::NonTradingApp)));

    INSTANCE.clone()
}

/// Global rate limiter for historical price data requests (10,000 points per week)
pub fn historical_price_limiter() -> Arc<RateLimiter> {
    static INSTANCE: once_cell::sync::Lazy<Arc<RateLimiter>> =
        once_cell::sync::Lazy::new(|| Arc::new(RateLimiter::new(RateLimitType::HistoricalPrice)));

    INSTANCE.clone()
}

/// Default global rate limiter (uses the most conservative limit: non-trading account)
pub fn global_rate_limiter() -> Arc<RateLimiter> {
    account_non_trading_limiter()
}

/// Macro to mark tests that should be run individually to avoid rate limiting
#[macro_export]
macro_rules! rate_limited_test {
    (fn $name:ident() $body:block) => {
        #[test]
        #[ignore]
        fn $name() $body
    };
}
