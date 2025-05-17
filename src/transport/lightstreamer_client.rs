use serde::{Deserialize, Serialize};

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

// /// Market data update
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct MarketUpdate {
//     /// Market epic
//     pub epic: String,
//     /// Current bid price
//     pub bid: f64,
//     /// Current offer price
//     pub offer: f64,
//     /// High price
//     pub high: Option<f64>,
//     /// Low price
//     pub low: Option<f64>,
//     /// Mid open price
//     pub mid_open: Option<f64>,
//     /// Change
//     pub change: Option<f64>,
//     /// Change percentage
//     pub change_pct: Option<f64>,
//     /// Market delay
//     pub market_delay: Option<i32>,
//     /// Market state
//     pub market_state: Option<String>,
//     /// Timestamp of the update
//     pub timestamp: String,
// }

// /// Account update
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct AccountUpdate {
//     /// Account ID
//     pub account_id: String,
//     /// Update type (POSITION, ORDER, etc.)
//     pub update_type: String,
//     /// The data associated with the update
//     pub data: serde_json::Value,
// }

// /// Represents a subscription to a specific market or account stream
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct IgSubscription {
//     /// The subscription ID
//     pub id: String,
//     /// The type of subscription (MARKET, ACCOUNT, etc.)
//     pub subscription_type: SubscriptionType,
//     /// The specific item being subscribed to (e.g., market epic)
//     pub item: String,
// }

// /// Subscription listener for market updates
// pub struct MarketUpdateListener {
//     pub tx: Sender<MarketUpdate>,
//     pub epic: String,
// }
//
// impl SubscriptionListener for MarketUpdateListener {
//     fn on_subscription(&mut self) {
//         info!("Market subscription confirmed by the server for {}", self.epic);
//     }
//
//     fn on_item_update(&self, update: &ItemUpdate) {
//         if let Some(item_name) = update.get_item_name() {
//             info!("Received market update for item: {}", item_name);
//         } else {
//             info!("Received market update for unknown item");
//         }
//
//         let mut market_update = MarketUpdate {
//             epic: self.epic.clone(),
//             bid: 0.0,
//             offer: 0.0,
//             high: None,
//             low: None,
//             mid_open: None,
//             change: None,
//             change_pct: None,
//             market_delay: None,
//             market_state: None,
//             timestamp: chrono::Utc::now().to_rfc3339(),
//         };
//
//         let mut updated = false;
//
//         // Process all fields in the update
//         let fields = update.get_fields();
//         for (field, value_opt) in fields.iter() {
//             if let Some(value) = value_opt {
//                 info!("  {} = {}", field, value);
//
//                 match field.as_str() {
//                     "BID" => {
//                         if let Ok(bid) = value.parse::<f64>() {
//                             market_update.bid = bid;
//                             updated = true;
//                         }
//                     },
//                     "OFFER" => {
//                         if let Ok(offer) = value.parse::<f64>() {
//                             market_update.offer = offer;
//                             updated = true;
//                         }
//                     },
//                     "HIGH" => {
//                         if let Ok(high) = value.parse::<f64>() {
//                             market_update.high = Some(high);
//                             updated = true;
//                         }
//                     },
//                     "LOW" => {
//                         if let Ok(low) = value.parse::<f64>() {
//                             market_update.low = Some(low);
//                             updated = true;
//                         }
//                     },
//                     "MID_OPEN" => {
//                         if let Ok(mid_open) = value.parse::<f64>() {
//                             market_update.mid_open = Some(mid_open);
//                             updated = true;
//                         }
//                     },
//                     "CHANGE" => {
//                         if let Ok(change) = value.parse::<f64>() {
//                             market_update.change = Some(change);
//                             updated = true;
//                         }
//                     },
//                     "CHANGE_PCT" => {
//                         if let Ok(change_pct) = value.parse::<f64>() {
//                             market_update.change_pct = Some(change_pct);
//                             updated = true;
//                         }
//                     },
//                     "MARKET_DELAY" => {
//                         if let Ok(market_delay) = value.parse::<i32>() {
//                             market_update.market_delay = Some(market_delay);
//                             updated = true;
//                         }
//                     },
//                     "MARKET_STATE" => {
//                         market_update.market_state = Some(value.to_string());
//                         updated = true;
//                     },
//                     "UPDATE_TIME" => {
//                         market_update.timestamp = value.to_string();
//                         updated = true;
//                     },
//                     _ => {
//                         // Ignore other fields
//                     }
//                 }
//             }
//         }
//
//         // Only send the update if at least one field was updated
//         if updated {
//             info!("Sending market update for {}: bid={}, offer={}",
//                   market_update.epic, market_update.bid, market_update.offer);
//
//             if let Err(e) = self.tx.try_send(market_update) {
//                 error!("Failed to send market update: {}", e);
//             }
//         }
//     }
// }

// /// Subscription listener for account updates
// pub struct AccountUpdateListener {
//     pub tx: Sender<AccountUpdate>,
//     pub account_id: String,
// }
//
// impl SubscriptionListener for AccountUpdateListener {
//     fn on_subscription(&mut self) {
//         info!("Account subscription confirmed by the server for {}", self.account_id);
//     }
//
//     fn on_item_update(&self, update: &ItemUpdate) {
//         if let Some(item_name) = update.get_item_name() {
//             info!("Received account update for item: {}", item_name);
//         } else {
//             info!("Received account update for unknown item");
//         }
//
//         // Process all fields in the update
//         let fields = update.get_fields();
//         for (field, value_opt) in fields.iter() {
//             if let Some(value) = value_opt {
//                 info!("  {} = {}", field, value);
//
//                 // Create an account update
//                 let account_update = AccountUpdate {
//                     account_id: self.account_id.clone(),
//                     update_type: field.clone(),
//                     data: serde_json::json!({
//                         field: value
//                     }),
//                 };
//
//                 // Send the update
//                 if let Err(e) = self.tx.try_send(account_update) {
//                     error!("Failed to send account update: {}", e);
//                 }
//             }
//         }
//     }
// }

// /// Implementation of the WebSocket client using Lightstreamer
// pub struct LightstreamerClientImpl {
//     /// Configuration
//     config: Arc<Config>,
//     /// Whether the client is connected
//     connected: Arc<Mutex<bool>>,
//     /// Subscriptions
//     subscriptions: Arc<Mutex<HashMap<String, IgSubscription>>>,
//     /// Lightstreamer client
//     ls_client: Arc<Mutex<Option<LightstreamerClient>>>,
//     /// Lightstreamer subscription IDs
//     ls_subscription_ids: Arc<Mutex<HashMap<String, usize>>>,
//     /// Market updates channel
//     market_tx: Sender<MarketUpdate>,
//     /// Market updates receiver
//     market_rx: Arc<Mutex<Option<Receiver<MarketUpdate>>>>,
//     /// Account updates channel
//     account_tx: Sender<AccountUpdate>,
//     /// Account updates receiver
//     account_rx: Arc<Mutex<Option<Receiver<AccountUpdate>>>>,
//     /// Shutdown signal
//     shutdown: Arc<Notify>,
// }
//
// impl LightstreamerClientImpl {
//     /// Create a new Lightstreamer client
//     pub fn new(config: Arc<Config>) -> Self {
//         let (market_tx, market_rx) = mpsc::channel(100);
//         let (account_tx, account_rx) = mpsc::channel(100);
//
//         Self {
//             config,
//             connected: Arc::new(Mutex::new(false)),
//             subscriptions: Arc::new(Mutex::new(HashMap::new())),
//             ls_client: Arc::new(Mutex::new(None)),
//             ls_subscription_ids: Arc::new(Mutex::new(HashMap::new())),
//             market_tx,
//             market_rx: Arc::new(Mutex::new(Some(market_rx))),
//             account_tx,
//             account_rx: Arc::new(Mutex::new(Some(account_rx))),
//             shutdown: Arc::new(Notify::new()),
//         }
//     }
//
//     /// Initialize the Lightstreamer client
//     async fn init_client(&self, session: &IgSession) -> Result<LightstreamerClient, AppError> {
//         // Determine if we're in demo environment based on the WebSocket URL in config
//         // If the URL contains 'demo', we're in demo environment
//         let ws_url = &self.config.websocket.url;
//         let is_demo = ws_url.contains("demo");
//
//         // Determine the server address based on environment
//         // Según la documentación de IG, la URL correcta es:
//         let server_address = if is_demo {
//             "https://demo-apd.marketdatasystems.com/lightstreamer"
//         } else {
//             "https://apd.marketdatasystems.com/lightstreamer"
//         };
//
//         // Para IG Markets, no necesitamos especificar un adapter set
//         // Dejamos esto vacío para que funcione correctamente
//         let adapter_set = "";
//
//         info!("Using Lightstreamer server: {}", server_address);
//         info!("Using adapter set: {}", adapter_set);
//         info!("Using account ID: {}", session.account_id.trim());
//
//         // Format the password as required by IG's Lightstreamer authentication
//         // This follows the format "CST-{cst}|XST-{token}" as mentioned in your memories
//         let cst = session.cst.trim();
//         let token = session.token.trim();
//         let password = format!("CST-{}|XST-{}", cst, token);
//
//         info!("Using CST token of length: {}", cst.len());
//         info!("Using XST token of length: {}", token.len());
//
//         // Create the Lightstreamer client
//         // Usamos el constructor con parámetros
//         let mut client = LightstreamerClient::new(
//             Some(server_address),
//             None, // No usamos adapter_set
//             Some(session.account_id.trim()),
//             Some(&password),
//         ).map_err(|e| AppError::WebSocketError(format!("Failed to create Lightstreamer client: {}", e)))?;
//
//         // Configure connection options
//         client.connection_options.set_forced_transport(Some(Transport::WsStreaming));
//
//         Ok(client)
//     }
// }
//
// #[async_trait]
// impl IgWebLSClient for LightstreamerClientImpl {
//     async fn connect(&self, session: &IgSession) -> Result<(), AppError> {
//         info!("Connecting to Lightstreamer server...");
//
//         // Initialize the client
//         let mut client = self.init_client(session).await?;
//
//         // Create a shutdown signal for graceful termination
//         let shutdown_signal = self.shutdown.clone();
//
//         // Connect to the Lightstreamer server
//         if let Err(e) = client.connect(shutdown_signal).await {
//             error!("Failed to connect to Lightstreamer server: {}", e);
//             return Err(AppError::WebSocketError(format!("Failed to connect to Lightstreamer server: {}", e)));
//         }
//
//         // Store the client
//         {
//             let mut ls_client = self.ls_client.lock().unwrap();
//             *ls_client = Some(client);
//         }
//
//         // Set connected flag
//         *self.connected.lock().unwrap() = true;
//
//         info!("Connected to Lightstreamer server");
//
//         Ok(())
//     }
//
//     async fn disconnect(&self) -> Result<(), AppError> {
//         info!("Disconnecting from Lightstreamer server...");
//
//         // Signal shutdown
//         self.shutdown.notify_one();
//
//         // Set connected flag
//         *self.connected.lock().unwrap() = false;
//
//         // Clear client
//         {
//             let mut ls_client = self.ls_client.lock().unwrap();
//             *ls_client = None;
//         }
//
//         // Clear subscriptions
//         {
//             let mut subscriptions = self.subscriptions.lock().unwrap();
//             subscriptions.clear();
//
//             let mut ls_subscription_ids = self.ls_subscription_ids.lock().unwrap();
//             ls_subscription_ids.clear();
//         }
//
//         info!("Disconnected from Lightstreamer server");
//
//         Ok(())
//     }
//
//     async fn subscribe_market(&self, epic: &str) -> Result<String, AppError> {
//         // Generate a subscription ID
//         let subscription_id = format!("MARKET-{}", uuid::Uuid::new_v4());
//
//         // Create subscription
//         let subscription = IgSubscription {
//             id: subscription_id.clone(),
//             subscription_type: SubscriptionType::Market,
//             item: epic.to_string(),
//         };
//
//         // Store subscription
//         {
//             let mut subscriptions = self.subscriptions.lock().unwrap();
//             subscriptions.insert(subscription_id.clone(), subscription.clone());
//         }
//
//         // Create Lightstreamer subscription
//         // Usamos los mismos campos que se muestran en el Companion
//         let mut ls_subscription = Subscription::new(
//             SubscriptionMode::Merge,
//             Some(vec![epic.to_string()]),
//             Some(vec![
//                 "BID".to_string(),
//                 "OFFER".to_string(),
//                 "HIGH".to_string(),
//                 "LOW".to_string(),
//                 "MID_OPEN".to_string(),
//                 "CHANGE".to_string(),
//                 "CHANGE_PCT".to_string(),
//                 "MARKET_DELAY".to_string(),
//                 "MARKET_STATE".to_string(),
//                 "UPDATE_TIME".to_string()
//             ]),
//         ).map_err(|e| AppError::WebSocketError(format!("Failed to create subscription: {}", e)))?;
//
//         // Add a listener to the subscription
//         let listener = Box::new(MarketUpdateListener {
//             tx: self.market_tx.clone(),
//             epic: epic.to_string(),
//         });
//         ls_subscription.add_listener(listener);
//
//         // Get the client's subscription sender
//         let subscription_sender = {
//             let ls_client_guard = self.ls_client.lock().unwrap();
//             if let Some(client) = &*ls_client_guard {
//                 client.subscription_sender.clone()
//             } else {
//                 return Err(AppError::WebSocketError("Client not connected".to_string()));
//             }
//         };
//
//         // Subscribe to the item
//         LightstreamerClient::subscribe(subscription_sender, ls_subscription).await;
//
//         // Store the subscription ID (using a counter for now)
//         let subscription_id_num = {
//             let mut ls_subscription_ids = self.ls_subscription_ids.lock().unwrap();
//             let id = ls_subscription_ids.len() + 1;
//             ls_subscription_ids.insert(subscription_id.clone(), id);
//             id
//         };
//
//         info!("Subscribed to market updates for {} with ID {}", epic, subscription_id_num);
//
//         Ok(subscription_id)
//     }
//
//     async fn subscribe_account(&self) -> Result<String, AppError> {
//         // Generate a subscription ID
//         let subscription_id = format!("ACCOUNT-{}", uuid::Uuid::new_v4());
//
//         // Create subscription
//         let subscription = IgSubscription {
//             id: subscription_id.clone(),
//             subscription_type: SubscriptionType::Account,
//             item: "ACCOUNT".to_string(),
//         };
//
//         // Store subscription
//         {
//             let mut subscriptions = self.subscriptions.lock().unwrap();
//             subscriptions.insert(subscription_id.clone(), subscription.clone());
//         }
//
//         // Create Lightstreamer subscription
//         // Para suscripciones de cuenta, necesitamos usar el formato correcto para IG
//         // Según la documentación, debemos usar ACCOUNT:{accountId} como item
//         let account_id = self.config.credentials.account_id.trim().to_string();
//         let account_item = format!("ACCOUNT:{}", account_id);
//
//         let mut ls_subscription = Subscription::new(
//             SubscriptionMode::Merge,
//             Some(vec![account_item.clone()]),
//             Some(vec!["CONFIRMS".to_string(), "OPU".to_string(), "WOU".to_string()]),
//         ).map_err(|e| AppError::WebSocketError(format!("Failed to create account subscription: {}", e)))?;
//
//         // Add a listener to the subscription
//         let listener = Box::new(AccountUpdateListener {
//             tx: self.account_tx.clone(),
//             account_id: account_item,
//         });
//         ls_subscription.add_listener(listener);
//
//         // Get the client's subscription sender
//         let subscription_sender = {
//             let ls_client_guard = self.ls_client.lock().unwrap();
//             if let Some(client) = &*ls_client_guard {
//                 client.subscription_sender.clone()
//             } else {
//                 return Err(AppError::WebSocketError("Client not connected".to_string()));
//             }
//         };
//
//         // Subscribe to the item
//         LightstreamerClient::subscribe(subscription_sender, ls_subscription).await;
//
//         // Store the subscription ID (using a counter for now)
//         let subscription_id_num = {
//             let mut ls_subscription_ids = self.ls_subscription_ids.lock().unwrap();
//             let id = ls_subscription_ids.len() + 1;
//             ls_subscription_ids.insert(subscription_id.clone(), id);
//             id
//         };
//
//         info!("Subscribed to account updates with ID {}", subscription_id_num);
//
//         Ok(subscription_id)
//     }
//
//     async fn unsubscribe(&self, subscription_id: &str) -> Result<(), AppError> {
//         // Get the Lightstreamer subscription ID
//         let ls_subscription_id = {
//             let ls_subscription_ids = self.ls_subscription_ids.lock().unwrap();
//             match ls_subscription_ids.get(subscription_id) {
//                 Some(id) => *id,
//                 None => return Err(AppError::WebSocketError(format!("Subscription not found: {}", subscription_id))),
//             }
//         };
//
//         // Get the client's subscription sender
//         // Note: In the current implementation, we don't have a way to unsubscribe by ID
//         // We would need to keep track of the subscription objects
//         // For now, we'll just remove it from our tracking
//
//         // Remove subscription
//         {
//             let mut subscriptions = self.subscriptions.lock().unwrap();
//             subscriptions.remove(subscription_id);
//
//             let mut ls_subscription_ids = self.ls_subscription_ids.lock().unwrap();
//             ls_subscription_ids.remove(subscription_id);
//         }
//
//         info!("Unsubscribed from {}", subscription_id);
//
//         Ok(())
//     }
//
//     fn is_connected(&self) -> bool {
//         *self.connected.lock().unwrap()
//     }
//
//     fn market_updates(&self) -> Receiver<MarketUpdate> {
//         let mut rx_guard = self.market_rx.lock().unwrap();
//         if let Some(rx) = rx_guard.take() {
//             return rx;
//         }
//
//         // Create a new channel if none exists
//         let (_, rx) = mpsc::channel::<MarketUpdate>(100);
//         rx
//     }
//
//     fn account_updates(&self) -> Receiver<AccountUpdate> {
//         let mut rx_guard = self.account_rx.lock().unwrap();
//         if let Some(rx) = rx_guard.take() {
//             return rx;
//         }
//
//         // Create a new channel if none exists
//         let (_, rx) = mpsc::channel::<AccountUpdate>(100);
//         rx
//     }
// }
//
// // Implement Clone for LightstreamerClientImpl
// impl Clone for LightstreamerClientImpl {
//     fn clone(&self) -> Self {
//         let (market_tx, market_rx) = mpsc::channel(100);
//         let (account_tx, account_rx) = mpsc::channel(100);
//
//         Self {
//             config: self.config.clone(),
//             connected: self.connected.clone(),
//             subscriptions: self.subscriptions.clone(),
//             ls_client: self.ls_client.clone(),
//             ls_subscription_ids: self.ls_subscription_ids.clone(),
//             market_tx,
//             market_rx: Arc::new(Mutex::new(Some(market_rx))),
//             account_tx,
//             account_rx: Arc::new(Mutex::new(Some(account_rx))),
//             shutdown: self.shutdown.clone(),
//         }
//     }
// }
