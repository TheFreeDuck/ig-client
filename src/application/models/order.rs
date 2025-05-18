/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 13/5/25
******************************************************************************/
use serde::{Deserialize, Serialize};

/// Order direction (buy or sell)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum Direction {
    /// Buy direction (long position)
    #[default]
    Buy,
    /// Sell direction (short position)
    Sell,
}

/// Order type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderType {
    /// Limit order - executed when price reaches specified level
    #[default]
    Limit,
    /// Market order - executed immediately at current market price
    Market,
    /// Quote order - executed at quoted price
    Quote,
    /// Stop order - becomes market order when price reaches specified level
    Stop,
    /// Stop limit order - becomes limit order when price reaches specified level
    StopLimit,
}

/// Represents the status of an order or transaction in the system.
///
/// This enum covers various states an order can be in throughout its lifecycle,
/// from creation to completion or cancellation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum Status {
    /// Order has been amended or modified after initial creation
    Amended,
    /// Order has been deleted from the system
    Deleted,
    /// Order has been completely closed with all positions resolved
    #[serde(rename = "FULLY_CLOSED")]
    FullyClosed,
    /// Order has been opened and is active in the market
    Opened,
    /// Order has been partially closed with some positions still open
    #[serde(rename = "PARTIALLY_CLOSED")]
    PartiallyClosed,
    /// Order has been closed but may differ from FullyClosed in context
    Closed,
    /// Default state - order is open and active in the market
    #[default]
    Open,
    /// Order has been updated with new parameters
    Updated,
    /// Order has been accepted by the system or exchange
    Accepted,
    /// Order has been rejected by the system or exchange
    Rejected,
    /// Order is currently working (waiting to be filled)
    Working,
    /// Order has been filled (executed)
    Filled,
    /// Order has been cancelled
    Cancelled,
    /// Order has expired (time in force elapsed)
    Expired,
}

/// Order duration (time in force)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum TimeInForce {
    /// Order remains valid until cancelled by the client
    #[serde(rename = "GOOD_TILL_CANCELLED")]
    #[default]
    GoodTillCancelled,
    /// Order remains valid until a specified date
    #[serde(rename = "GOOD_TILL_DATE")]
    GoodTillDate,
    /// Order is executed immediately (partially or completely) or cancelled
    #[serde(rename = "IMMEDIATE_OR_CANCEL")]
    ImmediateOrCancel,
    /// Order must be filled completely immediately or cancelled
    #[serde(rename = "FILL_OR_KILL")]
    FillOrKill,
}

/// Model for creating a new order
#[derive(Debug, Clone, Serialize)]
pub struct CreateOrderRequest {
    /// Instrument EPIC identifier
    pub epic: String,
    /// Order direction (buy or sell)
    pub direction: Direction,
    /// Order size/quantity
    pub size: f64,
    /// Type of order (market, limit, etc.)
    #[serde(rename = "orderType")]
    pub order_type: OrderType,
    /// Order duration (how long the order remains valid)
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// Price level for limit orders
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<f64>,
    /// Whether to use a guaranteed stop
    #[serde(rename = "guaranteedStop", skip_serializing_if = "Option::is_none")]
    pub guaranteed_stop: Option<bool>,
    /// Price level for stop loss
    #[serde(rename = "stopLevel", skip_serializing_if = "Option::is_none")]
    pub stop_level: Option<f64>,
    /// Distance for stop loss
    #[serde(rename = "stopDistance", skip_serializing_if = "Option::is_none")]
    pub stop_distance: Option<f64>,
    /// Price level for take profit
    #[serde(rename = "limitLevel", skip_serializing_if = "Option::is_none")]
    pub limit_level: Option<f64>,
    /// Distance for take profit
    #[serde(rename = "limitDistance", skip_serializing_if = "Option::is_none")]
    pub limit_distance: Option<f64>,
    /// Expiry date for the order
    #[serde(rename = "expiry", skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    /// Client-generated reference for the deal
    #[serde(rename = "dealReference", skip_serializing_if = "Option::is_none")]
    pub deal_reference: Option<String>,
    /// Whether to force open a new position
    #[serde(rename = "forceOpen", skip_serializing_if = "Option::is_none")]
    pub force_open: Option<bool>,
}

impl CreateOrderRequest {
    /// Creates a new market order
    pub fn market(epic: String, direction: Direction, size: f64) -> Self {
        Self {
            epic,
            direction,
            size,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::FillOrKill,
            level: None,
            guaranteed_stop: None,
            stop_level: None,
            stop_distance: None,
            limit_level: None,
            limit_distance: None,
            expiry: None,
            deal_reference: None,
            force_open: Some(true),
        }
    }

    /// Creates a new limit order
    pub fn limit(epic: String, direction: Direction, size: f64, level: f64) -> Self {
        Self {
            epic,
            direction,
            size,
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::GoodTillCancelled,
            level: Some(level),
            guaranteed_stop: None,
            stop_level: None,
            stop_distance: None,
            limit_level: None,
            limit_distance: None,
            expiry: None,
            deal_reference: None,
            force_open: Some(true),
        }
    }

    /// Adds a stop loss to the order
    pub fn with_stop_loss(mut self, stop_level: f64) -> Self {
        self.stop_level = Some(stop_level);
        self
    }

    /// Adds a take profit to the order
    pub fn with_take_profit(mut self, limit_level: f64) -> Self {
        self.limit_level = Some(limit_level);
        self
    }

    /// Adds a reference to the order
    pub fn with_reference(mut self, reference: String) -> Self {
        self.deal_reference = Some(reference);
        self
    }
}

/// Response to order creation
#[derive(Debug, Clone, Deserialize)]
pub struct CreateOrderResponse {
    /// Client-generated reference for the deal
    #[serde(rename = "dealReference")]
    pub deal_reference: String,
}

/// Details of a confirmed order
#[derive(Debug, Clone, Deserialize)]
pub struct OrderConfirmation {
    /// Date and time of the confirmation
    pub date: String,
    /// Status of the order (accepted, rejected, etc.)
    pub status: Status,
    /// Reason for rejection if applicable
    pub reason: Option<String>,
    /// Unique identifier for the deal
    #[serde(rename = "dealId")]
    pub deal_id: Option<String>,
    /// Client-generated reference for the deal
    #[serde(rename = "dealReference")]
    pub deal_reference: String,
    /// Status of the deal
    #[serde(rename = "dealStatus")]
    pub deal_status: Option<String>,
    /// Instrument EPIC identifier
    pub epic: Option<String>,
    /// Expiry date for the order
    #[serde(rename = "expiry")]
    pub expiry: Option<String>,
    /// Whether a guaranteed stop was used
    #[serde(rename = "guaranteedStop")]
    pub guaranteed_stop: Option<bool>,
    /// Price level of the order
    #[serde(rename = "level")]
    pub level: Option<f64>,
    /// Distance for take profit
    #[serde(rename = "limitDistance")]
    pub limit_distance: Option<f64>,
    /// Price level for take profit
    #[serde(rename = "limitLevel")]
    pub limit_level: Option<f64>,
    /// Size/quantity of the order
    pub size: Option<f64>,
    /// Distance for stop loss
    #[serde(rename = "stopDistance")]
    pub stop_distance: Option<f64>,
    /// Price level for stop loss
    #[serde(rename = "stopLevel")]
    pub stop_level: Option<f64>,
    /// Whether a trailing stop was used
    #[serde(rename = "trailingStop")]
    pub trailing_stop: Option<bool>,
    /// Direction of the order (buy or sell)
    pub direction: Option<Direction>,
}

/// Model for updating an existing position
#[derive(Debug, Clone, Serialize)]
pub struct UpdatePositionRequest {
    /// New price level for stop loss
    #[serde(rename = "stopLevel", skip_serializing_if = "Option::is_none")]
    pub stop_level: Option<f64>,
    /// New price level for take profit
    #[serde(rename = "limitLevel", skip_serializing_if = "Option::is_none")]
    pub limit_level: Option<f64>,
    /// Whether to enable trailing stop
    #[serde(rename = "trailingStop", skip_serializing_if = "Option::is_none")]
    pub trailing_stop: Option<bool>,
    /// Distance for trailing stop
    #[serde(
        rename = "trailingStopDistance",
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_distance: Option<f64>,
}

/// Model for closing an existing position
#[derive(Debug, Clone, Serialize)]
pub struct ClosePositionRequest {
    /// Unique identifier for the position to close
    #[serde(rename = "dealId")]
    pub deal_id: String,
    /// Direction of the closing order (opposite to the position)
    pub direction: Direction,
    /// Size/quantity to close
    pub size: f64,
    /// Type of order to use for closing
    #[serde(rename = "orderType")]
    pub order_type: OrderType,
    /// Order duration for the closing order
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// Price level for limit close orders
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<f64>,
}

impl ClosePositionRequest {
    /// Creates a request to close a position at market price
    pub fn market(deal_id: String, direction: Direction, size: f64) -> Self {
        Self {
            deal_id,
            direction,
            size,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::FillOrKill,
            level: None,
        }
    }
}

/// Response to closing a position
#[derive(Debug, Clone, Deserialize)]
pub struct ClosePositionResponse {
    /// Client-generated reference for the closing deal
    #[serde(rename = "dealReference")]
    pub deal_reference: String,
}
