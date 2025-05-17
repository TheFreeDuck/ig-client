use async_trait::async_trait;
use reqwest::Method;
use std::sync::Arc;
use tracing::{debug, info};

use crate::{
    application::models::account::{
        AccountActivity, AccountInfo, Positions, TransactionHistory, WorkingOrders,
    },
    config::Config,
    error::AppError,
    session::interface::IgSession,
    transport::http_client::IgHttpClient,
};

/// Interface for the account service
#[async_trait]
pub trait AccountService: Send + Sync {
    /// Gets information about all user accounts
    async fn get_accounts(&self, session: &IgSession) -> Result<AccountInfo, AppError>;

    /// Gets open positions
    async fn get_positions(&self, session: &IgSession) -> Result<Positions, AppError>;

    /// Gets working orders
    async fn get_working_orders(&self, session: &IgSession) -> Result<WorkingOrders, AppError>;

    /// Gets account activity
    async fn get_activity(
        &self,
        session: &IgSession,
        from: &str,
        to: &str,
    ) -> Result<AccountActivity, AppError>;

    /// Gets transaction history
    async fn get_transactions(
        &self,
        session: &IgSession,
        from: &str,
        to: &str,
        page_size: u32,
        page_number: u32,
    ) -> Result<TransactionHistory, AppError>;
}

/// Implementation of the account service
pub struct AccountServiceImpl<T: IgHttpClient> {
    config: Arc<Config>,
    client: Arc<T>,
}

impl<T: IgHttpClient> AccountServiceImpl<T> {
    /// Creates a new instance of the account service
    pub fn new(config: Arc<Config>, client: Arc<T>) -> Self {
        Self { config, client }
    }

    /// Gets the current configuration
    ///
    /// # Returns
    /// * The current configuration as an `Arc<Config>`
    pub fn get_config(&self) -> Arc<Config> {
        self.config.clone()
    }

    /// Sets a new configuration
    ///
    /// # Arguments
    /// * `config` - The new configuration to use
    pub fn set_config(&mut self, config: Arc<Config>) {
        self.config = config;
    }
}

#[async_trait]
impl<T: IgHttpClient + 'static> AccountService for AccountServiceImpl<T> {
    async fn get_accounts(&self, session: &IgSession) -> Result<AccountInfo, AppError> {
        info!("Getting account information");

        let result = self
            .client
            .request::<(), AccountInfo>(Method::GET, "accounts", session, None, "1")
            .await?;

        debug!(
            "Account information obtained: {} accounts",
            result.accounts.len()
        );
        Ok(result)
    }

    async fn get_positions(&self, session: &IgSession) -> Result<Positions, AppError> {
        info!("Getting open positions");

        let result = self
            .client
            .request::<(), Positions>(Method::GET, "positions", session, None, "2")
            .await?;

        debug!("Positions obtained: {} positions", result.positions.len());
        Ok(result)
    }

    async fn get_working_orders(&self, session: &IgSession) -> Result<WorkingOrders, AppError> {
        info!("Getting working orders");

        let result = self
            .client
            .request::<(), WorkingOrders>(Method::GET, "workingorders", session, None, "2")
            .await?;

        debug!(
            "Working orders obtained: {} orders",
            result.working_orders.len()
        );
        Ok(result)
    }

    async fn get_activity(
        &self,
        session: &IgSession,
        from: &str,
        to: &str,
    ) -> Result<AccountActivity, AppError> {
        let path = format!("history/activity?from={}&to={}", from, to);
        info!("Getting account activity");

        let result = self
            .client
            .request::<(), AccountActivity>(Method::GET, &path, session, None, "3")
            .await?;

        debug!(
            "Account activity obtained: {} activities",
            result.activities.len()
        );
        Ok(result)
    }

    async fn get_transactions(
        &self,
        session: &IgSession,
        from: &str,
        to: &str,
        page_size: u32,
        page_number: u32,
    ) -> Result<TransactionHistory, AppError> {
        let path = format!(
            "history/transactions?from={}&to={}&pageSize={}&pageNumber={}",
            from, to, page_size, page_number
        );
        info!("Getting transaction history");

        let result = self
            .client
            .request::<(), TransactionHistory>(Method::GET, &path, session, None, "2")
            .await?;

        debug!(
            "Transaction history obtained: {} transactions",
            result.transactions.len()
        );
        Ok(result)
    }
}
