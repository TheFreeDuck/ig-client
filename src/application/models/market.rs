/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 13/5/25
******************************************************************************/
use serde::{Deserialize, Serialize};

/// Instrument type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InstrumentType {
    /// Stocks and shares
    Shares,
    /// Foreign exchange currency pairs
    Currencies,
    /// Market indices
    Indices,
    /// Short-term binary bets
    SprintMarket,
    /// Raw materials and natural resources
    Commodities,
    /// Option contracts
    Options,
    /// Binary options
    #[serde(rename = "BINARY")]
    Binary,
    /// Unknown instrument type
    #[serde(other)]
    Unknown,
}

/// Model for a market instrument
#[derive(Debug, Clone, Deserialize)]
pub struct Instrument {
    /// Unique identifier for the instrument
    pub epic: String,
    /// Human-readable name of the instrument
    pub name: String,
    /// Type of the instrument
    #[serde(rename = "instrumentType")]
    pub instrument_type: InstrumentType,
    /// Expiry date of the instrument
    pub expiry: String,
    /// Size of one contract
    #[serde(rename = "contractSize")]
    pub contract_size: Option<f64>,
    /// Size of one lot
    #[serde(rename = "lotSize")]
    pub lot_size: Option<f64>,
    /// Upper price limit for the instrument
    #[serde(rename = "highLimitPrice")]
    pub high_limit_price: Option<f64>,
    /// Lower price limit for the instrument
    #[serde(rename = "lowLimitPrice")]
    pub low_limit_price: Option<f64>,
    /// Margin factor for the instrument
    #[serde(rename = "marginFactor")]
    pub margin_factor: Option<f64>,
    /// Unit for the margin factor
    #[serde(rename = "marginFactorUnit")]
    pub margin_factor_unit: Option<String>,
    /// Factor for price slippage
    #[serde(rename = "slippageFactor")]
    pub slippage_factor: Option<f64>,
    /// Premium for limited risk trades
    #[serde(rename = "limitedRiskPremium")]
    pub limited_risk_premium: Option<f64>,
    /// Code for news related to this instrument
    #[serde(rename = "newsCode")]
    pub news_code: Option<String>,
    /// Code for chart data related to this instrument
    #[serde(rename = "chartCode")]
    pub chart_code: Option<String>,
    /// Available currencies for trading this instrument
    pub currencies: Option<Vec<Currency>>,
}

/// Model for an instrument's currency
#[derive(Debug, Clone, Deserialize)]
pub struct Currency {
    /// Currency code (e.g., "USD", "EUR")
    pub code: String,
    /// Currency symbol (e.g., "$", "€")
    pub symbol: Option<String>,
    /// Base exchange rate for the currency
    #[serde(rename = "baseExchangeRate")]
    pub base_exchange_rate: Option<f64>,
    /// Current exchange rate
    #[serde(rename = "exchangeRate")]
    pub exchange_rate: Option<f64>,
    /// Whether this is the default currency for the instrument
    #[serde(rename = "isDefault")]
    pub is_default: Option<bool>,
}

/// Model for market data
#[derive(Debug, Clone, Deserialize)]
pub struct MarketDetails {
    /// Detailed information about the instrument
    pub instrument: Instrument,
    /// Current market snapshot with prices
    pub snapshot: MarketSnapshot,
}

/// Trading rules for a market
#[derive(Debug, Clone, Deserialize)]
pub struct DealingRules {
    /// Minimum deal size allowed
    #[serde(rename = "minDealSize")]
    pub min_deal_size: Option<f64>,
    /// Maximum deal size allowed
    #[serde(rename = "maxDealSize")]
    pub max_deal_size: Option<f64>,
    /// Minimum distance for controlled risk stop
    #[serde(rename = "minControlledRiskStopDistance")]
    pub min_controlled_risk_stop_distance: Option<f64>,
    /// Minimum distance for normal stop or limit orders
    #[serde(rename = "minNormalStopOrLimitDistance")]
    pub min_normal_stop_or_limit_distance: Option<f64>,
    /// Maximum distance for stop or limit orders
    #[serde(rename = "maxStopOrLimitDistance")]
    pub max_stop_or_limit_distance: Option<f64>,
    /// Market order preference setting
    #[serde(rename = "marketOrderPreference")]
    pub market_order_preference: String,
    /// Trailing stops preference setting
    #[serde(rename = "trailingStopsPreference")]
    pub trailing_stops_preference: String,
}

/// Market snapshot
#[derive(Debug, Clone, Deserialize)]
pub struct MarketSnapshot {
    /// Current status of the market (e.g., "OPEN", "CLOSED")
    #[serde(rename = "marketStatus")]
    pub market_status: String,
    /// Net change in price since previous close
    #[serde(rename = "netChange")]
    pub net_change: Option<f64>,
    /// Percentage change in price since previous close
    #[serde(rename = "percentageChange")]
    pub percentage_change: Option<f64>,
    /// Time of the last price update
    #[serde(rename = "updateTime")]
    pub update_time: Option<String>,
    /// Delay time in milliseconds for market data
    #[serde(rename = "delayTime")]
    pub delay_time: Option<i64>,
    /// Current bid price
    pub bid: Option<f64>,
    /// Current offer/ask price
    pub offer: Option<f64>,
    /// Highest price of the current trading session
    #[serde(rename = "high")]
    pub high: Option<f64>,
    /// Lowest price of the current trading session
    #[serde(rename = "low")]
    pub low: Option<f64>,
    /// Odds for binary markets
    #[serde(rename = "binaryOdds")]
    pub binary_odds: Option<f64>,
    /// Factor for decimal places in price display
    #[serde(rename = "decimalPlacesFactor")]
    pub decimal_places_factor: Option<i64>,
    /// Factor for scaling prices
    #[serde(rename = "scalingFactor")]
    pub scaling_factor: Option<i64>,
    /// Extra spread for controlled risk trades
    #[serde(rename = "controlledRiskExtraSpread")]
    pub controlled_risk_extra_spread: Option<f64>,
}

/// Model for market search results
#[derive(Debug, Clone, Deserialize)]
pub struct MarketSearchResult {
    /// List of markets matching the search criteria
    pub markets: Vec<MarketData>,
}

/// Basic market data
#[derive(Debug, Clone, Deserialize)]
pub struct MarketData {
    /// Unique identifier for the market
    pub epic: String,
    /// Human-readable name of the instrument
    #[serde(rename = "instrumentName")]
    pub instrument_name: String,
    /// Type of the instrument
    #[serde(rename = "instrumentType")]
    pub instrument_type: InstrumentType,
    /// Expiry date of the instrument
    pub expiry: String,
    /// Upper price limit for the market
    #[serde(rename = "highLimitPrice")]
    pub high_limit_price: Option<f64>,
    /// Lower price limit for the market
    #[serde(rename = "lowLimitPrice")]
    pub low_limit_price: Option<f64>,
    /// Current status of the market
    #[serde(rename = "marketStatus")]
    pub market_status: String,
    /// Net change in price since previous close
    #[serde(rename = "netChange")]
    pub net_change: Option<f64>,
    /// Percentage change in price since previous close
    #[serde(rename = "percentageChange")]
    pub percentage_change: Option<f64>,
    /// Time of the last price update
    #[serde(rename = "updateTime")]
    pub update_time: Option<String>,
    /// Current bid price
    pub bid: Option<f64>,
    /// Current offer/ask price
    pub offer: Option<f64>,
}

/// Model for historical prices
#[derive(Debug, Clone, Deserialize)]
pub struct HistoricalPricesResponse {
    /// List of historical price points
    pub prices: Vec<HistoricalPrice>,
    /// Type of the instrument
    #[serde(rename = "instrumentType")]
    pub instrument_type: InstrumentType,
    /// API usage allowance information
    #[serde(rename = "allowance")]
    pub allowance: PriceAllowance,
}

/// Historical price data point
#[derive(Debug, Clone, Deserialize)]
pub struct HistoricalPrice {
    /// Timestamp of the price data point
    #[serde(rename = "snapshotTime")]
    pub snapshot_time: String,
    /// Opening price for the period
    #[serde(rename = "openPrice")]
    pub open_price: PricePoint,
    /// Highest price for the period
    #[serde(rename = "highPrice")]
    pub high_price: PricePoint,
    /// Lowest price for the period
    #[serde(rename = "lowPrice")]
    pub low_price: PricePoint,
    /// Closing price for the period
    #[serde(rename = "closePrice")]
    pub close_price: PricePoint,
    /// Volume traded during the period
    #[serde(rename = "lastTradedVolume")]
    pub last_traded_volume: Option<i64>,
}

/// Price point with bid, ask and last traded prices
#[derive(Debug, Clone, Deserialize)]
pub struct PricePoint {
    /// Bid price at this point
    pub bid: Option<f64>,
    /// Ask/offer price at this point
    pub ask: Option<f64>,
    /// Last traded price at this point
    #[serde(rename = "lastTraded")]
    pub last_traded: Option<f64>,
}

/// Information about API usage allowance for price data
#[derive(Debug, Clone, Deserialize)]
pub struct PriceAllowance {
    /// Remaining API calls allowed in the current period
    #[serde(rename = "remainingAllowance")]
    pub remaining_allowance: i64,
    /// Total API calls allowed per period
    #[serde(rename = "totalAllowance")]
    pub total_allowance: i64,
    /// Time until the allowance resets
    #[serde(rename = "allowanceExpiry")]
    pub allowance_expiry: i64,
}
