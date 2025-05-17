use crate::application::services::ListenerResult;
use crate::presentation::AccountData;
use lightstreamer_rs::subscription::{ItemUpdate, SubscriptionListener};
use std::sync::Arc;
use tracing::log::debug;
use tracing::{error, info};

/// Listener para datos de cuenta que procesa actualizaciones a través de un callback
/// Seguro entre hilos y puede ser compartido entre hilos
pub struct AccountListener {
    callback: Arc<dyn Fn(&AccountData) -> ListenerResult + Send + Sync>,
}

impl AccountListener {
    /// Crea un nuevo AccountListener con el callback especificado
    ///
    /// # Arguments
    ///
    /// * `callback` - Una función que será llamada con las actualizaciones de datos de cuenta
    ///
    /// # Returns
    ///
    /// Una nueva instancia de AccountListener
    pub fn new<F>(callback: F) -> Self
    where
        F: Fn(&AccountData) -> ListenerResult + Send + Sync + 'static,
    {
        AccountListener {
            callback: Arc::new(callback),
        }
    }

    /// Actualiza la función de callback
    ///
    /// # Arguments
    ///
    /// * `callback` - La nueva función de callback
    #[allow(dead_code)]
    fn set_callback<F>(&mut self, callback: F)
    where
        F: Fn(&AccountData) -> ListenerResult + Send + Sync + 'static,
    {
        self.callback = Arc::new(callback);
    }

    /// Ejecuta el callback con los datos de cuenta proporcionados
    ///
    /// # Arguments
    ///
    /// * `account_data` - Los datos de cuenta para pasar al callback
    ///
    /// # Returns
    ///
    /// El resultado de la función de callback
    fn callback(&self, account_data: &AccountData) -> ListenerResult {
        (self.callback)(account_data)
    }

    /// Solo para propósitos de prueba - crea un listener que registra pero no llama a ningún callback
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
