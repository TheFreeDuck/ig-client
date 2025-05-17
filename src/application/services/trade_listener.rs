use crate::application::services::ListenerResult;
use crate::presentation::TradeData;
use lightstreamer_rs::subscription::{ItemUpdate, SubscriptionListener};
use std::sync::Arc;
use tracing::log::debug;
use tracing::{error, info};

/// Trade data listener that processes updates through a callback
/// Thread-safe and can be shared between threads
pub struct TradeListener {
    /// The callback function that will be called with trade data updates
    callback: Arc<dyn Fn(&TradeData) -> ListenerResult + Send + Sync>,
}

impl TradeListener {
    /// Creates a new TradeListener with the specified callback
    ///
    /// # Arguments
    ///
    /// * `callback` - A function that will be called with trade data updates
    ///
    /// # Returns
    ///
    /// A new instance of TradeListener
    pub fn new<F>(callback: F) -> Self
    where
        F: Fn(&TradeData) -> ListenerResult + Send + Sync + 'static,
    {
        TradeListener {
            callback: Arc::new(callback),
        }
    }

    /// Updates the callback function
    ///
    /// # Arguments
    ///
    /// * `callback` - The new callback function
    #[allow(dead_code)]
    fn set_callback<F>(&mut self, callback: F)
    where
        F: Fn(&TradeData) -> ListenerResult + Send + Sync + 'static,
    {
        self.callback = Arc::new(callback);
    }

    /// Executes the callback with the provided trade data
    ///
    /// # Arguments
    ///
    /// * `trade_data` - The trade data to pass to the callback
    ///
    /// # Returns
    ///
    /// The result of the callback function
    fn callback(&self, trade_data: &TradeData) -> ListenerResult {
        (self.callback)(trade_data)
    }

    /// For testing purposes only - creates a listener that logs but doesn't call any callback
    #[cfg(test)]
    pub fn mock() -> Self {
        Self::new(|data| {
            debug!("Mock trade callback received: {}", data);
            Ok(())
        })
    }
}

impl SubscriptionListener for TradeListener {
    fn on_item_update(&self, update: &ItemUpdate) {
        let trade_data: TradeData = update.into();

        match self.callback(&trade_data) {
            Ok(_) => debug!("{}", trade_data),
            Err(e) => error!("Error in trade data callback: {}", e),
        }
    }

    fn on_subscription(&mut self) {
        info!("Trade Subscription confirmed by the server");
    }
}
