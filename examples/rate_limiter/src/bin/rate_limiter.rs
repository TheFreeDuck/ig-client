use futures::future::join_all;
use ig_client::{
    config::Config,
    error::AppError,
    session::auth::IgAuth,
    session::interface::{IgAuthenticator, IgSession},
    utils::logger::setup_logger,
    utils::rate_limiter::{RateLimitType, RateLimiterStats},
};
use std::env;
use std::{error::Error, sync::Arc, time::Duration};
use tokio::{sync::Mutex, time::sleep};
use tracing::{debug, info, warn};

/// Example to test the rate limiter functionality in a concurrent environment
///
/// This example demonstrates how the rate limiter works in a multi-threaded environment by:
/// 1. Creating a configuration with a specific rate limit type
/// 2. Sharing a session across multiple concurrent tasks
/// 3. Making multiple API requests concurrently from different tasks
/// 4. Showing how the rate limiter prevents hitting API limits even with concurrent requests
/// 5. Displaying statistics about the rate limiter usage
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Setup logger with debug level
    unsafe {
        env::set_var("LOGLEVEL", "DEBUG");
    }
    setup_logger();

    // Create a configuration with a specific rate limit type and a smaller safety margin
    // to better demonstrate the rate limiter behavior
    let config = Config::with_rate_limit_type(RateLimitType::NonTradingAccount, 0.7);
    let config = Arc::new(config);

    // Display the rate limit configuration
    info!(
        "Using rate limit type: {:?} with safety margin: {}",
        config.rate_limit_type, config.rate_limit_safety_margin
    );
    info!(
        "This means we'll use {}% of the actual limit",
        config.rate_limit_safety_margin * 100.0
    );

    // Create an authentication handler
    let auth = IgAuth::new(&config);

    // Log in to get a session
    info!("Logging in to IG Markets API...");
    let session = auth.login().await?;
    let session = Arc::new(session);

    // Display initial rate limiter statistics
    if let Some(stats) = session.get_rate_limit_stats().await {
        display_rate_limiter_stats(&stats);
    } else {
        warn!("No rate limiter statistics available");
    }

    // Create a shared counter to track total requests
    let counter = Arc::new(Mutex::new(0));

    // Create a shared statistics tracker
    let stats_tracker = Arc::new(Mutex::new(Vec::new()));

    // Number of concurrent tasks to run
    let num_tasks = 5;
    // Number of requests per task
    let requests_per_task = 10;
    // Total number of requests
    let total_requests = num_tasks * requests_per_task;

    info!(
        "Starting {} concurrent tasks with {} requests each (total: {})",
        num_tasks, requests_per_task, total_requests
    );

    // Create multiple tasks that will run concurrently
    let mut tasks = Vec::new();
    for task_id in 1..=num_tasks {
        // Clone the Arc references for this task
        let session_clone = Arc::clone(&session);
        let counter_clone = Arc::clone(&counter);
        let stats_tracker_clone = Arc::clone(&stats_tracker);

        // Create a new task
        let task = tokio::spawn(async move {
            for req_id in 1..=requests_per_task {
                // Update the shared counter
                let current_req;
                {
                    let mut counter = counter_clone.lock().await;
                    *counter += 1;
                    current_req = *counter;
                }

                info!(
                    "Task {} making request {}/{} (global request: {}/{})",
                    task_id, req_id, requests_per_task, current_req, total_requests
                );

                // Make the request and respect rate limits
                match make_test_request(&session_clone).await {
                    Ok(stats) => {
                        // Store the statistics
                        let mut stats_vec = stats_tracker_clone.lock().await;
                        stats_vec.push((task_id, req_id, current_req, stats));
                    }
                    Err(e) => {
                        warn!("Task {} request {} failed: {}", task_id, req_id, e);
                    }
                }

                // Add a small random delay to simulate varying processing times
                let delay = rand::random::<u64>() % 100;
                sleep(Duration::from_millis(delay)).await;
            }
            info!(
                "Task {} completed all {} requests",
                task_id, requests_per_task
            );
        });

        tasks.push(task);
    }

    // Wait for all tasks to complete
    info!("Waiting for all tasks to complete...");
    join_all(tasks).await;

    // Display final statistics
    info!("All tasks completed. Displaying statistics...");
    let stats_vec = stats_tracker.lock().await;

    // Create a vector of indices sorted by global request number
    let mut indices: Vec<usize> = (0..stats_vec.len()).collect();
    indices.sort_by_key(|&i| {
        let (_, _, global_req, _) = &stats_vec[i];
        *global_req
    });

    // Display statistics in order
    for &i in &indices {
        let (task_id, req_id, global_req, ref stats) = stats_vec[i];
        info!(
            "Task {} Request {}/{} (Global: {}/{}):",
            task_id, req_id, requests_per_task, global_req, total_requests
        );
        display_rate_limiter_stats(stats);
    }

    // Get final rate limiter statistics
    if let Some(stats) = session.get_rate_limit_stats().await {
        info!("Final rate limiter statistics:");
        display_rate_limiter_stats(&stats);
    }

    info!("Concurrent rate limiter test completed successfully");
    Ok(())
}

/// Makes a test request to the IG Markets API
///
/// This function simulates making an API request and returns the rate limiter statistics
async fn make_test_request(session: &IgSession) -> Result<RateLimiterStats, AppError> {
    // Respect the rate limit before making the request
    // This is where the magic happens - the rate limiter will ensure we don't exceed the limits
    // even with concurrent requests from multiple tasks
    debug!("Calling respect_rate_limit()");
    session.respect_rate_limit().await?;

    // In a real scenario, we would make an actual API request here
    // For this example, we'll just simulate a request by waiting a bit
    sleep(Duration::from_millis(50)).await;

    // Get and return the rate limiter statistics
    session
        .get_rate_limit_stats()
        .await
        .ok_or_else(|| AppError::InvalidInput("No rate limiter statistics available".to_string()))
}

/// Displays rate limiter statistics in a formatted way
fn display_rate_limiter_stats(stats: &RateLimiterStats) {
    info!("Rate Limiter Statistics:");
    info!("  Type: {:?}", stats.limit_type);
    info!(
        "  Current requests: {}/{}",
        stats.request_count, stats.effective_limit
    );
    info!("  Usage: {:.1}%", stats.usage_percent);

    if stats.usage_percent > 80.0 {
        warn!(
            "  High usage detected: {:.1}% of limit",
            stats.usage_percent
        );
    } else if stats.usage_percent > 50.0 {
        info!("  Moderate usage: {:.1}% of limit", stats.usage_percent);
    } else {
        info!("  Low usage: {:.1}% of limit", stats.usage_percent);
    }
}
