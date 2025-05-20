use std::sync::Arc;
use tracing::{error, info};
use ig_client::application::services::AccountService;
use ig_client::{
    application::services::account_service::AccountServiceImpl, config::Config,
    session::auth::IgAuth, session::interface::IgAuthenticator,
    transport::http_client::IgHttpClientImpl, utils::logger::setup_logger,
};

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

    // Get open positions
    info!("Fetching open positions...");
    let mut activity = match account_service
        .get_activity(
            &session,
            "2025-03-01T00:00:00Z",
            "2025-04-01T00:00:00Z",
        )
        .await {
        Ok(activity) => activity,
        Err(e) => {
            error!("Failed to get activity: {}", e);
            return Err(Box::<dyn std::error::Error>::from(format!(
                "Failed to get activity: {}",
                e
            )));
        }
        
    };

    if activity.activities.is_empty() {
        info!("No open positions currently");
    } else {
        info!("Open positions: {}", activity.activities.len());

        // Display positions
        for (i, position) in activity.activities.iter_mut().enumerate() {
            // Log the position as pretty JSON
            info!(
                "Transactions #{}: {}",
                i + 1,
                serde_json::to_string_pretty(&serde_json::to_value(position).unwrap()).unwrap()
            );
        }
    }

    Ok(())
}
