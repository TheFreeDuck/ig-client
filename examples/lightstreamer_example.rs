use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;
use ig_client::config::Config;
use ig_client::session::auth::IgAuth;
use ig_client::session::interface::IgAuthenticator;
use ig_client::transport::lightstreamer_client::LightstreamerClientImpl;
use ig_client::transport::lightstreamer_interface::IgWebLSClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    
    // Load configuration from environment
    dotenv::dotenv().ok();
    let config = Arc::new(Config::new());
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
            error!("Failed to log in: {}", e);
            error!("Please check your credentials in the .env file");
            error!("Make sure your IG_USERNAME, IG_PASSWORD, and IG_API_KEY are correct");
            return Err(Box::new(e) as Box<dyn std::error::Error>);
        }
    };
    info!("Login successful");
    
    // Create a Lightstreamer client
    let ls_client = LightstreamerClientImpl::new(config.clone());
    
    // Connect to the Lightstreamer server
    info!("Connecting to Lightstreamer server...");
    ls_client.connect(&session).await?;
    info!("Connected to Lightstreamer server");
    
    // Get a receiver for market updates
    let mut market_rx = ls_client.market_updates();
    
    // Get a receiver for account updates
    let mut account_rx = ls_client.account_updates();
    
    // Subscribe to market updates for a few popular markets
    let markets = vec![
        "CS.D.EURUSD.CFD.IP", // EUR/USD
        "CS.D.USDJPY.CFD.IP", // USD/JPY
        "CS.D.GBPUSD.CFD.IP", // GBP/USD
        "IX.D.NASDAQ.IFD.IP", // NASDAQ
        "IX.D.DAX.IFD.IP",    // DAX
    ];
    
    for market in &markets {
        info!("Subscribing to market updates for {}", market);
        let subscription_id = ls_client.subscribe_market(market).await?;
        info!("Subscribed to market updates with ID: {}", subscription_id);
    }
    
    // Subscribe to account updates
    info!("Subscribing to account updates");
    let account_subscription_id = ls_client.subscribe_account().await?;
    info!("Subscribed to account updates with ID: {}", account_subscription_id);
    
    // Process updates for a while
    info!("Processing updates for 60 seconds...");
    let start_time = std::time::Instant::now();
    let timeout = Duration::from_secs(60);
    
    loop {
        tokio::select! {
            Some(market_update) = market_rx.recv() => {
                info!("Market update: {} - Bid: {}, Offer: {}", market_update.epic, market_update.bid, market_update.offer);
            }
            Some(account_update) = account_rx.recv() => {
                info!("Account update: {} - Type: {}, Data: {}", account_update.account_id, account_update.update_type, account_update.data);
            }
            _ = sleep(Duration::from_millis(100)) => {
                if start_time.elapsed() > timeout {
                    break;
                }
            }
        }
    }
    
    // Unsubscribe from all subscriptions
    info!("Unsubscribing from account updates");
    ls_client.unsubscribe(&account_subscription_id).await?;
    
    // Disconnect from the Lightstreamer server
    info!("Disconnecting from Lightstreamer server...");
    ls_client.disconnect().await?;
    info!("Disconnected from Lightstreamer server");
    
    Ok(())
}
