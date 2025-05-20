use crate::application::models::market::{
    HistoricalPricesResponse, MarketDetails, MarketNavigationResponse, MarketSearchResult,
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

    /// Gets the top-level market navigation nodes
    /// 
    /// This method returns the root nodes of the market hierarchy, which can be used
    /// to navigate through the available markets.
    async fn get_market_navigation(
        &self,
        session: &IgSession,
    ) -> Result<MarketNavigationResponse, AppError>;

    /// Gets the market navigation node with the specified ID
    /// 
    /// This method returns the child nodes and markets under the specified node ID.
    /// 
    /// # Arguments
    /// * `node_id` - The ID of the navigation node to retrieve
    async fn get_market_navigation_node(
        &self,
        session: &IgSession,
        node_id: &str,
    ) -> Result<MarketNavigationResponse, AppError>;
}
