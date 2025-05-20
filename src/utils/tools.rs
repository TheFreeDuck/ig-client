use crate::constants::{ERROR_COOLDOWN_SECONDS, MAX_CONSECUTIVE_ERRORS};
use std::time::Duration;
use tokio::time;
use tracing::warn;

/// Apply exponential backoff when too many errors occur
pub async fn apply_backoff(consecutive_errors: &mut u32) {
    let backoff_time =
        ERROR_COOLDOWN_SECONDS * (2_u64.pow(*consecutive_errors - MAX_CONSECUTIVE_ERRORS));
    let capped_backoff = backoff_time.min(3600); // Cap at 1 hour max

    warn!(
        "Hit maximum consecutive errors ({}). Entering cooldown period of {} seconds",
        MAX_CONSECUTIVE_ERRORS, capped_backoff
    );

    // Pause for cooldown period
    time::sleep(Duration::from_secs(capped_backoff)).await;
    *consecutive_errors = 0; // Reset after cooldown
}
