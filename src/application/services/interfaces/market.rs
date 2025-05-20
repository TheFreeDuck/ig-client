use crate::application::models::market::{
    HistoricalPricesResponse, MarketDetails, MarketSearchResult,
};
use crate::error::AppError;
use crate::session::interface::IgSession;
use async_trait::async_trait;

/// Interface for the market service
#[async_trait]
pub trait MarketService: Send + Sync {
    /// Searches markets by search term
    async fn search_markets(
        &self,
        session: &IgSession,
        search_term: &str,
    ) -> Result<MarketSearchResult, AppError>;

    /// Gets details of a specific market by its EPIC
    async fn get_market_details(
        &self,
        session: &IgSession,
        epic: &str,
    ) -> Result<MarketDetails, AppError>;

    /// Gets historical prices for a market
    async fn get_historical_prices(
        &self,
        session: &IgSession,
        epic: &str,
        resolution: &str,
        from: &str,
        to: &str,
    ) -> Result<HistoricalPricesResponse, AppError>;
}
