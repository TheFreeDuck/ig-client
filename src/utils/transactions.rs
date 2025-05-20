// src/utils/transactions.rs
//
// Transaction utilities for the IG client

use chrono::{DateTime, Duration, NaiveDate, Utc};
use sqlx::PgPool;
use tracing::{debug, info};

use crate::application::models::transaction::Transaction;
use crate::constants::DAYS_TO_BACK_LOOK;
use crate::{
    application::services::ig_tx_client::{IgTxClient, IgTxFetcher},
    config::Config,
    error::AppError,
    session::auth::IgAuth,
    session::interface::IgAuthenticator,
    storage::utils::store_transactions,
};

/// Fetch transactions from IG API and store them in the database
///
/// This function handles the entire process of:
/// 1. Authenticating with IG
/// 2. Creating a transaction client
/// 3. Fetching transactions for a date range
/// 4. Storing them in the database
///
/// # Arguments
///
/// * `cfg` - The configuration object
/// * `pool` - PostgreSQL connection pool
/// * `from_days_ago` - Optional number of days to look back (defaults to 10 days)
///
/// # Returns
///
/// * `Result<usize, AppError>` - Number of transactions inserted, or an error
///
/// # Example
///
/// ```
/// use ig_client::utils::transactions::fetch_and_store_transactions;
/// use ig_client::config::Config;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
///     let cfg = Config::new();
///     let pool = cfg.pg_pool().await?;
///     
///     // Fetch transactions from the last 30 days
///     let inserted = fetch_and_store_transactions(&cfg, &pool, Some(30)).await?;
///     info!("Inserted {} transactions", inserted);
///     
///     Ok(())
/// }
/// ```
pub async fn fetch_and_store_transactions(
    cfg: &Config,
    pool: &PgPool,
    from_days_ago: Option<i64>,
) -> Result<usize, AppError> {
    // Authenticate with IG
    let auth = IgAuth::new(cfg);
    let sess = auth.login().await?;
    info!("Successfully authenticated with IG");

    // Create the transaction client
    let tx_client = IgTxClient::new(cfg);

    // Calculate date range
    let to = Utc::now();
    let from = if let Some(days) = from_days_ago {
        to - Duration::days(days)
    } else {
        to - Duration::days(DAYS_TO_BACK_LOOK)
    };

    debug!("Fetching transactions from {} to {}", from, to);
    let txs = tx_client.fetch_range(&sess, from, to).await?;
    info!("Fetched {} transactions", txs.len());

    // Store the transactions
    let inserted = store_transactions(pool, &txs).await?;
    info!("Inserted {} rows", inserted);

    Ok(inserted)
}

/// Fetch transactions for a specific date range
///
/// This is a simpler version that only fetches transactions without storing them
///
/// # Arguments
///
/// * `cfg` - The configuration object
/// * `from` - Start date
/// * `to` - End date
///
/// # Returns
///
/// * `Result<Vec<Transaction>, AppError>` - List of transactions, or an error
pub async fn fetch_transactions(
    cfg: &Config,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
) -> Result<Vec<Transaction>, AppError> {
    // Authenticate with IG
    let auth = IgAuth::new(cfg);
    let sess = auth.login().await?;
    debug!("Successfully authenticated with IG");

    // Create the transaction client
    let tx_client = IgTxClient::new(cfg);

    // Fetch transactions
    debug!("Fetching transactions from {} to {}", from, to);
    let txs = tx_client.fetch_range(&sess, from, to).await?;
    debug!("Fetched {} transactions", txs.len());

    Ok(txs)
}

/// Formats a transaction ID by combining type, date, instrument, and id with '|'
pub fn format_transaction_id(tx_type: &str, date: &str, instrument: &str, id: &str) -> String {
    format!("{}|{}|{}|{}", tx_type, date, instrument, id)
}

/// Parses a transaction ID into its components: (type, date, instrument, id)
pub fn parse_transaction_id(id_str: &str) -> (&str, &str, &str, &str) {
    let mut parts = id_str.splitn(4, '|');
    let t = parts.next().unwrap_or("");
    let d = parts.next().unwrap_or("");
    let inst = parts.next().unwrap_or("");
    let id_rest = parts.next().unwrap_or("");
    (t, d, inst, id_rest)
}

/// Extracts the transaction type component from a transaction ID
pub fn extract_transaction_type(id_str: &str) -> &str {
    parse_transaction_id(id_str).0
}

/// Extracts the date component from a transaction ID as `NaiveDate`
pub fn extract_transaction_date(id_str: &str) -> Option<NaiveDate> {
    let date_str = parse_transaction_id(id_str).1;
    if date_str.is_empty() {
        None
    } else {
        NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok()
    }
}

/// Extracts the instrument component from a transaction ID
pub fn extract_transaction_instrument(id_str: &str) -> &str {
    parse_transaction_id(id_str).2
}
