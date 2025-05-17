use crate::application::services::ListenerResult;
use crate::presentation::MarketData;
use lightstreamer_rs::subscription::{ItemUpdate, SubscriptionListener};
use std::sync::Arc;
use tracing::log::debug;
use tracing::{error, info};

/// Listener for market data updates from the IG Markets streaming API
pub struct MarketListener {
    /// Callback function to be called when market data is received
    callback: Arc<dyn Fn(&MarketData) -> ListenerResult + Send + Sync>,
}

impl MarketListener {
    /// Creates a new market listener with the specified callback function
    ///
    /// # Arguments
    /// * `callback` - Function to be called when market data is received
    ///
    /// # Returns
    /// * A new MarketListener instance
    pub fn new<F>(callback: F) -> Self
    where
        F: Fn(&MarketData) -> ListenerResult + Send + Sync + 'static,
    {
        MarketListener {
            callback: Arc::new(callback),
        }
    }

    /// Sets a new callback function for this listener
    ///
    /// # Arguments
    /// * `callback` - New function to be called when market data is received
    #[allow(dead_code)]
    fn set_callback<F>(&mut self, callback: F)
    where
        F: Fn(&MarketData) -> ListenerResult + Send + Sync + 'static,
    {
        self.callback = Arc::new(callback);
    }

    /// Calls the callback function with the provided market data
    ///
    /// # Arguments
    /// * `market_data` - Market data to pass to the callback
    ///
    /// # Returns
    /// * Result of the callback function
    fn callback(&self, market_data: &MarketData) -> ListenerResult {
        (self.callback)(market_data)
    }

    /// Creates a mock market listener for testing
    ///
    /// # Returns
    /// * A MarketListener with a debug logging callback
    #[cfg(test)]
    pub fn mock() -> Self {
        Self::new(|data| {
            debug!("Mock account callback received: {}", data);
            Ok(())
        })
    }
}

impl SubscriptionListener for MarketListener {
    fn on_item_update(&self, update: &ItemUpdate) {
        let market_data: MarketData = update.into();

        match self.callback(&market_data) {
            Ok(_) => debug!("{}", market_data),
            Err(e) => error!("Error in market data callback: {}", e),
        }
    }

    fn on_subscription(&mut self) {
        info!("Market Subscription confirmed by the server");
    }
}
