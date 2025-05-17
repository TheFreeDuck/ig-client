use crate::presentation::serialization::{option_string_empty_as_none, string_as_float_opt};
use lightstreamer_rs::subscription::ItemUpdate;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum Direction {
    #[serde(rename = "BUY")]
    #[default]
    Buy,
    #[serde(rename = "SELL")]
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum Status {
    #[serde(rename = "AMENDED")]
    Amended,
    #[serde(rename = "DELETED")]
    Deleted,
    #[serde(rename = "FULLY_CLOSED")]
    FullyClosed,
    #[serde(rename = "OPENED")]
    Opened,
    #[serde(rename = "PARTIALLY_CLOSED")]
    PartiallyClosed,
    #[serde(rename = "CLOSED")]
    Closed,
    #[serde(rename = "OPEN")]
    #[default]
    Open,
    #[serde(rename = "UPDATED")]
    Updated,
    #[serde(rename = "ACCEPTED")]
    Accepted,
    #[serde(rename = "REJECTED")]
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum OrderType {
    #[serde(rename = "LIMIT")]
    #[default]
    Limit,
    #[serde(rename = "STOP")]
    Stop,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum TimeInForce {
    #[serde(rename = "GOOD_TILL_CANCELLED")]
    #[default]
    GoodTillCancelled,
    #[serde(rename = "GOOD_TILL_DATE")]
    GoodTillDate,
}

/// Main structure for trade data received from the IG Markets API
/// Contains information about trades, positions and working orders
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TradeData {
    /// Name of the item (usually the trade ID)
    item_name: String,
    /// Position of the item in the subscription
    item_pos: i32,
    /// All trade fields for this item
    fields: TradeFields,
    /// Fields that have changed in this update
    changed_fields: TradeFields,
    /// Whether this is a snapshot or an update
    is_snapshot: bool,
}

// Main fields
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TradeFields {
    #[serde(rename = "CONFIRMS")]
    #[serde(with = "option_string_empty_as_none")]
    #[serde(default)]
    confirms: Option<String>,

    #[serde(rename = "OPU")]
    #[serde(default)]
    opu: Option<OpenPositionUpdate>,

    #[serde(rename = "WOU")]
    #[serde(default)]
    wou: Option<WorkingOrderUpdate>,
}

// Structure for Open Position Update (OPU)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OpenPositionUpdate {
    #[serde(default)]
    deal_reference: Option<String>,

    #[serde(default)]
    deal_id: Option<String>,

    #[serde(default)]
    direction: Option<Direction>,

    #[serde(default)]
    epic: Option<String>,

    #[serde(default)]
    status: Option<Status>,

    #[serde(default)]
    deal_status: Option<Status>,

    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    level: Option<f64>,

    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    size: Option<f64>,

    #[serde(default)]
    currency: Option<String>,

    #[serde(default)]
    timestamp: Option<String>, // We can convert to DateTime if needed

    #[serde(default)]
    channel: Option<String>,

    #[serde(default)]
    expiry: Option<String>,

    // Fields specific to OPU
    #[serde(default)]
    deal_id_origin: Option<String>,
}

// Structure for Working Order Update (WOU)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkingOrderUpdate {
    #[serde(default)]
    deal_reference: Option<String>,

    #[serde(default)]
    deal_id: Option<String>,

    #[serde(default)]
    direction: Option<Direction>,

    #[serde(default)]
    epic: Option<String>,

    #[serde(default)]
    status: Option<Status>,

    #[serde(default)]
    deal_status: Option<Status>,

    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    level: Option<f64>,

    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    size: Option<f64>,

    #[serde(default)]
    currency: Option<String>,

    #[serde(default)]
    timestamp: Option<String>, // We can convert to DateTime if needed

    #[serde(default)]
    channel: Option<String>,

    #[serde(default)]
    expiry: Option<String>,

    // Fields specific to WOU
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    stop_distance: Option<f64>,

    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    limit_distance: Option<f64>,

    #[serde(default)]
    guaranteed_stop: Option<bool>,

    #[serde(default)]
    order_type: Option<OrderType>,

    #[serde(default)]
    time_in_force: Option<TimeInForce>,

    #[serde(default)]
    good_till_date: Option<String>, // We can convert to DateTime if needed
}

impl TradeData {
    /// Converts a Lightstreamer ItemUpdate to a TradeData object
    ///
    /// # Arguments
    ///
    /// * `item_update` - The ItemUpdate from Lightstreamer containing trade data
    ///
    /// # Returns
    ///
    /// A Result containing either the parsed TradeData or an error message
    pub fn from_item_update(item_update: &ItemUpdate) -> Result<Self, String> {
        // Extract the item_name, defaulting to an empty string if None
        let item_name = item_update.item_name.clone().unwrap_or_default();

        // Convert item_pos from usize to i32
        let item_pos = item_update.item_pos as i32;

        // Extract is_snapshot
        let is_snapshot = item_update.is_snapshot;

        // Convert fields
        let fields = Self::create_trade_fields(&item_update.fields)?;

        // Convert changed_fields by first creating a HashMap<String, Option<String>>
        let mut changed_fields_map: HashMap<String, Option<String>> = HashMap::new();
        for (key, value) in &item_update.changed_fields {
            changed_fields_map.insert(key.clone(), Some(value.clone()));
        }
        let changed_fields = Self::create_trade_fields(&changed_fields_map)?;

        Ok(TradeData {
            item_name,
            item_pos,
            fields,
            changed_fields,
            is_snapshot,
        })
    }

    // Helper method to create TradeFields from a HashMap
    fn create_trade_fields(
        fields_map: &HashMap<String, Option<String>>,
    ) -> Result<TradeFields, String> {
        // Helper function to safely get a field value
        let get_field = |key: &str| -> Option<String> { fields_map.get(key).cloned().flatten() };

        // Parse CONFIRMS
        let confirms = get_field("CONFIRMS");

        // Parse OPU
        let opu_str = get_field("OPU");
        let opu = if let Some(opu_json) = opu_str {
            if !opu_json.is_empty() {
                match serde_json::from_str::<OpenPositionUpdate>(&opu_json) {
                    Ok(parsed_opu) => Some(parsed_opu),
                    Err(e) => return Err(format!("Failed to parse OPU JSON: {}", e)),
                }
            } else {
                None
            }
        } else {
            None
        };

        // Parse WOU
        let wou_str = get_field("WOU");
        let wou = if let Some(wou_json) = wou_str {
            if !wou_json.is_empty() {
                match serde_json::from_str::<WorkingOrderUpdate>(&wou_json) {
                    Ok(parsed_wou) => Some(parsed_wou),
                    Err(e) => return Err(format!("Failed to parse WOU JSON: {}", e)),
                }
            } else {
                None
            }
        } else {
            None
        };

        Ok(TradeFields { confirms, opu, wou })
    }
}

impl fmt::Display for TradeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string(self).map_err(|_| fmt::Error)?;
        write!(f, "{}", json)
    }
}

impl From<&ItemUpdate> for TradeData {
    fn from(item_update: &ItemUpdate) -> Self {
        Self::from_item_update(item_update).unwrap_or_default()
    }
}
