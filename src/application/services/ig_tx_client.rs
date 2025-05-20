use crate::application::models::transaction::{StoreTransaction};
use crate::config::Config;
use crate::error::AppError;
use crate::session::interface::IgSession;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use reqwest::{Client, StatusCode};
use tracing::debug;
use crate::application::models::account::AccountTransaction;

/// Interface for fetching transaction data from IG Markets
#[async_trait]
pub trait IgTxFetcher {
    /// Fetches transactions within a date range
    ///
    /// # Arguments
    /// * `sess` - Active IG session
    /// * `from` - Start date for the transaction range
    /// * `to` - End date for the transaction range
    ///
    /// # Returns
    /// * `Result<Vec<Transaction>, AppError>` - List of transactions or an error
    async fn fetch_range(
        &self,
        sess: &IgSession,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<StoreTransaction>, AppError>;
}

/// Client for fetching transaction data from IG Markets API
pub struct IgTxClient<'a> {
    /// Configuration for the IG API
    cfg: &'a Config,
    /// HTTP client for making requests
    http: Client,
}

impl<'a> IgTxClient<'a> {
    /// Creates a new IG transaction client
    ///
    /// # Arguments
    /// * `cfg` - Configuration for the IG API
    ///
    /// # Returns
    /// * A new IgTxClient instance
    pub fn new(cfg: &'a Config) -> Self {
        Self {
            cfg,
            http: Client::builder()
                .user_agent("ig-rs/0.1")
                .build()
                .expect("reqwest"),
        }
    }

    /// Constructs a REST API URL from the base URL and path
    ///
    /// # Arguments
    /// * `path` - API endpoint path
    ///
    /// # Returns
    /// * Complete URL string
    #[allow(dead_code)]
    fn rest_url(&self, path: &str) -> String {
        format!(
            "{}/{}",
            self.cfg.rest_api.base_url.trim_end_matches('/'),
            path
        )
    }
    
}

#[async_trait]
impl IgTxFetcher for IgTxClient<'_> {
    async fn fetch_range(
        &self,
        sess: &IgSession,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<StoreTransaction>, AppError> {
        let mut page = 1;
        let mut out = Vec::new();

        loop {
            let url = format!(
                "{}/history/transactions?from={}&to={}&pageNumber={}&pageSize=200",
                self.cfg.rest_api.base_url,
                from.format("%Y-%m-%dT%H:%M:%S"),
                to.format("%Y-%m-%dT%H:%M:%S"),
                page
            );
            debug!("🔗 Fetching IG txs from URL: {}", url);

            let resp = self
                .http
                .get(&url)
                .header("X-IG-API-KEY", &self.cfg.credentials.api_key)
                .header("CST", &sess.cst)
                .header("X-SECURITY-TOKEN", &sess.token)
                .header("Version", "2")
                .header("Accept", "application/json; charset=UTF-8")
                .send()
                .await?;

            if resp.status() != StatusCode::OK {
                return Err(AppError::Unexpected(resp.status()));
            }

            let json: serde_json::Value = resp.json().await?;
            let raws: Vec<AccountTransaction> =
                serde_json::from_value(json["transactions"].clone()).unwrap_or_default();

            if raws.is_empty() {
                break;
            }

            out.extend(raws.into_iter().map(|r| StoreTransaction::from(r)));

            let meta = &json["metadata"]["pageData"];
            let total_pages = meta["totalPages"].as_u64().unwrap_or(1);
            if page >= total_pages {
                break;
            }
            page += 1;
        }

        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;
    use super::*;
    use crate::config::Config;

    #[test]
    fn test_rest_url() {
        unsafe {
            std::env::set_var("IG_REST_BASE_URL", "https://api.example.com");
            std::env::set_var("IG_REST_TIMEOUT", "60");
            std::env::set_var("IG_WS_URL", "wss://ws.example.com");
            std::env::set_var("IG_WS_RECONNECT_INTERVAL", "10");
        }
        let config = Config::new();
        let client = IgTxClient::new(&config);
        assert_eq!(client.rest_url("path"), "https://api.example.com/path");
    }

    #[test]
    fn test_convert_basic() {
        let config = Config::new();
        let client = IgTxClient::new(&config);
        let raw = AccountTransaction {
            date: "".to_string(),
            date_utc: "2024-01-01T12:00:00".to_string(),
            open_date_utc: "".to_string(),
            instrument_name: "EURUSD".to_string(),
            period: "".to_string(),
            profit_and_loss: "E1000".to_string(),
            transaction_type: "DEAL".to_string(),
            reference: "REF123".to_string(),
            open_level: "".to_string(),
            close_level: "".to_string(),
            size: "".to_string(),
            currency: "".to_string(),
            cash_transaction: false,
        };
        let tx: StoreTransaction = StoreTransaction::from(raw.clone());
        assert_eq!(tx.transaction_type, raw.transaction_type);
        assert_eq!(tx.reference, raw.reference);
        assert_eq!(tx.pnl_eur, 1000.0);
        assert!(!tx.is_fee);
        assert_eq!(
            tx.deal_date.timestamp(),
            NaiveDateTime::parse_from_str(&raw.date_utc, "%Y-%m-%dT%H:%M:%S")
                .unwrap()
                .and_utc()
                .timestamp()
        );
        assert!(tx.raw_json.contains(&raw.reference));
    }

    #[test]
    fn test_convert_fee() {
        let config = Config::new();
        let client = IgTxClient::new(&config);
        let raw = AccountTransaction {
            date: "".to_string(),
            date_utc: "2024-01-02T00:00:00".to_string(),
            open_date_utc: "".to_string(),
            instrument_name: "".to_string(),
            period: "".to_string(),
            profit_and_loss: "E0.5".to_string(),
            transaction_type: "WITH".to_string(),
            reference: "FEE".to_string(),
            open_level: "".to_string(),
            close_level: "".to_string(),
            size: "".to_string(),
            currency: "".to_string(),
            cash_transaction: false,
        };
        let tx: StoreTransaction = StoreTransaction::from(raw.clone());
        assert_eq!(tx.pnl_eur, 0.5);
        assert!(tx.is_fee);
    }
}
