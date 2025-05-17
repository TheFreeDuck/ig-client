use crate::application::services::ListenerResult;
use crate::presentation::MarketData;
use lightstreamer_rs::subscription::{ItemUpdate, SubscriptionListener};
use std::sync::Arc;
use tracing::log::debug;
use tracing::{error, info};

pub struct MarketListener {
    callback: Arc<dyn Fn(&MarketData) -> ListenerResult + Send + Sync>,
}

impl MarketListener {
    pub fn new<F>(callback: F) -> Self
    where
        F: Fn(&MarketData) -> ListenerResult + Send + Sync + 'static,
    {
        MarketListener {
            callback: Arc::new(callback),
        }
    }

    #[allow(dead_code)]
    fn set_callback<F>(&mut self, callback: F)
    where
        F: Fn(&MarketData) -> ListenerResult + Send + Sync + 'static,
    {
        self.callback = Arc::new(callback);
    }

    fn callback(&self, market_data: &MarketData) -> ListenerResult {
        (self.callback)(market_data)
    }

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
