use ig_client::{
    application::services::MarketService,
    config::Config,
    session::auth::IgAuth,
    session::interface::IgAuthenticator,
    transport::http_client::IgHttpClientImpl,
    utils::logger::setup_logger,
};
use std::{error::Error, sync::Arc};
use tracing::{error, info};
use ig_client::application::services::market_service::MarketServiceImpl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Set up logging
    setup_logger();

    // Load configuration
    let config = Arc::new(Config::new());
    info!("Loaded configuration → {}", config.rest_api.base_url);

    // Create the HTTP client
    let client = Arc::new(IgHttpClientImpl::new(config.clone()));

    // Create the authenticator
    let auth = IgAuth::new(&config);

    // Create the market service
    let market_service = MarketServiceImpl::new(config.clone(), client);

    // Login to get a session
    info!("Logging in...");
    let session = auth.login().await.map_err(|e| Box::new(e) as Box<dyn Error>)?;
    info!("Login successful. Account ID: {}", session.account_id);

    // Check if we need to switch accounts
    let session = if !config.credentials.account_id.is_empty() && session.account_id != config.credentials.account_id {
        info!("Switching to account: {}", config.credentials.account_id);
        match auth.switch_account(&session, &config.credentials.account_id, Some(true)).await {
            Ok(new_session) => {
                info!("✅ Switched to account: {}", new_session.account_id);
                new_session
            },
            Err(e) => {
                error!("Could not switch to account {}: {:?}. Continuing with current account.", 
                      config.credentials.account_id, e);
                session
            }
        }
    } else {
        session
    };

    // Get the search term from command line arguments or use a default
    let search_term = std::env::args().nth(1).unwrap_or_else(|| "Daily Germany 40".to_string());
    info!("Searching for markets matching: '{}'", search_term);

    // Search for markets
    match market_service.search_markets(&session, &search_term).await {
        Ok(result) => {
            info!("Found {} markets matching '{}'", result.markets.len(), search_term);
            
            // Display the results in a table format
            println!("\n{:<40} {:<15} {:<10} {:<10}", "INSTRUMENT NAME", "EPIC", "BID", "OFFER");
            println!("{:-<40} {:-<15} {:-<10} {:-<10}", "", "", "", "");
            
            // Guardar los resultados en JSON primero
            let json = serde_json::to_string_pretty(&result.markets)
                .map_err(|e| Box::new(e) as Box<dyn Error>)?;
                
            // Luego mostrar los resultados en la consola
            for market in &result.markets {
                println!(
                    "{:<40} {:<15} {:<10} {:<10}",
                    truncate(&market.instrument_name, 38),
                    truncate(&market.epic, 13),
                    market.bid.map(|b| b.to_string()).unwrap_or_else(|| "-".to_string()),
                    market.offer.map(|o| o.to_string()).unwrap_or_else(|| "-".to_string())
                );
            }
            let filename = format!("Data/market_search_{}.json", search_term.replace(" ", "_"));
            std::fs::write(&filename, &json)
                .map_err(|e| Box::new(e) as Box<dyn Error>)?;
            info!("Results saved to '{}'", filename);
        },
        Err(e) => {
            error!("Error searching markets: {:?}", e);
            return Err(Box::new(e) as Box<dyn Error>);
        }
    }

    Ok(())
}

// Helper function to truncate strings to a maximum length
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[0..max_len-3])
    }
}
