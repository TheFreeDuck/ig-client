use async_trait::async_trait;
use reqwest::Method;
use std::sync::Arc;
use tracing::{debug, info};

use crate::{
    application::models::market::{HistoricalPricesResponse, MarketDetails, MarketSearchResult},
    config::Config,
    error::AppError,
    session::interface::IgSession,
    transport::http_client::IgHttpClient,
};

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

/// Implementation of the market service
pub struct MarketServiceImpl<T: IgHttpClient> {
    config: Arc<Config>,
    client: Arc<T>,
}

impl<T: IgHttpClient> MarketServiceImpl<T> {
    /// Creates a new instance of the market service
    pub fn new(config: Arc<Config>, client: Arc<T>) -> Self {
        Self { config, client }
    }

    /// Gets the current configuration
    ///
    /// # Returns
    /// * Reference to the current configuration
    pub fn get_config(&self) -> &Config {
        &self.config
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
impl<T: IgHttpClient + 'static> MarketService for MarketServiceImpl<T> {
    async fn search_markets(
        &self,
        session: &IgSession,
        search_term: &str,
    ) -> Result<MarketSearchResult, AppError> {
        let path = format!("markets?searchTerm={}", search_term);
        info!("Searching markets with term: {}", search_term);

        let result = self
            .client
            .request::<(), MarketSearchResult>(Method::GET, &path, session, None, "1")
            .await?;

        debug!("{} markets found", result.markets.len());
        Ok(result)
    }

    async fn get_market_details(
        &self,
        session: &IgSession,
        epic: &str,
    ) -> Result<MarketDetails, AppError> {
        let path = format!("markets/{}", epic);
        info!("Getting market details: {}", epic);

        let result = self
            .client
            .request::<(), MarketDetails>(Method::GET, &path, session, None, "3")
            .await?;

        debug!("Market details obtained for: {}", epic);
        Ok(result)
    }

    async fn get_historical_prices(
        &self,
        session: &IgSession,
        epic: &str,
        resolution: &str,
        from: &str,
        to: &str,
    ) -> Result<HistoricalPricesResponse, AppError> {
        let path = format!("prices/{}/{}?from={}&to={}", epic, resolution, from, to);
        info!("Getting historical prices for: {}", epic);

        let result = self
            .client
            .request::<(), HistoricalPricesResponse>(Method::GET, &path, session, None, "3")
            .await?;

        debug!("Historical prices obtained for: {}", epic);
        Ok(result)
    }
}
