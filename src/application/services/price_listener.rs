use crate::application::services::ListenerResult;
use crate::presentation::PriceData;
use lightstreamer_rs::subscription::{ItemUpdate, SubscriptionListener};
use std::sync::Arc;
use tracing::log::debug;
use tracing::{error, info};

/// Price data listener that processes updates through a callback
/// Thread-safe and can be shared between threads
pub struct PriceListener {
    callback: Arc<dyn Fn(&PriceData) -> ListenerResult + Send + Sync>,
}

impl PriceListener {
    /// Creates a new PriceListener with the specified callback
    ///
    /// # Arguments
    ///
    /// * `callback` - A function that will be called with price data updates
    ///
    /// # Returns
    ///
    /// A new instance of PriceListener
    pub fn new<F>(callback: F) -> Self
    where
        F: Fn(&PriceData) -> ListenerResult + Send + Sync + 'static,
    {
        PriceListener {
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
        F: Fn(&PriceData) -> ListenerResult + Send + Sync + 'static,
    {
        self.callback = Arc::new(callback);
    }

    /// Executes the callback with the provided price data
    ///
    /// # Arguments
    ///
    /// * `price_data` - The price data to pass to the callback
    ///
    /// # Returns
    ///
    /// The result of the callback function
    fn callback(&self, price_data: &PriceData) -> ListenerResult {
        (self.callback)(price_data)
    }

    /// For testing purposes only - creates a listener that logs but doesn't call any callback
    #[cfg(test)]
    pub fn mock() -> Self {
        Self::new(|data| {
            debug!("Mock price callback received: {}", data);
            Ok(())
        })
    }
}

impl SubscriptionListener for PriceListener {
    fn on_item_update(&self, update: &ItemUpdate) {
        let price_data: PriceData = update.into();

        match self.callback(&price_data) {
            Ok(_) => debug!("{}", price_data),
            Err(e) => error!("Error in price data callback: {}", e),
        }
    }

    fn on_subscription(&mut self) {
        info!("Price Subscription confirmed by the server");
    }
}
