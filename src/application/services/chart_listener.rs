use crate::application::services::ListenerResult;
use crate::presentation::ChartData;
use lightstreamer_rs::subscription::{ItemUpdate, SubscriptionListener};
use std::sync::Arc;
use tracing::log::debug;
use tracing::{error, info};

/// Chart data listener that processes updates through a callback
/// Thread-safe and can be shared between threads
pub struct ChartListener {
    callback: Arc<dyn Fn(&ChartData) -> ListenerResult + Send + Sync>,
}

impl ChartListener {
    /// Creates a new ChartListener with the specified callback
    ///
    /// # Arguments
    ///
    /// * `callback` - A function that will be called with chart data updates
    ///
    /// # Returns
    ///
    /// A new instance of ChartListener
    pub fn new<F>(callback: F) -> Self
    where
        F: Fn(&ChartData) -> ListenerResult + Send + Sync + 'static,
    {
        ChartListener {
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
        F: Fn(&ChartData) -> ListenerResult + Send + Sync + 'static,
    {
        self.callback = Arc::new(callback);
    }

    /// Executes the callback with the provided chart data
    ///
    /// # Arguments
    ///
    /// * `chart_data` - The chart data to pass to the callback
    ///
    /// # Returns
    ///
    /// The result of the callback function
    fn callback(&self, chart_data: &ChartData) -> ListenerResult {
        (self.callback)(chart_data)
    }

    /// For testing purposes only - creates a listener that logs but doesn't call any callback
    #[cfg(test)]
    pub fn mock() -> Self {
        Self::new(|data| {
            debug!("Mock chart callback received: {}", data);
            Ok(())
        })
    }
}

impl SubscriptionListener for ChartListener {
    fn on_item_update(&self, update: &ItemUpdate) {
        let chart_data: ChartData = update.into();

        match self.callback(&chart_data) {
            Ok(_) => debug!("{}", chart_data),
            Err(e) => error!("Error in chart data callback: {}", e),
        }
    }

    fn on_subscription(&mut self) {
        info!("Chart Subscription confirmed by the server");
    }
}
