/// Default number of days to look back when fetching historical data
pub const DAYS_TO_BACK_LOOK: i64 = 10;
// Maximum number of consecutive errors before forcing a cooldown
/// Maximum number of consecutive errors before forcing a cooldown
pub const MAX_CONSECUTIVE_ERRORS: u32 = 3;
// Cooldown time in seconds when hitting max errors
/// Cooldown time in seconds when hitting max errors (5 minutes)
pub const ERROR_COOLDOWN_SECONDS: u64 = 300;

// Default sleep time in hours if not specified in environment
/// Default sleep time in hours if not specified in environment (24 hours)
pub const DEFAULT_SLEEP_TIME: u64 = 24;

// Default page size for transaction fetching
/// Default page size for API requests
pub const DEFAULT_PAGE_SIZE: u32 = 50;

pub const SLEEP_TIME_PER_REQUEST: u64 = 1000;