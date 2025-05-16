use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use async_trait::async_trait;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::Notify;
use tracing::{error, info, warn};
use lightstreamer_client::ls_client::{LightstreamerClient, LogType, Transport, SubscriptionRequest};
use lightstreamer_client::subscription::{Subscription as LsSubscription, Subscription, SubscriptionMode};
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::error::AppError;
use crate::session::interface::IgSession;
use crate::transport::lightstreamer_interface::IgWebLSClient;

/// Types of subscriptions available
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SubscriptionType {
    /// Market data subscription (prices, etc.)
    #[serde(rename = "MARKET")]
    Market,
    /// Account updates (positions, working orders, etc.)
    #[serde(rename = "ACCOUNT")]
    Account,
    /// Trade confirmations
    #[serde(rename = "TRADE")]
    Trade,
    /// Chart data
    #[serde(rename = "CHART")]
    Chart,
}

/// Market data update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketUpdate {
    /// Market epic
    pub epic: String,
    /// Current bid price
    pub bid: f64,
    /// Current offer price
    pub offer: f64,
    /// Timestamp of the update
    pub timestamp: String,
}

/// Account update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountUpdate {
    /// Account ID
    pub account_id: String,
    /// Update type (POSITION, ORDER, etc.)
    pub update_type: String,
    /// The data associated with the update
    pub data: serde_json::Value,
}

/// Represents a subscription to a specific market or account stream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IgSubscription {
    /// The subscription ID
    pub id: String,
    /// The type of subscription (MARKET, ACCOUNT, etc.)
    pub subscription_type: SubscriptionType,
    /// The specific item being subscribed to (e.g., market epic)
    pub item: String,
}

/// Implementation of the WebSocket client using Lightstreamer
pub struct LightstreamerClientImpl {
    /// Configuration
    config: Arc<Config>,
    /// Connection state
    connected: Arc<Mutex<bool>>,
    /// Map of active subscriptions
    subscriptions: Arc<Mutex<HashMap<String, IgSubscription>>>,
    /// Lightstreamer client
    ls_client: Arc<Mutex<Option<LightstreamerClient>>>,
    /// Map of subscription IDs to Lightstreamer subscriptions
    ls_subscriptions: Arc<Mutex<HashMap<String, LsSubscription>>>,
    /// Sender for market updates
    market_tx: Sender<MarketUpdate>,
    /// Receiver for market updates
    market_rx: Arc<Mutex<Option<Receiver<MarketUpdate>>>>,
    /// Sender for account updates
    account_tx: Sender<AccountUpdate>,
    /// Receiver for account updates
    account_rx: Arc<Mutex<Option<Receiver<AccountUpdate>>>>,
    /// Shutdown signal
    shutdown: Arc<Notify>,
}

impl LightstreamerClientImpl {
    /// Create a new Lightstreamer client
    pub fn new(config: Arc<Config>) -> Self {
        let (market_tx, market_rx) = mpsc::channel(100);
        let (account_tx, account_rx) = mpsc::channel(100);
        
        Self {
            config,
            connected: Arc::new(Mutex::new(false)),
            subscriptions: Arc::new(Mutex::new(HashMap::new())),
            ls_client: Arc::new(Mutex::new(None)),
            ls_subscriptions: Arc::new(Mutex::new(HashMap::new())),
            market_tx,
            market_rx: Arc::new(Mutex::new(Some(market_rx))),
            account_tx,
            account_rx: Arc::new(Mutex::new(Some(account_rx))),
            shutdown: Arc::new(Notify::new()),
        }
    }
    
    /// Initialize the Lightstreamer client
    async fn init_client(&self, session: &IgSession) -> Result<LightstreamerClient, AppError> {
        // Determine if we're in demo environment based on the WebSocket URL in config
        // If the URL contains 'demo', we're in demo environment
        let ws_url = &self.config.websocket.url;
        let is_demo = ws_url.contains("demo");
        
        // Determine the server address based on environment
        // According to the official documentation, we should use http:// URLs
        // The library will automatically convert them to wss:// for WebSocket connections
        let server_address = if is_demo {
            "http://demo-pushserv.marketdatasystems.com/lightstreamer"
        } else {
            "http://push.lightstreamer.com/lightstreamer"
        };
        
        // Determine the adapter set based on environment
        let adapter_set = if is_demo { "DEMO" } else { "PROD" };
        
        info!("Using Lightstreamer server: {}", server_address);
        info!("Using adapter set: {}", adapter_set);
        info!("Using account ID: {}", session.account_id.trim());
        
        // Format the password as required by IG's Lightstreamer authentication
        // This follows the format "CST-{cst}|XST-{token}" as mentioned in your memories
        let cst = session.cst.trim();
        let token = session.token.trim();
        let password = format!("CST-{}|XST-{}", cst, token);
        
        info!("Using CST token of length: {}", cst.len());
        info!("Using XST token of length: {}", token.len());
        
        // Create the Lightstreamer client
        let mut client = LightstreamerClient::new(
            Some(server_address),
            Some(adapter_set),
            Some(&session.account_id.trim()),
            Some(&password),
        ).map_err(|e| AppError::WebSocketError(format!("Failed to create Lightstreamer client: {}", e)))?;
        
        // Set logging to use tracing
        client.set_logging_type(LogType::TracingLogs);
        
        // Force WebSocket streaming transport
        client.connection_options.set_forced_transport(Some(Transport::WsStreaming));
        
        Ok(client)
    }
    
    /// Process market updates from Lightstreamer
    async fn process_market_update(&self, subscription_id: &str, field_name: &str, value: &str) -> Result<(), AppError> {
        // Get the subscription
        let subscriptions = self.subscriptions.lock().unwrap();
        let subscription = match subscriptions.get(subscription_id) {
            Some(sub) => sub,
            None => {
                warn!("Received update for unknown subscription: {}", subscription_id);
                return Ok(());
            }
        };
        
        // Only process market updates
        if subscription.subscription_type != SubscriptionType::Market {
            return Ok(());
        }
        
        // Get the current market update or create a new one
        let mut market_update = MarketUpdate {
            epic: subscription.item.clone(),
            bid: 0.0,
            offer: 0.0,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };
        
        // Update the field
        match field_name {
            "BID" => {
                if let Ok(bid) = value.parse::<f64>() {
                    market_update.bid = bid;
                }
            },
            "OFFER" => {
                if let Ok(offer) = value.parse::<f64>() {
                    market_update.offer = offer;
                }
            },
            "UPDATE_TIME" => {
                market_update.timestamp = value.to_string();
            },
            _ => {
                // Ignore other fields
            }
        }
        
        // Send the update
        if let Err(e) = self.market_tx.send(market_update).await {
            error!("Failed to send market update: {}", e);
        }
        
        Ok(())
    }
    
    /// Process account updates from Lightstreamer
    async fn process_account_update(&self, subscription_id: &str, field_name: &str, value: &str) -> Result<(), AppError> {
        // Get the subscription
        let subscriptions = self.subscriptions.lock().unwrap();
        let subscription = match subscriptions.get(subscription_id) {
            Some(sub) => sub,
            None => {
                warn!("Received update for unknown subscription: {}", subscription_id);
                return Ok(());
            }
        };
        
        // Only process account updates
        if subscription.subscription_type != SubscriptionType::Account {
            return Ok(());
        }
        
        // Create an account update
        let account_update = AccountUpdate {
            account_id: subscription.item.clone(),
            update_type: field_name.to_string(),
            data: serde_json::json!({
                field_name: value
            }),
        };
        
        // Send the update
        if let Err(e) = self.account_tx.send(account_update).await {
            error!("Failed to send account update: {}", e);
        }
        
        Ok(())
    }
}

#[async_trait]
impl IgWebLSClient for LightstreamerClientImpl {
    async fn connect(&self, session: &IgSession) -> Result<(), AppError> {
        info!("Connecting to Lightstreamer server...");
        
        // Initialize the client
        let client = self.init_client(session).await?;
        
        // Store the client
        {
            let mut ls_client = self.ls_client.lock().unwrap();
            *ls_client = Some(client);
        }
        
        // Set connected flag
        *self.connected.lock().unwrap() = true;
        
        info!("Connected to Lightstreamer server");
        
        Ok(())
    }
    
    async fn disconnect(&self) -> Result<(), AppError> {
        info!("Disconnecting from Lightstreamer server...");
        
        // Signal shutdown
        self.shutdown.notify_one();
        
        // Set connected flag
        *self.connected.lock().unwrap() = false;
        
        // Clear client
        {
            let mut ls_client = self.ls_client.lock().unwrap();
            *ls_client = None;
        }
        
        // Clear subscriptions
        {
            let mut subscriptions = self.subscriptions.lock().unwrap();
            subscriptions.clear();
            
            let mut ls_subscriptions = self.ls_subscriptions.lock().unwrap();
            ls_subscriptions.clear();
        }
        
        info!("Disconnected from Lightstreamer server");
        
        Ok(())
    }
    
    async fn subscribe_market(&self, epic: &str) -> Result<String, AppError> {
        // Generate a subscription ID
        let subscription_id = format!("MARKET-{}", uuid::Uuid::new_v4());
        
        // Create subscription
        let subscription = IgSubscription {
            id: subscription_id.clone(),
            subscription_type: SubscriptionType::Market,
            item: epic.to_string(),
        };
        
        // Store subscription
        {
            let mut subscriptions = self.subscriptions.lock().unwrap();
            subscriptions.insert(subscription_id.clone(), subscription);
        }
        
        info!("Subscribed to market updates for {}", epic);
        
        Ok(subscription_id)
    }
    
    async fn subscribe_account(&self) -> Result<String, AppError> {
        // Generate a subscription ID
        let subscription_id = format!("ACCOUNT-{}", uuid::Uuid::new_v4());
        
        // Create subscription
        let subscription = IgSubscription {
            id: subscription_id.clone(),
            subscription_type: SubscriptionType::Account,
            item: "ACCOUNT".to_string(),
        };
        
        // Store subscription
        {
            let mut subscriptions = self.subscriptions.lock().unwrap();
            subscriptions.insert(subscription_id.clone(), subscription);
        }
        
        info!("Subscribed to account updates");
        
        Ok(subscription_id)
    }
    
    async fn unsubscribe(&self, subscription_id: &str) -> Result<(), AppError> {
        // Check if subscription exists
        {
            let mut subscriptions = self.subscriptions.lock().unwrap();
            if !subscriptions.contains_key(subscription_id) {
                return Err(AppError::WebSocketError(format!("Subscription not found: {}", subscription_id)));
            }
            
            // Remove subscription
            subscriptions.remove(subscription_id);
        }
        
        info!("Unsubscribed from {}", subscription_id);
        
        Ok(())
    }
    
    fn is_connected(&self) -> bool {
        *self.connected.lock().unwrap()
    }
    
    fn market_updates(&self) -> Receiver<MarketUpdate> {
        let mut rx_guard = self.market_rx.lock().unwrap();
        if let Some(rx) = rx_guard.take() {
            return rx;
        }
        
        // Create a new channel if none exists
        let (_, rx) = mpsc::channel::<MarketUpdate>(100);
        rx
    }
    
    fn account_updates(&self) -> Receiver<AccountUpdate> {
        let mut rx_guard = self.account_rx.lock().unwrap();
        if let Some(rx) = rx_guard.take() {
            return rx;
        }
        
        // Create a new channel if none exists
        let (_, rx) = mpsc::channel::<AccountUpdate>(100);
        rx
    }
}

// Implement Clone for LightstreamerClientImpl
impl Clone for LightstreamerClientImpl {
    fn clone(&self) -> Self {
        let (market_tx, market_rx) = mpsc::channel(100);
        let (account_tx, account_rx) = mpsc::channel(100);
        
        Self {
            config: self.config.clone(),
            connected: self.connected.clone(),
            subscriptions: self.subscriptions.clone(),
            ls_client: self.ls_client.clone(),
            ls_subscriptions: self.ls_subscriptions.clone(),
            market_tx,
            market_rx: Arc::new(Mutex::new(Some(market_rx))),
            account_tx,
            account_rx: Arc::new(Mutex::new(Some(account_rx))),
            shutdown: self.shutdown.clone(),
        }
    }
}

