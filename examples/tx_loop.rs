use chrono::{Duration, Utc};
use ig_client::application::models::transaction::TransactionList;
use ig_client::application::services::AccountService;
use ig_client::application::services::account_service::AccountServiceImpl;
use ig_client::config::Config;
use ig_client::constants::DAYS_TO_BACK_LOOK;
use ig_client::session::auth::IgAuth;
use ig_client::session::interface::IgAuthenticator;
use ig_client::storage::utils::store_transactions;
use ig_client::transport::http_client::IgHttpClientImpl;
use ig_client::utils::logger::setup_logger;
use std::sync::Arc;
use std::time::Duration as StdDuration;
use tokio::signal;
use tokio::time;
use tracing::{debug, error, info, warn};

// Maximum number of consecutive errors before forcing a cooldown
const MAX_CONSECUTIVE_ERRORS: u32 = 3;
// Cooldown time in seconds when hitting max errors
const ERROR_COOLDOWN_SECONDS: u64 = 300; // 5 minutes

const SLEEP_TIME: u64 = 24; // Sleep time in hours

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logger();
    let config = Arc::new(Config::new());
    debug!("Loaded config: database={}", config.database);

    // Build the Postgres pool once at startup
    let pool = config.pg_pool().await?;
    info!("Postgres pool established");

    // Initialize error counter
    let mut consecutive_errors = 0;

    // Set up signal handlers for graceful shutdown
    let ctrl_c = signal::ctrl_c();
    tokio::pin!(ctrl_c);

    let hour_interval = time::interval(StdDuration::from_secs(SLEEP_TIME * 3600));
    tokio::pin!(hour_interval);

    info!("Service started, will fetch transactions hourly");

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

                let http_client = Arc::new(IgHttpClientImpl::new(Arc::clone(&config)));
                let authenticator = IgAuth::new(&config);
                let session = match authenticator.login().await {
                    Ok(session) => {
                        info!("Session started successfully");
                        session
                    }
                    Err(e) => {
                        error!("Failed to login: {}", e);
                        consecutive_errors += 1;
                        continue; // Skip this iteration and try again
                    }
                };

                let account_service = AccountServiceImpl::new(Arc::clone(&config), Arc::clone(&http_client));

                let to = format!( "{}" ,Utc::now().format("%Y-%m-%dT%H:%M:%S"));
                let from_date = Utc::now() - Duration::days(DAYS_TO_BACK_LOOK);
                let from = format!( "{}" ,from_date.format("%Y-%m-%dT%H:%M:%S"));

                let mut transactions = match account_service
                    .get_transactions(
                        &session,
                        &from,
                        &to,
                        0,
                        1,
                    )
                    .await{
                    Ok(transactions) => transactions,
                    Err(e) => {
                        error!("Failed to get transactions: {}", e);
                        consecutive_errors += 1;
                        return Err(Box::<dyn std::error::Error>::from(format!(
                            "Failed to get transactions: {}",
                            e
                        )));
                    }
                };
                if transactions.transactions.is_empty() {
                    info!("No open transactions currently");
                } else {
                    info!("Open transactions: {}", transactions.transactions.len());

                    for (i, transaction) in transactions.transactions.iter_mut().enumerate() {
                        // Log the transaction as pretty JSON
                        debug!(
                            "Transactions #{}: {}",
                            i + 1,
                            serde_json::to_string_pretty(&serde_json::to_value(transaction).unwrap()).unwrap()
                        );
                    }
                }

                let tx = TransactionList::from(&transactions.transactions);
                let tx = tx.as_ref();
                let inserted = match store_transactions(&pool, tx).await {
                    Ok(inserted) => {
                        info!("Successfully processed {} transactions", inserted);
                        consecutive_errors = 0; // Reset error counter on success
                        inserted
                    }
                    Err(e) => {
                        error!("Error processing transactions: {}", e);
                        consecutive_errors += 1;
                        continue; // Skip this iteration and try again
                    }
            };
                info!("Inserted {} rows", inserted);


            }
        }
        if consecutive_errors >= MAX_CONSECUTIVE_ERRORS {
            warn!(
                "Hit maximum consecutive errors ({}). Entering cooldown period of {} seconds",
                MAX_CONSECUTIVE_ERRORS, ERROR_COOLDOWN_SECONDS
            );

            // Pause for cooldown period
            time::sleep(StdDuration::from_secs(ERROR_COOLDOWN_SECONDS)).await;
            consecutive_errors = 0; // Reset after cooldown
        }
    }

    info!("Service shutting down");
    Ok(())
}
