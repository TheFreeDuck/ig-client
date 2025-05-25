use ig_client::application::models::market::MarketNode;
use ig_client::application::services::market_service::MarketServiceImpl;
use ig_client::presentation::build_market_hierarchy;
use ig_client::utils::logger::setup_logger;
use ig_client::{
    application::models::market::MarketData,
    config::Config,
    session::auth::IgAuth,
    session::interface::IgAuthenticator,
    transport::http_client::IgHttpClientImpl,
};
use std::{error::Error, sync::Arc};
use tracing::{error, info};
use ig_client::utils::market_parser::parse_instrument_name;

/// Constants for formatting the table
const EPIC_WIDTH: usize = 30;
const NAME_WIDTH: usize = 50;
const EXPIRY_WIDTH: usize = 20;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Configure logger
    setup_logger();

    // Load configuration from environment variables
    let config = Arc::new(Config::new());
    info!("Loaded configuration → {}", config.rest_api.base_url);

    // Create HTTP client
    let client = Arc::new(IgHttpClientImpl::new(config.clone()));

    // Create market service
    let market_service = MarketServiceImpl::new(config.clone(), client);

    // Create authenticator
    let auth = IgAuth::new(&config);

    // Login
    info!("Logging in...");
    let session = match auth.login().await {
        Ok(session) => {
            info!("Login successful. Account ID: {}", session.account_id);
            session
        }
        Err(e) => {
            error!("Login failed: {:?}", e);
            return Err(Box::new(e) as Box<dyn Error>);
        }
    };

    // Create a directory for the output file if it doesn't exist
    std::fs::create_dir_all("Data").map_err(|e| Box::new(e) as Box<dyn Error>)?;

    // Check if we need to switch accounts
    let session = if !config.credentials.account_id.is_empty()
        && session.account_id != config.credentials.account_id
    {
        info!("Switching to account: {}", config.credentials.account_id);
        match auth
            .switch_account(&session, &config.credentials.account_id, Some(true))
            .await
        {
            Ok(new_session) => {
                info!("✅ Switched to account: {}", new_session.account_id);
                new_session
            }
            Err(e) => {
                error!(
                    "Could not switch to account {}: {:?}. Continuing with current account.",
                    config.credentials.account_id, e
                );
                session
            }
        }
    } else {
        session
    };

    // Build the complete market hierarchy
    info!("Building market hierarchy...");
    let hierarchy = match build_market_hierarchy(&market_service, &session, None, 0).await {
        Ok(h) => {
            info!("Successfully built hierarchy with {} top-level nodes", h.len());
            h
        }
        Err(e) => {
            error!("Error building complete hierarchy: {:?}", e);
            return Err(Box::new(e) as Box<dyn Error>);
        }
    };

    // Extract all markets from the hierarchy into a flat list
    let markets = extract_markets_from_hierarchy(&hierarchy);
    info!("Extracted {} markets from the hierarchy", markets.len());

    // Save the complete data to a JSON file
    let json_data = markets
        .iter()
        .map(|market| {
            let parsed_info = parse_instrument_name(&market.instrument_name);
            match serde_json::to_value(parsed_info) {
                Ok(value) => value,
                Err(e) => {
                    error!("Failed to serialize parsed market data: {:?}", e);
                    return serde_json::json!({});
                }
            }
        })
        .collect::<Vec<_>>();

    let json = match serde_json::to_string_pretty(&json_data) {
        Ok(json) => json,
        Err(e) => {
            error!("Failed to serialize to JSON: {:?}", e);
            return Err(Box::new(e) as Box<dyn Error>);
        }
    };
    
    let filename = "Data/market_table.json";
    if let Err(e) = std::fs::write(&filename, &json) {
        error!("Failed to write to file {}: {:?}", filename, e);
        return Err(Box::new(e) as Box<dyn Error>);
    }
    info!("Complete market data saved to '{}'", filename);

    // Print the table header
    println!(
        "\n{:<width_epic$} {:<width_name$} {:<width_expiry$}",
        "EPIC", "INSTRUMENT NAME", "EXPIRY",
        width_epic = EPIC_WIDTH,
        width_name = NAME_WIDTH,
        width_expiry = EXPIRY_WIDTH
    );
    println!(
        "{:-<width_epic$} {:-<width_name$} {:-<width_expiry$}",
        "", "", "",
        width_epic = EPIC_WIDTH,
        width_name = NAME_WIDTH,
        width_expiry = EXPIRY_WIDTH
    );

    // Sort markets by instrument name for better readability
    let mut sorted_markets = markets;
    sorted_markets.sort_by(|a, b| a.instrument_name.to_lowercase().cmp(&b.instrument_name.to_lowercase()));

    // Print the table rows
    for market in sorted_markets {
        println!(
            "{:<width_epic$} {:<width_name$} {:<width_expiry$}",
            truncate(&market.epic, EPIC_WIDTH - 2),
            truncate(&market.instrument_name, NAME_WIDTH - 2),
            truncate(&market.expiry, EXPIRY_WIDTH - 2),
            width_epic = EPIC_WIDTH,
            width_name = NAME_WIDTH,
            width_expiry = EXPIRY_WIDTH
        );
    }

    info!("Market table generated successfully");
    Ok(())
}

/// Recursively extract all markets from the hierarchy into a flat list
fn extract_markets_from_hierarchy(nodes: &[MarketNode]) -> Vec<MarketData> {
    let mut all_markets = Vec::new();

    for node in nodes {
        // Add markets from this node
        all_markets.extend(node.markets.clone());

        // Recursively add markets from child nodes
        if !node.children.is_empty() {
            all_markets.extend(extract_markets_from_hierarchy(&node.children));
        }
    }

    all_markets
}

/// Helper function to truncate strings to a maximum length
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[0..max_len - 3])
    }
}
