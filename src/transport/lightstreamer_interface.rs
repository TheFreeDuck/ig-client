use async_trait::async_trait;
use tokio::sync::mpsc::Receiver;

use crate::error::AppError;
use crate::session::interface::IgSession;
use crate::transport::lightstreamer_client::{AccountUpdate, MarketUpdate};

/// Interface for WebSocket client
#[async_trait]
pub trait IgWebLSClient: Send + Sync {
    /// Connect to the WebSocket server
    async fn connect(&self, session: &IgSession) -> Result<(), AppError>;

    /// Disconnect from the WebSocket server
    async fn disconnect(&self) -> Result<(), AppError>;

    /// Subscribe to market updates for a specific epic
    async fn subscribe_market(&self, epic: &str) -> Result<String, AppError>;

    /// Subscribe to account updates
    async fn subscribe_account(&self) -> Result<String, AppError>;

    /// Unsubscribe from updates
    async fn unsubscribe(&self, subscription_id: &str) -> Result<(), AppError>;

    /// Check if the client is connected
    fn is_connected(&self) -> bool;

    /// Get the receiver for market updates
    fn market_updates(&self) -> Receiver<MarketUpdate>;

    /// Get the receiver for account updates
    fn account_updates(&self) -> Receiver<AccountUpdate>;
}
