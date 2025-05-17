use crate::application::services::ListenerResult;
use crate::presentation::AccountData;
use lightstreamer_rs::subscription::{ItemUpdate, SubscriptionListener};
use std::sync::Arc;
use tracing::log::debug;
use tracing::{error, info};

/// Account data listener that processes updates through a callback
/// Thread-safe and can be shared between threads
pub struct AccountListener {
    callback: Arc<dyn Fn(&AccountData) -> ListenerResult + Send + Sync>,
}

impl AccountListener {
    /// Creates a new AccountListener with the specified callback
    ///
    /// # Arguments
    ///
    /// * `callback` - A function that will be called with account data updates
    ///
    /// # Returns
    ///
    /// A new instance of AccountListener
    pub fn new<F>(callback: F) -> Self
    where
        F: Fn(&AccountData) -> ListenerResult + Send + Sync + 'static,
    {
        AccountListener {
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
        F: Fn(&AccountData) -> ListenerResult + Send + Sync + 'static,
    {
        self.callback = Arc::new(callback);
    }

    /// Executes the callback with the provided account data
    ///
    /// # Arguments
    ///
    /// * `account_data` - The account data to pass to the callback
    ///
    /// # Returns
    ///
    /// The result of the callback function
    fn callback(&self, account_data: &AccountData) -> ListenerResult {
        (self.callback)(account_data)
    }

    /// For testing purposes only - creates a listener that logs but doesn't call any callback
    #[cfg(test)]
    pub fn mock() -> Self {
        Self::new(|data| {
            debug!("Mock account callback received: {}", data);
            Ok(())
        })
    }
}

impl SubscriptionListener for AccountListener {
    fn on_item_update(&self, update: &ItemUpdate) {
        let account_data: AccountData = update.into();

        match self.callback(&account_data) {
            Ok(_) => debug!("{}", account_data),
            Err(e) => error!("Error in account data callback: {}", e),
        }
    }

    fn on_subscription(&mut self) {
        info!("Account Subscription confirmed by the server");
    }
}
