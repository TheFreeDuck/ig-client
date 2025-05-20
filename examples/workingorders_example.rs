use ig_client::application::services::AccountService;
use ig_client::{
    application::services::account_service::AccountServiceImpl, config::Config,
    session::auth::IgAuth, session::interface::IgAuthenticator,
    transport::http_client::IgHttpClientImpl, utils::logger::setup_logger,
};
use std::sync::Arc;
use tracing::info;

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

    // No need for delay anymore

    // Create account service
    let account_service = AccountServiceImpl::new(Arc::clone(&config), Arc::clone(&http_client));
    info!("Account service created");

    // Get working orders
    info!("Fetching working orders...");
    let working_orders = account_service.get_working_orders(&session).await?;

    if working_orders.working_orders.is_empty() {
        info!("No working orders currently");
    } else {
        info!("Working orders: {}", working_orders.working_orders.len());

        // Display details of each working order as JSON
        for (i, order) in working_orders.working_orders.iter().enumerate() {
            // Log the working order as pretty JSON
            info!(
                "Working Order #{}: {}",
                i + 1,
                serde_json::to_string_pretty(&serde_json::to_value(order).unwrap()).unwrap()
            );
        }
    }

    Ok(())
}
