/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 12/5/25
******************************************************************************/
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Raw JSON coming from IG’s transactions endpoint
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RawTransaction {
    #[serde(rename = "date")]
    pub(crate) date: String,

    #[serde(rename = "dateUtc")]
    pub(crate) date_utc: String,

    #[serde(rename = "openDateUtc")]
    pub(crate) open_date_utc: String,

    #[serde(rename = "instrumentName")]
    pub(crate) instrument_name: String,

    #[serde(rename = "period")]
    pub(crate) period: String,

    #[serde(rename = "profitAndLoss")]
    pub(crate) pnl_raw: String,

    #[serde(rename = "transactionType")]
    pub(crate) transaction_type: String,

    pub(crate) reference: String,

    #[serde(rename = "openLevel")]
    pub(crate) open_level: String,

    #[serde(rename = "closeLevel")]
    pub(crate) close_level: String,

    #[serde(rename = "size")]
    pub(crate) size: String,

    #[serde(rename = "currency")]
    pub(crate) currency: String,

    #[serde(rename = "cashTransaction")]
    pub(crate) cash_transaction: bool,
}

impl fmt::Display for RawTransaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = serde_json::to_string(self).map_err(|_| fmt::Error)?;
        write!(f, "{}", s)
    }
}

/// Represents a processed transaction from IG Markets with parsed fields
#[derive(Debug)]
pub struct Transaction {
    /// Date and time when the transaction was executed
    pub(crate) deal_date: DateTime<Utc>,
    /// Underlying asset or instrument (e.g., "GOLD", "US500")
    pub(crate) underlying: Option<String>,
    /// Strike price for options
    pub(crate) strike: Option<f64>,
    /// Type of option ("CALL" or "PUT")
    pub(crate) option_type: Option<String>,
    /// Expiration date for options
    pub(crate) expiry: Option<NaiveDate>,
    /// Type of transaction (e.g., "DEAL", "WITH")
    pub(crate) transaction_type: String,
    /// Profit and loss in EUR
    pub(crate) pnl_eur: f64,
    /// Unique reference for the transaction
    pub(crate) reference: String,
    /// Whether this transaction is a fee
    pub(crate) is_fee: bool,
    /// Original JSON string of the transaction
    pub(crate) raw_json: String,
}
