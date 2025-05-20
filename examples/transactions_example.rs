use std::sync::Arc;
use tracing::{error, info};
use ig_client::application::services::AccountService;
use ig_client::{
    application::services::account_service::AccountServiceImpl, config::Config,
    session::auth::IgAuth, session::interface::IgAuthenticator,
    transport::http_client::IgHttpClientImpl, utils::logger::setup_logger,
};
use ig_client::application::models::transaction::{StoreTransaction, TransactionList};
use ig_client::storage::utils::store_transactions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logger();

    // Create configuration using the default Config implementation
    // This will read from environment variables as defined in src/config.rs
    let config = Arc::new(Config::new());
    info!("Configuration loaded");

    // Create HTTP client
    let http_client = Arc::new(IgHttpClientImpl::new(Arc::clone(&config)));
    info!("HTTP client created");

    // Create authenticator
    let authenticator = IgAuth::new(&config);
    info!("Authenticator created");

    // Login to IG
    info!("Logging in to IG...");
    let session = authenticator.login().await?;
    info!("Session started successfully");

    // Create account service
    let account_service = AccountServiceImpl::new(Arc::clone(&config), Arc::clone(&http_client));
    info!("Account service created");

    // Get open transactions
    info!("Fetching open transactions...");
    let mut transactions = match account_service
        .get_transactions(
            &session,
            "2025-03-01T00:00:00Z",
            "2025-04-01T00:00:00Z",
            0,
            1,
        )
        .await{
        Ok(transactions) => transactions,
        Err(e) => {
            error!("Failed to get transactions: {}", e);
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
            info!(
                "Transactions #{}: {}",
                i + 1,
                serde_json::to_string_pretty(&serde_json::to_value(transaction).unwrap()).unwrap()
            );
        }
    }
    let pool = config.pg_pool().await?;
    // Store the transactions
    let tx = TransactionList::from(&transactions.transactions);
    let tx = tx.as_ref();
    let inserted = store_transactions(&pool, tx).await?;
    info!("Inserted {} rows", inserted);

    Ok(())
}
