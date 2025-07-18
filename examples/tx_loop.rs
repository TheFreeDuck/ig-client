//! Transaction Loop Example
//!
//! This example demonstrates how to periodically fetch and store transactions
//! from the IG Markets API. It runs in a continuous loop, fetching transactions
//! at regular intervals and storing them in a PostgreSQL database.
//!
//! The example uses environment variables for configuration and implements
//! error handling with exponential backoff.

use chrono::{Duration, Utc};
use ig_client::utils::rate_limiter::RateLimitType;
use ig_client::{
    application::models::transaction::TransactionList, application::services::AccountService,
    application::services::account_service::AccountServiceImpl, config::Config,
    session::auth::IgAuth, session::interface::IgAuthenticator, storage::utils::store_transactions,
    transport::http_client::IgHttpClientImpl, utils::logger::setup_logger,
};
use std::{sync::Arc, time::Duration as StdDuration};
use tokio::{signal, time};
use tracing::{debug, error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logger();

    info!("Starting transaction loop service");

    let config = Arc::new(Config::with_rate_limit_type(
        RateLimitType::NonTradingAccount,
        0.7,
    ));
    info!(
        "Configuration: interval={} hours, page_size={}, lookback={} days",
        config.sleep_hours, config.page_size, config.days_to_look_back
    );
    debug!("Loaded config: database={}", config.database);

    // Build the Postgres pool once at startup
    let pool = match config.pg_pool().await {
        Ok(pool) => {
            info!("Postgres pool established");
            pool
        }
        Err(e) => {
            error!("Failed to establish database connection: {}", e);
            return Err(Box::<dyn std::error::Error>::from(e));
        }
    };

    // Set up signal handlers for graceful shutdown
    let ctrl_c = signal::ctrl_c();
    tokio::pin!(ctrl_c);

    let hour_interval = time::interval(StdDuration::from_secs(config.sleep_hours * 3600));
    tokio::pin!(hour_interval);

    info!(
        "Service started, will fetch transactions every {} hours",
        config.sleep_hours
    );

    // Immediately run once, then continue with the hourly interval
    loop {
        tokio::select! {
            _ = &mut ctrl_c => {
                info!("Received shutdown signal, terminating gracefully");
                break;
            }
            _ = hour_interval.tick() => {
                // If this is the first run, the interval will tick immediately
                info!("Starting scheduled transaction fetch");

                // Create HTTP client and authenticator
                let http_client = Arc::new(IgHttpClientImpl::new(Arc::clone(&config)));
                let authenticator = IgAuth::new(&config);

                // Attempt to login
                let session = match authenticator.login().await {
                    Ok(session) => {
                        info!("Session started successfully");
                        session
                    }
                    Err(e) => {
                        error!("Failed to login: {}", e);
                        continue; // Skip this iteration and try again
                    }
                };

                // Create account service
                let account_service = AccountServiceImpl::new(Arc::clone(&config), Arc::clone(&http_client));

                // Calculate date range
                let to = Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
                let from = (Utc::now() - Duration::days(config.days_to_look_back))
                    .format("%Y-%m-%dT%H:%M:%S")
                    .to_string();

                info!("Fetching transactions from {} to {}", from, to);

                // Fetch first page to get total pages
                let first_page = match account_service
                    .get_transactions(
                        &session,
                        &from,
                        &to,
                        config.page_size,
                        1, // Start with page 1
                    )
                    .await {
                    Ok(transactions) => transactions,
                    Err(e) => {
                        error!("Failed to get transactions: {}", e);
                        continue; // Skip this iteration and try again
                    }
                };

                // Calculate total pages
                let total_pages = first_page.metadata.page_data.total_pages as u32;

                info!("Found {} transactions in page 1 of {}", first_page.transactions.len(), total_pages);

                // Process first page
                let mut all_transactions = first_page.transactions;

                // Fetch remaining pages if any
                for page in 2..=total_pages {
                    info!("Fetching page {} of {}", page, total_pages);

                    // Add a small delay between requests to avoid rate limiting
                    time::sleep(StdDuration::from_millis(500)).await;

                    match account_service
                        .get_transactions(
                            &session,
                            &from,
                            &to,
                            config.page_size,
                            page,
                        )
                        .await {
                        Ok(page_data) => {
                            info!("Retrieved {} transactions from page {}", page_data.transactions.len(), page);
                            all_transactions.extend(page_data.transactions);
                        }
                        Err(e) => {
                            error!("Failed to get page {}: {}", page, e);
                            // Continue with the transactions we have so far
                            break;
                        }
                    }
                }

                info!("Total transactions fetched: {}", all_transactions.len());

                // Log transaction details at debug level
                for (i, transaction) in all_transactions.iter().enumerate() {
                    debug!(
                        "Transaction #{}: {}",
                        i + 1,
                        serde_json::to_string_pretty(&serde_json::to_value(transaction).unwrap()).unwrap()
                    );
                }

                // Convert and store transactions
                if !all_transactions.is_empty() {
                    let tx_list = TransactionList::from(&all_transactions);
                    let tx_ref = tx_list.as_ref();

                    match store_transactions(&pool, tx_ref).await {
                        Ok(inserted) => {
                            info!("Successfully stored {} transactions in database", inserted);
                        }
                        Err(e) => {
                            error!("Error storing transactions: {}", e);
                        }
                    };
                } else {
                    info!("No transactions to store for the specified period");
                }
            }
        }
    }

    info!("Service shutting down");
    Ok(())
}
