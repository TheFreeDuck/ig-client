use crate::application::models::account::WorkingOrders;
use crate::application::models::order::{
    ClosePositionRequest, ClosePositionResponse, CreateOrderRequest, CreateOrderResponse,
    OrderConfirmation, UpdatePositionRequest, UpdatePositionResponse,
};
use crate::application::models::working_order::{
    CreateWorkingOrderRequest, CreateWorkingOrderResponse,
};
use crate::application::services::interfaces::order::OrderService;
use crate::config::Config;
use crate::error::AppError;
use crate::session::interface::IgSession;
use crate::transport::http_client::IgHttpClient;
use async_trait::async_trait;
use reqwest::Method;
use std::sync::Arc;
use tracing::{debug, info};

/// Implementation of the order service
pub struct OrderServiceImpl<T: IgHttpClient> {
    config: Arc<Config>,
    client: Arc<T>,
}

impl<T: IgHttpClient> OrderServiceImpl<T> {
    /// Creates a new instance of the order service
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
impl<T: IgHttpClient + 'static> OrderService for OrderServiceImpl<T> {
    async fn create_order(
        &self,
        session: &IgSession,
        order: &CreateOrderRequest,
    ) -> Result<CreateOrderResponse, AppError> {
        info!("Creating order for: {}", order.epic);

        let result = self
            .client
            .request::<CreateOrderRequest, CreateOrderResponse>(
                Method::POST,
                "positions/otc",
                session,
                Some(order),
                "2",
            )
            .await?;

        debug!("Order created with reference: {}", result.deal_reference);
        Ok(result)
    }

    async fn get_order_confirmation(
        &self,
        session: &IgSession,
        deal_reference: &str,
    ) -> Result<OrderConfirmation, AppError> {
        let path = format!("confirms/{deal_reference}");
        info!("Getting confirmation for order: {}", deal_reference);

        let result = self
            .client
            .request::<(), OrderConfirmation>(Method::GET, &path, session, None, "1")
            .await?;

        debug!("Confirmation obtained for order: {}", deal_reference);
        Ok(result)
    }

    async fn update_position(
        &self,
        session: &IgSession,
        deal_id: &str,
        update: &UpdatePositionRequest,
    ) -> Result<UpdatePositionResponse, AppError> {
        let path = format!("positions/otc/{deal_id}");
        info!("Updating position: {}", deal_id);

        let result = self
            .client
            .request::<UpdatePositionRequest, UpdatePositionResponse>(
                Method::PUT,
                &path,
                session,
                Some(update),
                "2",
            )
            .await?;

        debug!(
            "Position updated: {} with deal reference: {}",
            deal_id, result.deal_reference
        );
        Ok(result)
    }

    async fn close_position(
        &self,
        session: &IgSession,
        close_request: &ClosePositionRequest,
    ) -> Result<ClosePositionResponse, AppError> {
        info!("{}", serde_json::to_string(close_request)?);
        let result = self
            .client
            .request::<ClosePositionRequest, ClosePositionResponse>(
                Method::POST,
                "positions/otc",
                session,
                Some(close_request),
                "1",
            )
            .await?;

        debug!("Position closed with reference: {}", result.deal_reference);
        Ok(result)
    }

    async fn get_working_orders(&self, session: &IgSession) -> Result<WorkingOrders, AppError> {
        info!("Getting all working orders");

        let result = self
            .client
            .request::<(), WorkingOrders>(Method::GET, "workingorders", session, None, "2")
            .await?;

        debug!("Retrieved {} working orders", result.working_orders.len());
        Ok(result)
    }

    async fn create_working_order(
        &self,
        session: &IgSession,
        order: &CreateWorkingOrderRequest,
    ) -> Result<CreateWorkingOrderResponse, AppError> {
        info!("Creating working order for: {}", order.epic);

        let result = self
            .client
            .request::<CreateWorkingOrderRequest, CreateWorkingOrderResponse>(
                Method::POST,
                "workingorders/otc",
                session,
                Some(order),
                "2",
            )
            .await?;

        debug!(
            "Working order created with reference: {}",
            result.deal_reference
        );
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::transport::http_client::IgHttpClientImpl;
    use crate::utils::rate_limiter::RateLimitType;
    use std::sync::Arc;

    #[test]
    fn test_get_and_set_config() {
        let config = Arc::new(Config::with_rate_limit_type(
            RateLimitType::NonTradingAccount,
            0.7,
        ));
        let client = Arc::new(IgHttpClientImpl::new(config.clone()));
        let mut service = OrderServiceImpl::new(config.clone(), client.clone());

        let cfg1 = service.get_config();
        assert!(Arc::ptr_eq(&cfg1, &config));

        let new_cfg = Arc::new(Config::default());
        service.set_config(new_cfg.clone());
        assert!(Arc::ptr_eq(&service.get_config(), &new_cfg));
    }
}
