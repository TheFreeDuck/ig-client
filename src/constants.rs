pub const DAYS_TO_BACK_LOOK: i64 = 10;
// Maximum number of consecutive errors before forcing a cooldown
pub const MAX_CONSECUTIVE_ERRORS: u32 = 3;
// Cooldown time in seconds when hitting max errors
pub const ERROR_COOLDOWN_SECONDS: u64 = 300; // 5 minutes

// Default sleep time in hours if not specified in environment
pub const DEFAULT_SLEEP_TIME: u64 = 24; // 24 hours

// Default page size for transaction fetching
pub const DEFAULT_PAGE_SIZE: u32 = 50; // Maximum allowed by IG API
