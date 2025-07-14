/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 13/5/25
******************************************************************************/
use crate::impl_json_display;
use serde::{Deserialize, Deserializer, Serialize};

//const DEFAULT_ORDER_SELL_SIZE: f64 = 0.0;
const DEFAULT_ORDER_BUY_SIZE: f64 = 10000.0;

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

impl_json_display!(Direction);

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
    #[serde(rename = "EXECUTE_AND_ELIMINATE")]
    ExecuteAndEliminate,
    /// Order must be filled completely immediately or cancelled
    #[serde(rename = "FILL_OR_KILL")]
    FillOrKill,
}

/// Model for creating a new order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub epic: String,
    pub expiry: String,
    pub direction: Direction,
    pub size: f64,
    #[serde(rename = "orderType")]
    pub order_type: OrderType,
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    #[serde(rename = "dealReference", skip_serializing_if = "Option::is_none")]
    pub deal_reference: Option<String>,
    #[serde(rename = "forceOpen")]
    pub force_open: bool,
    #[serde(rename = "guaranteedStop")]
    pub guaranteed_stop: bool,

    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<f64>,

    #[serde(rename = "limitDistance", skip_serializing_if = "Option::is_none")]
    pub limit_distance: Option<f64>,

    #[serde(rename = "limitLevel", skip_serializing_if = "Option::is_none")]
    pub limit_level: Option<f64>,

    #[serde(rename = "quoteId", skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,

    #[serde(rename = "stopDistance", skip_serializing_if = "Option::is_none")]
    pub stop_distance: Option<f64>,

    #[serde(rename = "stopLevel", skip_serializing_if = "Option::is_none")]
    pub stop_level: Option<f64>,

    #[serde(rename = "trailingStop", skip_serializing_if = "Option::is_none")]
    pub trailing_stop: Option<bool>,

    #[serde(
        rename = "trailingStopIncrement",
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_increment: Option<f64>,
}

impl CreateOrderRequest {
    /// Creates a new market order
    pub fn market(epic: String, direction: Direction, size: f64, currency_code: String) -> Self {
        Self {
            epic,
            direction,
            size,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::ExecuteAndEliminate,
            level: None,
            guaranteed_stop: false,
            stop_level: None,
            stop_distance: None,
            limit_level: None,
            limit_distance: None,
            expiry: "-".into(),
            deal_reference: None,
            force_open: false,
            currency_code,
            quote_id: None,
            trailing_stop: None,
            trailing_stop_increment: None,
        }
    }

    /// Creates a new limit order
    pub fn limit(
        epic: String,
        direction: Direction,
        size: f64,
        level: f64,
        currency_code: String,
    ) -> Self {
        Self {
            epic,
            direction,
            size,
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::GoodTillCancelled,
            level: Some(level),
            guaranteed_stop: false,
            stop_level: None,
            stop_distance: None,
            limit_level: None,
            limit_distance: None,
            expiry: "-".into(),
            deal_reference: None,
            force_open: true,
            currency_code,
            quote_id: None,
            trailing_stop: None,
            trailing_stop_increment: None,
        }
    }
    /*
    /// Creates a new instance of a market sell option with predefined parameters.
    ///
    /// This function sets up a sell option to the market for a given asset (`epic`)
    /// with the specified size. It configures the order with default values
    /// for attributes such as direction, order type, and time-in-force.
    ///
    /// # Parameters
    /// - `epic`: A `String` that represents the epic (unique identifier or code) of the instrument
    ///   being traded.
    /// - `size`: A `f64` value representing the size or quantity of the order.
    ///
    /// # Returns
    /// An instance of `Self` (the type implementing this function), containing the specified
    /// `epic` and `size`, along with default values for other parameters:
    ///
    /// - `direction`: Set to `Direction::Sell`.
    /// - `order_type`: Set to `OrderType::Limit`.
    /// - `time_in_force`: Set to `TimeInForce::FillOrKill`.
    /// - `level`: Set to `Some(0.1)`.
    /// - `guaranteed_stop`: Set to `None`.
    /// - `stop_level`: Set to `None`.
    /// - `stop_distance`: Set to `None`.
    /// - `limit_level`: Set to `None`.
    /// - `limit_distance`: Set to `None`.
    /// - `expiry`: Set to `None`.
    /// - `deal_reference`: Set to `None`.
    /// - `force_open`: Set to `Some(true)`.
    /// - `currency_code`: Set to `None`.
    ///
    /// Note that this function allows for minimal input (the instrument and size),
    /// while other fields are provided default values. If further customization is required,
    /// you can modify the returned instance as needed.

    //I dont fully understand this fn so i wont refacotr -FreeDuck
    #[allow(clippy::ptr_arg)]
    pub fn sell_option_to_market(
        epic: &String,
        size: &f64,
        expiry: &Option<String>,
        deal_reference: &Option<String>,
        currency_code: &Option<String>,
    ) -> Self {
        let rounded_size = (size * 100.0).floor() / 100.0;
        let currency_code = if let Some(code) = currency_code {
            Some(code.clone())
        } else {
            Some("EUR".to_string())
        };

        let deal_reference = if deal_reference.is_none() {
            &Some(nanoid::nanoid!(30, &nanoid::alphabet::SAFE))
        } else {
            deal_reference
        };

        Self {
            epic: epic.clone(),
            direction: Direction::Sell,
            size: rounded_size,
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::FillOrKill,
            level: Some(DEFAULT_ORDER_SELL_SIZE),
            guaranteed_stop: Some(false),
            stop_level: None,
            stop_distance: None,
            limit_level: None,
            limit_distance: None,
            expiry: expiry.clone(),
            deal_reference: deal_reference.clone(),
            force_open: Some(true),
            currency_code,
        }
    }*/

    /// Constructs and returns a new instance of the `Self` struct representing a sell option
    /// to the market with specific parameters for execution.
    ///
    /// # Parameters
    /// - `epic`: A reference to a string slice (`&String`) that specifies the EPIC
    ///   (Exchanged Product Information Code) of the instrument for which the sell order is created.
    /// - `size`: A reference to a floating-point number (`&f64`) that represents the size of the sell
    ///   order. The size is rounded to two decimal places.
    /// - `expiry`: An optional string (`Option<String>`) that indicates the expiry date or period for
    ///   the sell order. If `None`, no expiry date will be set for the order.
    /// - `deal_reference`: An optional string (`Option<String>`) that contains a reference or identifier
    ///   for the deal. Can be used for tracking purposes.
    /// - `currency_code`: An optional string (`Option<String>`) representing the currency code. Defaults
    ///   to `"EUR"` if not provided.
    /// - `force_open`: An optional boolean (`Option<bool>`) that specifies whether to force open the
    ///   position. When `Some(true)`, a new position is opened even if an existing position for the
    ///   same instrument and direction is available.
    ///
    /// # Returns
    /// - `Self`: A new instance populated with the provided parameters, including the following default
    ///   properties:
    ///   - `direction`: Set to `Direction::Sell` to designate the sell operation.
    ///   - `order_type`: Set to `OrderType::Limit` to signify the type of the order.
    ///   - `time_in_force`: Set to `TimeInForce::FillOrKill` indicating the order should be fully
    ///     executed or canceled.
    ///   - `level`: Set to a constant value `DEFAULT_ORDER_SELL_SIZE`.
    ///   - `guaranteed_stop`: Set to `Some(false)` indicating no guaranteed stop.
    ///   - Other optional levels/distance fields (`stop_level`, `stop_distance`, `limit_level`,
    ///     `limit_distance`): Set to `None` by default.
    ///
    /// # Notes
    /// - The `#[allow(clippy::ptr_arg)]` attribute suppresses the lint warning for using
    ///   `&String` instead of `&str`. Consider updating the function signature to use `&str`
    ///   instead of `&String` for better idiomatic Rust code.
    /// - The input `size` is automatically rounded down to two decimal places before being stored.
    ///
    /*#[allow(clippy::ptr_arg)]
        pub fn sell_option_to_market_w_force(
            epic: &String,
            size: &f64,
            expiry: &Option<String>,
            deal_reference: &Option<String>,
            currency_code: &Option<String>,
            force_open: Option<bool>, // Compensate position if it is already open
        ) -> Self {
            let rounded_size = (size * 100.0).floor() / 100.0;
            let currency_code = if let Some(code) = currency_code {
                Some(code.clone())
            } else {
                Some("EUR".to_string())
            };
            let deal_reference = if deal_reference.is_none() {
                &Some(nanoid::nanoid!(30, &nanoid::alphabet::SAFE))
            } else {
                deal_reference
            };
            Self {
                epic: epic.clone(),
                direction: Direction::Sell,
                size: rounded_size,
                order_type: OrderType::Limit,
                time_in_force: TimeInForce::FillOrKill,
                level: Some(DEFAULT_ORDER_SELL_SIZE),
                guaranteed_stop: Some(false),
                stop_level: None,
                stop_distance: None,
                limit_level: None,
                limit_distance: None,
                expiry: expiry.clone(),
                deal_reference: deal_reference.clone(),
                force_open,
                currency_code,
            }
        }*/
    /*
        /// Creates a new instance of an order to buy an option in the market with specified parameters.
        ///
        /// This method initializes an order with the following default values:
        /// - `direction` is set to `Buy`.
        /// - `order_type` is set to `Limit`.
        /// - `time_in_force` is set to `FillOrKill`.
        /// - `level` is set to `Some(10000.0)`.
        /// - `force_open` is set to `Some(true)`.
        ///   Other optional parameters, such as stop levels, distances, expiry, and currency code, are left as `None`.
        ///
        /// # Parameters
        /// - `epic` (`String`): The identifier for the market or instrument to trade.
        /// - `size` (`f64`): The size or quantity of the order to be executed.
        ///
        /// # Returns
        /// A new instance of `Self` that represents the configured buy option for the given market.
        ///
        /// # Note
        /// Ensure the `epic` and `size` values provided are valid and match required market conditions.
        #[allow(clippy::ptr_arg)]
            pub fn buy_option_to_market(
                epic: &String,
                size: &f64,
                expiry: &Option<String>,
                deal_reference: &Option<String>,
                currency_code: &Option<String>,
            ) -> Self {
                let rounded_size = (size * 100.0).floor() / 100.0;
                let currency_code = if let Some(code) = currency_code {
                    Some(code.clone())
                } else {
                    Some("EUR".to_string())
                };
                let deal_reference = if deal_reference.is_none() {
                    &Some(nanoid::nanoid!(30, &nanoid::alphabet::SAFE))
                } else {
                    deal_reference
                };
                Self {
                    epic: epic.clone(),
                    direction: Direction::Buy,
                    size: rounded_size,
                    order_type: OrderType::Limit,
                    time_in_force: TimeInForce::FillOrKill,
                    level: Some(DEFAULT_ORDER_BUY_SIZE),
                    guaranteed_stop: Some(false),
                    stop_level: None,
                    stop_distance: None,
                    limit_level: None,
                    limit_distance: None,
                    expiry: expiry.clone(),
                    deal_reference: deal_reference.clone(),
                    force_open: Some(true),
                    currency_code: currency_code.clone(),
                }
            }*/
        /*
            /// Constructs a new instance of an order to buy an option in the market with optional force_open behavior.
            ///
            /// # Parameters
            ///
            /// * `epic` - A string slice representing the unique identifier of the instrument to be traded.
            /// * `size` - A reference to a 64-bit floating-point number that represents the size of the order.
            /// * `expiry` - An optional string reference representing the expiry date of the option.
            /// * `deal_reference` - An optional string reference for the deal reference identifier.
            /// * `currency_code` - An optional string reference representing the currency in which the order is denominated.
            ///   Defaults to "EUR" if not provided.
            /// * `force_open` - An optional boolean indicating whether to force open a new position regardless of existing positions.
            ///
            /// # Returns
            ///
            /// Returns a new instance of `Self`, representing the constructed order with the provided parameters.
            ///
            /// # Behavior
            ///
            /// * The size of the order will be rounded down to two decimal places for precision.
            /// * If a `currency_code` is not provided, the default currency code "EUR" is used.
            /// * Other parameters are directly mapped into the returned instance.
            ///
            /// # Notes
            ///
            /// * This function assumes that other order-related fields such as `level`, `stop_level`, `stop_distance`,
            ///   etc., are set to their defaults or require specific business logic, such as
            ///   `DEFAULT_ORDER_BUY_SIZE` for the initial buy size.
            /// * The Clippy lint for `clippy::ptr_arg` is explicitly allowed for the `epic` parameter, where
            ///   using a reference to `String` is intentional.
            ///
            #[allow(clippy::ptr_arg)]
            pub fn buy_option_to_market_w_force(
                epic: &String,
                size: &f64,
                expiry: &Option<String>,
                deal_reference: &Option<String>,
                currency_code: &Option<String>,
                force_open: Option<bool>,
            ) -> Self {
                let rounded_size = (size * 100.0).floor() / 100.0;
                let currency_code = if let Some(code) = currency_code {
                    Some(code.clone())
                } else {
                    Some("EUR".to_string())
                };
                let deal_reference = if deal_reference.is_none() {
                    &Some(nanoid::nanoid!(30, &nanoid::alphabet::SAFE))
                } else {
                    deal_reference
                };
                Self {
                    epic: epic.clone(),
                    direction: Direction::Buy,
                    size: rounded_size,
                    order_type: OrderType::Limit,
                    time_in_force: TimeInForce::FillOrKill,
                    level: Some(DEFAULT_ORDER_BUY_SIZE),
                    guaranteed_stop: Some(false),
                    stop_level: None,
                    stop_distance: None,
                    limit_level: None,
                    limit_distance: None,
                    expiry: expiry.clone(),
                    deal_reference: deal_reference.clone(),
                    force_open,
                    currency_code: currency_code.clone(),
                }
            }*/

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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderResponse {
    /// Client-generated reference for the deal
    #[serde(rename = "dealReference")]
    pub deal_reference: String,
}

/// Helper function to deserialize a nullable status field
/// When the status is null in the JSON, we default to Rejected status
fn deserialize_nullable_status<'de, D>(deserializer: D) -> Result<Status, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or(Status::Rejected))
}

/// Details of a confirmed order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderConfirmation {
    /// Date and time of the confirmation
    pub date: String,
    /// Status of the order (accepted, rejected, etc.)
    /// This can be null in some responses (e.g., when market is closed)
    #[serde(deserialize_with = "deserialize_nullable_status")]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClosePositionRequest {
    #[serde(rename = "dealId", skip_serializing_if = "Option::is_none")]
    pub deal_id: Option<String>,
    pub epic: String,
    pub expiry: String,
    pub direction: Direction,
    pub size: f64,
    #[serde(rename = "orderType")]
    pub order_type: OrderType,
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    #[serde(rename = "forceOpen")]
    pub force_open: bool,
    #[serde(rename = "guaranteedStop")]
    pub guaranteed_stop: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<f64>,
    #[serde(rename = "quote_id", skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,
}

impl ClosePositionRequest {
    /// Creates a request to close a position at market price
    pub fn market(
        deal_id: String,
        direction: Direction,
        size: f64,
        epic: String,
        currency_code: String,
    ) -> Self {
        Self {
            deal_id: Some(deal_id),
            direction,
            size,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::ExecuteAndEliminate,
            level: None,
            expiry: "-".into(),
            epic,
            quote_id: None,
            currency_code,
            force_open: false,
            guaranteed_stop: false,
        }
    }

    /// Creates a request to close a position at a specific price level
    ///
    /// This is useful for instruments that don't support market orders
    pub fn limit(
        deal_id: String,
        direction: Direction,
        size: f64,
        level: f64,
        epic: String,
        currency_code: String,
    ) -> Self {
        Self {
            deal_id: Some(deal_id),
            direction,
            size,
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::FillOrKill,
            level: Some(level),
            expiry: "-".into(),
            epic,
            quote_id: None,
            currency_code,
            force_open: false,
            guaranteed_stop: false,
        }
    }
    /*
    /// Creates a request to close an option position by deal ID using a limit order with predefined price levels
    ///
    /// This is specifically designed for options trading where market orders are not supported
    /// and a limit order with a predefined price level is required based on the direction.
    ///
    /// # Arguments
    /// * `deal_id` - The ID of the deal to close
    /// * `direction` - The direction of the closing order (opposite of the position direction)
    /// * `size` - The size of the position to close
    pub fn close_option_to_market_by_id(deal_id: String, direction: Direction, size: f64) -> Self {
        let level = match direction {
            Direction::Buy => Some(DEFAULT_ORDER_BUY_SIZE),
            Direction::Sell => Some(DEFAULT_ORDER_SELL_SIZE),
        };
        Self {
            deal_id: Some(deal_id),
            direction,
            size,
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::FillOrKill,
            level,
            expiry: None,
            epic: None,
            quote_id: None,
        }
    }*/

    /*/// Creates a request to close an option position by epic identifier using a limit order with predefined price levels
    ///
    /// This is specifically designed for options trading where market orders are not supported
    /// and a limit order with a predefined price level is required based on the direction.
    /// This method is used when the deal ID is not available but the epic and expiry are known.
    ///
    /// # Arguments
    /// * `epic` - The epic identifier of the instrument
    /// * `expiry` - The expiry date of the option
    /// * `direction` - The direction of the closing order (opposite of the position direction)
    /// * `size` - The size of the position to close
    pub fn close_option_to_market_by_epic(
        epic: String,
        expiry: String,
        direction: Direction,
        size: f64,
    ) -> Self {
        let level = match direction {
            Direction::Buy => Some(DEFAULT_ORDER_BUY_SIZE),
            Direction::Sell => Some(DEFAULT_ORDER_SELL_SIZE),
        };
        Self {
            deal_id: None,
            direction,
            size,
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::FillOrKill,
            level,
            expiry: Some(expiry),
            epic: Some(epic),
            quote_id: None,
        }
    }*/
}

/// Response to closing a position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClosePositionResponse {
    /// Client-generated reference for the closing deal
    #[serde(rename = "dealReference")]
    pub deal_reference: String,
}

/// Response to updating a position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePositionResponse {
    /// Client-generated reference for the update deal
    #[serde(rename = "dealReference")]
    pub deal_reference: String,
}

/// Model for creating a new working order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWorkingOrderRequest {
    /// Instrument EPIC identifier
    pub epic: String,
    /// Order direction (buy or sell)
    pub direction: Direction,
    /// Order size/quantity
    pub size: f64,
    /// Price level for the order
    pub level: f64,
    /// Type of working order (LIMIT or STOP)
    #[serde(rename = "type")]
    pub order_type: OrderType,
    /// Order duration (how long the order remains valid)
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
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
    /// Expiry date for GTD orders
    #[serde(rename = "goodTillDate", skip_serializing_if = "Option::is_none")]
    pub good_till_date: Option<String>,
    /// Client-generated reference for the deal
    #[serde(rename = "dealReference", skip_serializing_if = "Option::is_none")]
    pub deal_reference: Option<String>,
    /// Currency code for the order (e.g., "USD", "EUR")
    #[serde(rename = "currencyCode", skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
}

impl CreateWorkingOrderRequest {
    /// Creates a new limit working order
    pub fn limit(epic: String, direction: Direction, size: f64, level: f64) -> Self {
        Self {
            epic,
            direction,
            size,
            level,
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::GoodTillCancelled,
            guaranteed_stop: None,
            stop_level: None,
            stop_distance: None,
            limit_level: None,
            limit_distance: None,
            good_till_date: None,
            deal_reference: None,
            currency_code: None,
        }
    }

    /// Creates a new stop working order
    pub fn stop(epic: String, direction: Direction, size: f64, level: f64) -> Self {
        Self {
            epic,
            direction,
            size,
            level,
            order_type: OrderType::Stop,
            time_in_force: TimeInForce::GoodTillCancelled,
            guaranteed_stop: None,
            stop_level: None,
            stop_distance: None,
            limit_level: None,
            limit_distance: None,
            good_till_date: None,
            deal_reference: None,
            currency_code: None,
        }
    }

    /// Adds a stop loss to the working order
    pub fn with_stop_loss(mut self, stop_level: f64) -> Self {
        self.stop_level = Some(stop_level);
        self
    }

    /// Adds a take profit to the working order
    pub fn with_take_profit(mut self, limit_level: f64) -> Self {
        self.limit_level = Some(limit_level);
        self
    }

    /// Adds a reference to the working order
    pub fn with_reference(mut self, reference: String) -> Self {
        self.deal_reference = Some(reference);
        self
    }

    /// Sets the order to expire at a specific date
    pub fn expires_at(mut self, date: String) -> Self {
        self.time_in_force = TimeInForce::GoodTillDate;
        self.good_till_date = Some(date);
        self
    }
}

/// Response to working order creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWorkingOrderResponse {
    /// Client-generated reference for the deal
    #[serde(rename = "dealReference")]
    pub deal_reference: String,
}

impl_json_display!(
    CreateOrderRequest,
    CreateOrderResponse,
    OrderConfirmation,
    UpdatePositionRequest,
    ClosePositionRequest,
    ClosePositionResponse,
    UpdatePositionResponse,
    CreateWorkingOrderRequest,
    CreateWorkingOrderResponse
);
