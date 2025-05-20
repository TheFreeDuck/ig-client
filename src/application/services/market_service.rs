use crate::application::services::MarketService;
use crate::{
    application::models::market::{HistoricalPricesResponse, MarketDetails, MarketNavigationResponse, MarketSearchResult},
    config::Config,
    error::AppError,
    session::interface::IgSession,
    transport::http_client::IgHttpClient,
};
use async_trait::async_trait;
use reqwest::Method;
use std::sync::Arc;
use tracing::{debug, info};

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

    async fn get_market_navigation(
        &self,
        session: &IgSession,
    ) -> Result<MarketNavigationResponse, AppError> {
        let path = "marketnavigation";
        info!("Getting top-level market navigation nodes");

        let result = self
            .client
            .request::<(), MarketNavigationResponse>(Method::GET, path, session, None, "1")
            .await?;

        debug!("{} navigation nodes found", result.nodes.len());
        debug!("{} markets found at root level", result.markets.len());
        Ok(result)
    }

    async fn get_market_navigation_node(
        &self,
        session: &IgSession,
        node_id: &str,
    ) -> Result<MarketNavigationResponse, AppError> {
        let path = format!("marketnavigation/{}", node_id);
        info!("Getting market navigation node: {}", node_id);

        let result = self
            .client
            .request::<(), MarketNavigationResponse>(Method::GET, &path, session, None, "1")
            .await?;

        debug!("{} child nodes found", result.nodes.len());
        debug!("{} markets found in node {}", result.markets.len(), node_id);
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::transport::http_client::IgHttpClientImpl;
    use std::sync::Arc;

    #[test]
    fn test_get_and_set_config() {
        let config = Arc::new(Config::new());
        let client = Arc::new(IgHttpClientImpl::new(config.clone()));
        let mut service = MarketServiceImpl::new(config.clone(), client.clone());
        assert!(std::ptr::eq(service.get_config(), &*config));
        let new_cfg = Arc::new(Config::default());
        service.set_config(new_cfg.clone());
        assert!(std::ptr::eq(service.get_config(), &*new_cfg));
    }
}
