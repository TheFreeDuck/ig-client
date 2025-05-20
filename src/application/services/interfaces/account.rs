use async_trait::async_trait;
use crate::application::models::account::{AccountActivity, AccountInfo, Positions, TransactionHistory, WorkingOrders};
use crate::error::AppError;
use crate::session::interface::IgSession;

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