// examples/websocket_example.rs
//
// Example of using the WebSocket client to subscribe to market and account updates

use std::sync::Arc;
use tokio::time::Duration;
use tracing::{info, error};

use ig_client::{
    config::Config, session::auth::IgAuth, session::interface::IgAuthenticator,
    transport::websocket_client::IgWebSocketClientImpl, utils::logger::setup_logger,
};
use ig_client::transport::ws_interface::IgWebSocketClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up logging
    setup_logger();

    // Load configuration
    let config = Arc::new(Config::new());
    info!("Configuration loaded");
    info!("API URL: {}", config.rest_api.base_url);
    info!("Environment: {}", if config.rest_api.base_url.contains("demo") { "DEMO" } else { "PRODUCTION" });
    
    // Print credential information for debugging (without exposing sensitive data)
    info!("Username: {}", config.credentials.username);
    info!("API Key length: {}", config.credentials.api_key.len());
    info!("API Key first 3 chars: {}", if config.credentials.api_key.len() > 3 {
        &config.credentials.api_key[..3]
    } else {
        "too short"
    });
    
    // Check if we're using default values (which would indicate .env issues)
    if config.credentials.username == "default_username" || 
       config.credentials.api_key == "default_api_key" ||
       config.credentials.password == "default_password" {
        error!("WARNING: Using default credentials! This indicates your .env file is not being loaded properly.");
        error!("Please ensure your .env file exists and contains IG_USERNAME, IG_PASSWORD, and IG_API_KEY.");
    }

    // Create authenticator and log in
    let authenticator = IgAuth::new(&config);
    info!("Authenticator created");

    info!("Logging in to IG...");
    let session = match authenticator.login().await {
        Ok(session) => {
            info!("Session started successfully");
            info!("Account ID: {}", session.account_id);
            info!("CST Token length: {}", session.cst.len());
            info!("Security Token length: {}", session.token.len());
            session
        },
        Err(e) => {
            eprintln!("Failed to log in: {}", e);
            eprintln!("Please check your credentials in the .env file");
            eprintln!("Make sure your IG_USERNAME, IG_PASSWORD, and IG_API_KEY are correct");
            return Err(Box::new(e) as Box<dyn std::error::Error>);
        }
    };

    // Create WebSocket client
    let ws_client = IgWebSocketClientImpl::new(Arc::clone(&config));
    info!("WebSocket client created");

    // Connect to WebSocket server
    info!("Connecting to WebSocket server...");
    match ws_client.connect(&session).await {
        Ok(_) => info!("Connected to WebSocket server"),
        Err(e) => {
            eprintln!("Failed to connect to WebSocket server: {}", e);
            eprintln!("This could be due to invalid credentials or network issues");
            return Err(Box::new(e) as Box<dyn std::error::Error>);
        }
    };

    // Get receivers for updates
    let mut market_rx = ws_client.market_updates();
    let mut account_rx = ws_client.account_updates();

    // Subscribe to market updates for some popular markets
    // You can replace these with markets you're interested in
    let markets = vec![
        "CS.D.EURUSD.MINI.IP", // EUR/USD
        "IX.D.DAX.IFMM.IP",    // DAX
        "IX.D.FTSE.IFMM.IP",   // FTSE 100
    ];

    for market in &markets {
        info!("Subscribing to market: {}", market);
        let subscription_id = match ws_client.subscribe_market(market).await {
            Ok(id) => {
                info!("Subscribed with ID: {}", id);
                id
            },
            Err(e) => {
                error!("Failed to subscribe to market {}: {}", market, e);
                continue;
            }
        };

    }

    // Subscribe to account updates
    info!("Subscribing to account updates");
    let account_sub_id = match ws_client.subscribe_account().await {
        Ok(id) => {
            info!("Subscribed to account updates with ID: {}", id);
            id
        },
        Err(e) => {
            error!("Failed to subscribe to account updates: {}", e);
            String::new() // Empty string as fallback
        }
    };

    // Process updates for 60 seconds
    info!("Listening for updates for 60 seconds...");

    let start_time = std::time::Instant::now();
    let duration = Duration::from_secs(60);

    while start_time.elapsed() < duration {
        tokio::select! {
            // Process market updates
            Some(update) = market_rx.recv() => {
                info!("Market update received: {:?}", update);
            }

            // Process account updates
            Some(update) = account_rx.recv() => {
                info!("Account update received: {:?}", update);
            }

            // Wait a bit to avoid busy-waiting
            _ = tokio::time::sleep(Duration::from_millis(100)) => {}
        }
    }

    // Unsubscribe and disconnect
    for market in &markets {
        info!("Unsubscribing from market: {}", market);
        // Note: In a real application, you would store the subscription IDs
        // Here we're just demonstrating the concept
    }

    info!("Unsubscribing from account updates");
    if !account_sub_id.is_empty() {
        if let Err(e) = ws_client.unsubscribe(&account_sub_id).await {
            error!("Failed to unsubscribe from account updates: {}", e);
        } else {
            info!("Unsubscribed from account updates");
        }
    }

    info!("Disconnecting from WebSocket server");
    if let Err(e) = ws_client.disconnect().await {
        error!("Failed to disconnect from WebSocket server: {}", e);
    } else {
        info!("Disconnected");
    }
    info!("Disconnected");

    Ok(()) as Result<(), Box<dyn std::error::Error>>
}
