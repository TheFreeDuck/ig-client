/// Default number of days to look back when fetching historical data
pub const DAYS_TO_BACK_LOOK: i64 = 10;
/// Maximum number of consecutive errors before forcing a cooldown
pub const MAX_CONSECUTIVE_ERRORS: u32 = 3;
/// Cooldown time in seconds when hitting max errors (5 minutes)
pub const ERROR_COOLDOWN_SECONDS: u64 = 300;
/// Default sleep time in hours if not specified in environment (24 hours)
pub const DEFAULT_SLEEP_TIME: u64 = 24;
/// Default page size for API requests
pub const DEFAULT_PAGE_SIZE: u32 = 50;

// Constants for rate limiter configuration
/// Base delay in milliseconds used for proximity-based delays in the rate limiter
/// This value is used to calculate wait times when approaching rate limits
pub const BASE_DELAY_MS: u64 = 1000;

/// Additional safety buffer in milliseconds added to wait times
/// This provides extra margin to ensure rate limits are not exceeded
pub const SAFETY_BUFFER_MS: u64 = 1000;

pub const USER_AGENT: &str = "Rust-IG-Client/0.1.9";