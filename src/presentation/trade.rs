use crate::presentation::serialization::{option_string_empty_as_none, string_as_float_opt};
use lightstreamer_rs::subscription::ItemUpdate;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fmt;
use crate::application::models::order::{Direction, OrderType, TimeInForce, Status};


/// Main structure for trade data received from the IG Markets API
/// Contains information about trades, positions and working orders
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TradeData {
    /// Name of the item (usually the trade ID)
    pub item_name: String,
    /// Position of the item in the subscription
    pub item_pos: i32,
    /// All trade fields for this item
    pub fields: TradeFields,
    /// Fields that have changed in this update
    pub changed_fields: TradeFields,
    /// Whether this is a snapshot or an update
    pub is_snapshot: bool,
}

// Main fields
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TradeFields {
    #[serde(rename = "CONFIRMS")]
    #[serde(with = "option_string_empty_as_none")]
    #[serde(default)]
    pub confirms: Option<String>,

    #[serde(rename = "OPU")]
    #[serde(default)]
    pub opu: Option<OpenPositionUpdate>,

    #[serde(rename = "WOU")]
    #[serde(default)]
    pub wou: Option<WorkingOrderUpdate>,
}

// Structure for Open Position Update (OPU)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OpenPositionUpdate {
    #[serde(rename = "dealReference")]
    #[serde(with = "option_string_empty_as_none")]
    #[serde(default)]
    pub deal_reference: Option<String>,

    #[serde(rename = "dealId")]
    #[serde(with = "option_string_empty_as_none")]
    #[serde(default)]
    pub deal_id: Option<String>,

    #[serde(default)]
    pub direction: Option<Direction>,

    #[serde(default)]
    pub epic: Option<String>,

    #[serde(default)]
    pub status: Option<Status>,

    #[serde(rename = "dealStatus")]
    #[serde(default)]
    pub deal_status: Option<Status>,

    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    pub level: Option<f64>,

    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    pub size: Option<f64>,

    #[serde(with = "option_string_empty_as_none")]
    #[serde(default)]
    pub currency: Option<String>,

    #[serde(with = "option_string_empty_as_none")]
    #[serde(default)]
    pub timestamp: Option<String>, // We can convert to DateTime if needed

    #[serde(with = "option_string_empty_as_none")]
    #[serde(default)]
    pub channel: Option<String>,

    #[serde(with = "option_string_empty_as_none")]
    #[serde(default)]
    pub expiry: Option<String>,

    #[serde(rename = "dealIdOrigin")]
    #[serde(with = "option_string_empty_as_none")]
    #[serde(default)]
    pub deal_id_origin: Option<String>,
}

// Structure for Working Order Update (WOU)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkingOrderUpdate {
    #[serde(rename = "dealReference")]
    #[serde(with = "option_string_empty_as_none")]
    #[serde(default)]
    pub deal_reference: Option<String>,

    #[serde(rename = "dealId")]
    #[serde(with = "option_string_empty_as_none")]
    #[serde(default)]
    pub deal_id: Option<String>,

    #[serde(default)]
    pub direction: Option<Direction>,

    #[serde(with = "option_string_empty_as_none")]
    #[serde(default)]
    pub epic: Option<String>,

    #[serde(default)]
    pub status: Option<Status>,

    #[serde(rename = "dealStatus")]
    #[serde(default)]
    pub deal_status: Option<Status>,

    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    pub level: Option<f64>,

    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    pub size: Option<f64>,

    #[serde(with = "option_string_empty_as_none")]
    #[serde(default)]
    pub currency: Option<String>,

    #[serde(with = "option_string_empty_as_none")]
    #[serde(default)]
    pub timestamp: Option<String>, // We can convert to DateTime if needed

    #[serde(with = "option_string_empty_as_none")]
    #[serde(default)]
    pub channel: Option<String>,

    #[serde(with = "option_string_empty_as_none")]
    #[serde(default)]
    pub expiry: Option<String>,

    #[serde(rename = "stopDistance")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    pub stop_distance: Option<f64>,

    #[serde(rename = "limitDistance")]
    #[serde(with = "string_as_float_opt")]
    #[serde(default)]
    pub limit_distance: Option<f64>,

    #[serde(rename = "guaranteedStop")]
    #[serde(default)]
    pub guaranteed_stop: Option<bool>,

    #[serde(rename = "orderType")]
    #[serde(default)]
    pub order_type: Option<OrderType>,

    #[serde(rename = "timeInForce")]
    #[serde(default)]
    pub time_in_force: Option<TimeInForce>,

    #[serde(rename = "goodTillDate")]
    #[serde(with = "option_string_empty_as_none")]
    #[serde(default)]
    pub good_till_date: Option<String>, // We can convert to DateTime if needed
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
        let get_field = |key: &str| -> Option<String> { 
            let field = fields_map.get(key).cloned().flatten();
            match field { 
                Some(ref s) if s.is_empty() => None,
                _ => field,
            }
        };

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
